use crate::game::code_abstractions::{
    signals::RegisterSignal,
    node_operations::NodeReferences
};

use gdnative::api::AnimatedSprite;
use gdnative::prelude::*;
use gdnative::{api::RichTextLabel, api::NinePatchRect};

const DIALOGUE_SPEED: f64 = 0.05;

/// Enum that represents the posible states of the Dialogue Box.
///
/// Active -> Dialogue Box is printing text and is visible on the screen
/// Inactive -> The dialogue box has his visible property setted to `hidden`, so isn't appearing on the screen.
#[derive(PartialEq, Clone, Debug)]
pub enum DialogueBoxStatus {
    Active,
    Inactive
}
impl Default for DialogueBoxStatus {
    fn default() -> Self { DialogueBoxStatus::Inactive }
}

/// Dialogue Box it's build to manage all the text interactions in the game through the classical text box of Pokémon.
///
/// Showing text to the screen through his child (a RichTextLabel
#[derive(NativeClass)]
#[inherit(NinePatchRect)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct DialogueBox {
    printing: bool,
    timer: f64,

    dialogue_election: Option<DialogueElection<String>>,
    election_menu: Option<TRef<'static, NinePatchRect>>,
    menu_selector_arrow: Option<TRef<'static, Node2D>>,
    menu_selector_arrow_initial_position: Vector2,

    text_to_print: String,
    text_container: Vec<String>,
    current_text_container_position: i32,
    decision_selected: i32,
    number_of_decisions: i32,
    selection_enabled: bool,

    current_char: i32,
    current_line: i32,
    current_line_bound: i32,
    total_lines: i32,

    dialogue_text_label: Option<Ref<RichTextLabel>>,
    player_ref: Option<Ref<Node>>,
    dialogue_box_status: DialogueBoxStatus,
    times_pressed_interact: i32,
    // Gets an input singleton to point to the input events
    input: &'static Input
}

impl RegisterSignal<Self> for DialogueBox {
    fn register_signal(_builder: &ClassBuilder<Self>) {
        _builder.add_signal( Signal {
            name: "dialogue_box_active",
            args: &[],
        });
        _builder.add_signal( Signal {
            name: "dialogue_box_inactive",
            args: &[],
        });
    }
}

impl NodeReferences<NinePatchRect> for DialogueBox {
    fn get_node_reference_from_root(&mut self, owner: &NinePatchRect, path: &str) -> Option<Ref<Node>> {
        Some( unsafe { Node::get_tree(owner).unwrap()
            .assume_safe().root()
            .unwrap().assume_safe()
            .get_node(path)
            .unwrap()
            }
        )   
    }
}

#[gdnative::methods]
impl DialogueBox {

    fn new(_owner: &NinePatchRect) -> Self {
        Self {
            printing: false,
            timer: 0.0,

            dialogue_election: None,
            election_menu: None,
            menu_selector_arrow: None,
            menu_selector_arrow_initial_position: Vector2::new(0.0, 0.0),

            text_to_print: Default::default(),
            text_container: Default::default(),
            current_text_container_position: 0,
            decision_selected: 1,
            number_of_decisions: 0,
            selection_enabled: false,

            current_char: 0,
            current_line: 1,
            current_line_bound: 3,
            total_lines: 0,

            dialogue_text_label: None,
            player_ref: None,
            dialogue_box_status: DialogueBoxStatus::Inactive,
            times_pressed_interact: 0,
            input: Input::godot_singleton(),
        } 
    }

    #[export]
    fn _ready(&mut self, owner: TRef<NinePatchRect>) {
        owner.set_process(true);
        
        // Retrieves a reference Ref<RichTextLabel> to the text label
        self.dialogue_text_label = unsafe { Some(self.get_node_reference_from_root(
            &owner, "Game/Player/Camera2D/CanvasLayer/DialogueBox/DialogueTextLabel"
        ).unwrap().assume_safe().cast::<RichTextLabel>().unwrap().assume_shared()) };

        // Retrieves a Ref<T> of the player character that notifies if the DialogueBox is running
        // This will be called to manage if player can move again or it's reading the label
        self.player_ref = self.get_node_reference_from_root(&owner, "Game/Player");

        // Call the function that connect the signals of this struct with the player character
        self.connect_to_player(owner);

        // The **lines** of the text that will be printed
        self.total_lines = unsafe {self.dialogue_text_label.unwrap().assume_safe().get_line_count() as i32};

        // Decision menu and pointer
        self.election_menu = Some(unsafe { owner.get_node("ElectionMenu")
            .unwrap().assume_safe().cast::<NinePatchRect>().unwrap() });
        self.menu_selector_arrow = Some(unsafe { self.election_menu.unwrap().get_node("MenuSelector")
            .unwrap().assume_safe().cast::<Node2D>().unwrap() });
        self.menu_selector_arrow_initial_position = self.menu_selector_arrow.unwrap().position();
    }

    #[export]
    fn _process(&mut self, _owner: &NinePatchRect, _delta: f64) {

        // If the `printing` flag is true means that the `_print_dialogue` method was triggered by a signal binding
        if self.printing {
            self.timer += _delta; // Uses a timer as a "time handler", using delta to set it's value

            // Checks if there are elections in the current NPC dialogue
            if self.dialogue_election.as_ref().unwrap().get_number_of_decisions() > 0 && self.selection_enabled {
                self.enable_elections_on_interactive_dialogue(self.selection_enabled);
            }
            
            // Constant there acts algo as a barrier to trigger the print event
            if self.timer > DIALOGUE_SPEED {
                // Communicates to the potencial receivers that the dialogue box is currently visible on the screen
                _owner.emit_signal("dialogue_box_active", &[Variant::from_godot_string(
                    &GodotString::from_str("on_dialogue"))]);
                // Saves this status information on a property as a Variant 
                self.dialogue_box_status = DialogueBoxStatus::Active;
                // Resets the timers for the next frames, so when delta accumulates into it this if block executes again
                self.timer = 0.0;
                // Get the text to print from the struct attribute
                // Due to we need to access it's methods, we need to assume same to get a TRef<T>, where it's methods belong
                let dialogue_text_label = unsafe { self.dialogue_text_label.unwrap().assume_safe() };

                // Then, we should make visible the Pokémon Dialog Box
                _owner.set_visible(true);
                
                // Nested IF block. When code reaches this point basically we gonna check if there are still remaining characters to print.
                // If there still characters, we iterate to append to the label the next item
                if self.current_char < self.text_to_print.len() as i32 {

                    self.selection_enabled = false;
                    self.enable_elections_on_interactive_dialogue(self.selection_enabled);

                    if self.current_line < self.current_line_bound {
                        self.printer(&dialogue_text_label);
                    } else {
                        self.play_arrow_animation(_owner, &dialogue_text_label);
                    }

                } else if self.current_char == self.text_to_print.len() as i32 {
                    if self.number_of_decisions > 0 {
                        self.selection_enabled = true;
                    }    
                    self.play_arrow_animation(_owner, &dialogue_text_label);
                    if Input::is_action_pressed(&self.input, "Interact") {
                        self.current_char += 1;
                    }
                // but if all characters are printed, wait for the player that with one more interaction button press,
                // closes the label or chooses an option (depending on the NPC history)
                } else {

                    if let Some(dialogue_election) = self.dialogue_election.to_owned() {
            
                        if self.number_of_decisions >= 1 {

                            self.selection_enabled = false;
                            self.number_of_decisions -= 1;

                            // Sets the response based on what the player has choosed
                            // The Vec<String> with all the text maps the next characteristics:
                            // ! Index 0: Base text
                            // ! Index 1: Affirmative response / response that maps the selection nº 1
                            // ! Index 2: Negative response / response that maps the selection nº 2
                            // ! Index 3 and so forth...: Next response / response that maps the selection nº 3 and so forth...
                            self.current_text_container_position = self.decision_selected;
                                
                            self.text_to_print = dialogue_election.get_text_to_print()[self.current_text_container_position as usize].to_owned();

                            self.set_empty_dialogue_box(&dialogue_text_label);
                            self.printer(&dialogue_text_label);

                        } else {
                            self.play_arrow_animation(_owner, &dialogue_text_label);
                            self.finish_dialogue(_owner, &dialogue_text_label);
                        }
                    }
                }
            }
        }
    }

    fn enable_elections_on_interactive_dialogue(&mut self, visible: bool) {
        // Pop up election menu
        let election_menu = self.election_menu.unwrap();
        election_menu.set_visible(visible);
        let menu_selector_arrow = self.menu_selector_arrow.unwrap();
        let n_av_decisions = self.dialogue_election.as_ref().unwrap().get_availiable_decisions().len() as f32;

        if Input::is_action_just_pressed(&self.input, "Menu_Up") && self.current_char == self.text_to_print.len() as i32 {
            if self.decision_selected == 1 {
                self.decision_selected = n_av_decisions as i32;
                menu_selector_arrow.set_position(
                    Vector2::new(
                        menu_selector_arrow.position().x, menu_selector_arrow.position().y + (40.14 * (n_av_decisions - 1.0))
                    )
                )
            } else {
                self.decision_selected -= 1;
                menu_selector_arrow.set_position(
                    Vector2::new(
                        menu_selector_arrow.position().x, menu_selector_arrow.position().y - 40.14
                    )
                )
            }
        }

        if Input::is_action_just_pressed(&self.input, "Menu_Down") && self.current_char == self.text_to_print.len() as i32 {
            
            if self.decision_selected == n_av_decisions as i32{
                self.decision_selected = 1;
                menu_selector_arrow.set_position(
                    Vector2::new(
                        menu_selector_arrow.position().x, menu_selector_arrow.position().y - (40.14 * (n_av_decisions - 1.0))
                    )
                )
            } else {
                self.decision_selected += 1;
                menu_selector_arrow.set_position(
                    Vector2::new(
                        menu_selector_arrow.position().x, menu_selector_arrow.position().y + 40.14
                    )
                )
            } 
        }

    }

    fn play_arrow_animation(&mut self, owner: &NinePatchRect, dialogue_text_label: &TRef<RichTextLabel>) {

        let arrow_sprite = unsafe { owner.get_node("Cursor/Arrow")
            .unwrap().assume_safe().cast::<AnimatedSprite>().unwrap() };

        arrow_sprite.set_visible(true);
        arrow_sprite.play("", false);

        if Input::is_action_pressed(&self.input, "Interact") {
            dialogue_text_label.scroll_to_line(self.current_line as i64 - 1);
            self.current_line_bound += 1;
            arrow_sprite.stop();
            arrow_sprite.set_visible(false);
        }
    }

    fn printer(&mut self, dialogue_text_label: &TRef<RichTextLabel>) {

        if let Some(current_char_to_print) = self.text_to_print.chars().nth(self.current_char as usize) {

            if current_char_to_print == '\n' {
                self.current_line += 1;
            }

            dialogue_text_label.set_bbcode(dialogue_text_label.bbcode() + 
            GodotString::from(String::from(current_char_to_print)));
        }
                                
        // Go next character next time
        self.current_char += 1;
    }

    // Method for end the dialogue when there's no more text to print
    fn finish_dialogue(&mut self, owner: &NinePatchRect, dialogue_text_label: &TRef<RichTextLabel>) {
                
        // self.printing = false;

        if Input::is_action_pressed(&self.input, "Interact") {
            self.times_pressed_interact += 1;
            
            // Just checks if the player pressed the interact button **when all the characters are already printed**.
            if self.times_pressed_interact >= 1 {
                // Hides the `DialogueBox`
                owner.set_visible(false);
                // Reset the internal values of the inside label to the first ones, let it ready for next interaction...
                self.set_empty_dialogue_box(&dialogue_text_label);
                // Notifies all listeners the status of the DialogueBox
                owner.emit_signal("dialogue_box_inactive", &[Variant::from_godot_string(
                    &GodotString::from_str(""))]);
                // Restart the interact when all char printed to zero for the next time
                self.times_pressed_interact = 0;
                // Saves the current status of the DialogueBox for data management
                self.dialogue_box_status = DialogueBoxStatus::Inactive;
                // End of printing
                self.printing = false;
            }
        }
    }
    
    /// Sets the text label inside the Pokémon dialogue box to the initial status and all the variables that tracks it's status
    fn set_empty_dialogue_box(&mut self, dialogue_text_label: &TRef<RichTextLabel>) {
        self.current_char = 0;
        self.timer = 0.0;
        dialogue_text_label.set_bbcode("");
        self.current_line = 1;
        self.current_line_bound = 3;
        self.current_text_container_position = 0;
        self.decision_selected = 1;
        self.menu_selector_arrow.unwrap().set_position(self.menu_selector_arrow_initial_position);
    }

    #[export]
    /// Takes care about connect the DialogueBox signals to our PlayerCharacter
    fn connect_to_player(&self, _owner: TRef<NinePatchRect>) {
        let receiver = unsafe { self.player_ref.unwrap().assume_safe() };
        _owner.connect("dialogue_box_active", receiver, "handle_interaction",
            VariantArray::new_shared(), 0).unwrap();
        _owner.connect("dialogue_box_inactive", receiver, "handle_interaction",
            VariantArray::new_shared(), 0).unwrap();
    }

    #[export]
    /// Triggered by any connected signal through the game, sets the starting point to print content and provides
    /// the text that should be printed, passed by any availiable caller
    fn _print_dialogue(&mut self, _owner: &NinePatchRect, dialogue_elections: VariantArray) {

        // Converts back the data on the Variant Array to a User Defined Struct
        let dialogue_election_data = DialogueElection::<String>::get_data_from_variant(
            dialogue_elections
        );

        self.dialogue_election = Some(DialogueElection::new(
            dialogue_election_data.0,
            dialogue_election_data.1,
            dialogue_election_data.2,
        ));

        if let Some(dialogue_election) = &self.dialogue_election {
            self.printing = true;
            self.text_container = dialogue_election.get_text_to_print().to_owned();

            // At least always should one element inside the Vec of text_to_print
            self.text_to_print = self.text_container.get(0).unwrap().to_owned();

            // Counter
            self.number_of_decisions = dialogue_election.get_number_of_decisions();
        };
    }
}


#[derive(Debug, ToVariant, Clone)]
pub struct DialogueElection<T> {
    number_of_decisions: i32,
    availiable_decisions: Vec<T>,
    text_to_print: Vec<String>

    // ! Posible upgrade
    // responses: HashMap<T, String>
}

impl<T> DialogueElection<T> {

    pub fn new(
        number_of_decisions: i32, 
        availiable_decisions: Vec<T>, 
        text_to_print: Vec<String>
    ) -> Self { 
        Self { number_of_decisions, availiable_decisions, text_to_print } 
    }

    /// Converts the data encapsulated on a `VariantArray` argument on a new DialogueElection<T> instance
    ///
    /// Kind of a `static method`, 'cause just need this struct to destructure data into a custom data structure.
    pub fn get_data_from_variant(dialogue_elections: VariantArray) -> (i32, Vec<String>, Vec<String>) {

        let mut availiable_decisions: Vec<String> = Vec::new();
        for element in dialogue_elections.get(1).to_array().into_iter() {
            availiable_decisions.push(element.to_string())
        };

        let mut text_to_print: Vec<String> = Vec::new();
        for element in dialogue_elections.get(2).to_array().into_iter() {
            text_to_print.push(element.to_string())
        }

        let dialogue_election_data = (
            dialogue_elections.get(0).to_i64() as i32,
            availiable_decisions,
            text_to_print
        );

        dialogue_election_data
    }

    // Getters and Setters
    pub fn get_number_of_decisions(&self) -> i32 {
        self.number_of_decisions
    }

    pub fn set_number_of_decisions(&mut self, number_of_decisions: i32) {
        self.number_of_decisions = number_of_decisions;
    }

    pub fn get_availiable_decisions(&self) -> &Vec<T> {
        &self.availiable_decisions
    }

    pub fn set_availiable_decisions(&mut self, availiable_decisions: Vec<T>) {
        self.availiable_decisions = availiable_decisions;
    }

    pub fn get_text_to_print(&self) -> &Vec<String> {
        &self.text_to_print
    }

    pub fn set_text_to_print(&mut self, text_to_print: Vec<String>) {
        self.text_to_print = text_to_print;
    }
}

