use crate::game::code_abstractions::{
    signals::RegisterSignal,
    node_operations::NodeReferences
};

use gdnative::prelude::*;
use gdnative::{api::RichTextLabel, api::NinePatchRect};

const DIALOGUE_SPEED: f64 = 0.05;

/// Enum that represents the posible states of the Dialogue Box.
///
/// Active -> Dialogue Box is printing text and is visible on the screen
/// Inactive -> The dialogue box has his visible property setted to `hidden`, so isn't appearing on the screen.
///
///  That's what this two Variants represents.
#[derive(PartialEq, Clone, Debug)]
pub enum DialogueBoxStatus {
    Active,
    Inactive
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
    text_to_print: String,
    current_char: i32,
    dialogue_text_label: Option<Ref<RichTextLabel>>,
    player_ref: Option<Ref<Node>>,
    dialogue_box_status: DialogueBoxStatus,
    times_pressed_interact: i32,
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
            text_to_print: Default::default(),
            current_char: 0,
            dialogue_text_label: None,
            player_ref: None,
            dialogue_box_status: DialogueBoxStatus::Inactive,
            times_pressed_interact: 0,
        } 
    }

    #[export]
    fn _ready(&mut self, owner: TRef<NinePatchRect>) {
        owner.set_process(true);
        
        // Retrieves a reference Ref<RichTextLabel> to the text label
        self.dialogue_text_label = unsafe { Some(self.get_node_reference_from_root(
            &owner, "Game/Player/Camera2D/DialogueBox/DialogueTextLabel"
        ).unwrap().assume_safe().cast::<RichTextLabel>().unwrap().assume_shared()) };

        // Retrieves a Ref<T> of the player character that notifies if the DialogueBox is running
        // This will be called to manage if player can move again or it's reading the label
        self.player_ref = self.get_node_reference_from_root(
            &owner, "Game/Player"
        );

        // Call the function that connect the signals of this struct with the player character
        self.connect_to_player(owner);
    }

    #[export]
    fn _process(&mut self, _owner: &NinePatchRect, _delta: f64) {
        
        // If the `printing` flag is true means that the `_print_dialogue` method was triggered by a signal binding
        if self.printing {
            self.timer += _delta; // Uses a timer as a "time handler", using delta to set it's value
            
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
                // let _player_ref = unsafe { self.player_ref.unwrap().assume_safe() };

                // Then, we should make visible the Pokémon Dialog Box
                _owner.set_visible(true);
                
                // Nested IF block. When code reaches this point basically we gonna check if there are still remaining characters to print.
                // If there still characters, we iterate to append to the label the next item
                if self.current_char < self.text_to_print.len() as i32 - 1 {                       
                    dialogue_text_label.set_bbcode(dialogue_text_label.bbcode() + 
                        GodotString::from(String::from(self.text_to_print.chars().nth(self.current_char as usize).expect("No more chars to print"))));                       
                    // Go next character next time
                    self.current_char += 1;

                // but if all characters are printed, wait for the player that with one more interaction button press,
                // closes the label
                } else {
                    // Gets an input singleton to point to the input events
                    let input: &Input = Input::godot_singleton();
                    
                    if Input::is_action_pressed(&input, "Interact") {
                        self.times_pressed_interact += 1;
                    
                        // Just checks if the player pressed the interact button **when all the characters are already printed**.
                        if self.times_pressed_interact >= 1 {
                            // Hides the `DialogueBox`
                            _owner.set_visible(false);
                            // Reset the internal values of the inside label to the first ones, let it ready for next interaction...
                            self.set_empty_dialogue_box(dialogue_text_label);
                            // Notifies all listeners the status of the DialogueBox
                            _owner.emit_signal("dialogue_box_inactive", &[Variant::from_godot_string(
                                &GodotString::from_str(""))]);
                            // Restart the interact when all char printed to zero for the next time
                            self.times_pressed_interact = 0;
                            // Saves the current status of the DialogueBox for data management
                            self.dialogue_box_status = DialogueBoxStatus::Inactive
                        }
                    }
                } 
            }
        }
    }
    
    /// Sets the text label inside the Pokémon dialogue box to the initial status
    fn set_empty_dialogue_box(&mut self, dialogue_text_label: TRef<RichTextLabel>) {
        self.current_char = 0;
        self.printing = false;
        self.timer = 0.0;
        dialogue_text_label.set_bbcode("");
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
    fn _print_dialogue(&mut self, _owner: &NinePatchRect, text: GodotString) {
        self.printing = true;
        self.text_to_print = text.to_string();
    }
}