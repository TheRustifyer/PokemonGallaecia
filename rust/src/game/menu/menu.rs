use gdnative::prelude::*;
use gdnative::api::NinePatchRect;

use crate::game::code_abstractions::signals::RegisterSignal;
use crate::game::code_abstractions::node_operations::NodeReferences;

use crate::utils::utils;


#[derive(PartialEq, Clone, Debug)]
pub enum MenuStatus {
    Open,
    Closed
}

#[derive(PartialEq, Clone, Debug)]
pub enum MenuInput {
    Up,
    Down
}
#[derive(PartialEq, Clone, Debug)]
pub enum MenuOptions {
    Pokedex,
    Pokemon,
    Pokegear,
    Bag, 
    Player,
    Options, 
    Save,
    Exit
}

#[derive(NativeClass)]
#[inherit(NinePatchRect)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct Menu {
    player_node_ref: Option<Ref<Node>>,
    menu_status: MenuStatus,

    current_menu_option: i32,
    menu_labels: VariantArray,
    cursor_pointer: Option<Ref<Node>>
}

impl RegisterSignal<Self> for Menu {
    fn register_signal(_builder: &ClassBuilder<Self>) {
        _builder.add_signal( Signal {
            name: "menu_opened",
            args: &[],
        });
        _builder.add_signal( Signal {
            name: "menu_closed",
            args: &[],
        });
    }
}

impl NodeReferences<NinePatchRect> for Menu {
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
impl Menu {
    fn new(_owner: &NinePatchRect) -> Self {
        Self {
            player_node_ref: None,
            menu_status: MenuStatus::Closed,
            current_menu_option: 0,
            menu_labels: VariantArray::new().into_shared(),
            cursor_pointer: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<NinePatchRect>) {
        owner.set_process(true);

        // Menu always spawns from a base hidden status
        owner.set_visible(false);

        // Retrieves and saves a reference as a VariantArray of the menu options
        self.retrieve_menu_options(&owner);

        // Retrieves a Ref<T> of the player character.
        // This will be called to manage if player can move again or it's inside the menu
        self.player_node_ref = self.get_node_reference_from_root(
            &owner, "Game/Player"
        );

        // Call the function that connect the signals of this struct with the player character
        self.connect_to_player(owner);

        // Sets a reference to the cursor sprite
        self.cursor_pointer = self.get_node_reference_from_root(&owner,"Game/Player/Camera2D/Menu/Arrow");

        self.cursor_pointer_update(&owner);
    }

    #[export]
    fn _process(&mut self, owner: &NinePatchRect, _delta: f64) {
        self.handle_menu_input_events(owner);
    }

    fn handle_menu_input_events(&mut self, owner: &NinePatchRect) {

        // Gets an input singleton to point to the input events
        let input: &Input = Input::godot_singleton();

        // This block of code matches a keyboard input event with the actions over the menu
        if Input::is_action_just_pressed(&input, "Menu") {
            if self.menu_status == MenuStatus::Closed {
                owner.emit_signal("menu_opened", &[Variant::from_str("menu_active")]);
                self.open_menu(owner)
            } else {
                owner.emit_signal("menu_closed", &[Variant::from_str("")]);
                self.close_menu(owner)
            }  
        } else if Input::is_action_just_pressed(&input, "Menu_Up") {
            match self.current_menu_option {
                x if x == 0 => self.current_menu_option = self.menu_labels.len() - 1,
                _ => self.current_menu_option -= 1
            }
            self.cursor_pointer_update(owner);
        } else if Input::is_action_just_pressed(&input, "Menu_Down") {
            match self.current_menu_option {
                x if x == self.menu_labels.len() - 1 => self.current_menu_option = 0,
                _ => self.current_menu_option += 1
            }
            self.cursor_pointer_update(owner);
        } else if Input::is_action_just_pressed(&input, "Interact") && self.menu_status == MenuStatus::Open 
                || Input::is_action_just_pressed(&input, "Enter") && self.menu_status == MenuStatus::Open {
            godot_print!("Option nº {}, {:?} has been selected!",
            self.current_menu_option + 1, self.menu_labels.get(self.current_menu_option));
            self.menu_option_to_scene(owner, self.current_menu_option)
        }
        else if Input::is_action_pressed(&input, "Exit") && self.menu_status == MenuStatus::Open{
            owner.emit_signal("menu_closed", &[]);
            self.close_menu(owner)
        }
    }

    fn open_menu(&mut self, owner: &NinePatchRect) {
        self.menu_status = MenuStatus::Open;
        owner.set_visible(true)
    }

    fn close_menu(&mut self, owner: &NinePatchRect) {
        self.menu_status = MenuStatus::Closed;
        owner.set_visible(false)
    }

    fn retrieve_menu_options(&mut self, owner: &NinePatchRect) {
        let menu_options_ref: TRef<Node> = unsafe { owner.get_node("MenuOptions").unwrap().assume_safe() };
        let menu_options: VariantArray = menu_options_ref.get_children();
        self.menu_labels = menu_options;
    }

    /// Method that updates the menu arrow position on the screen, in order to use it as a pointer that acts as
    /// graphical indicator or selector over the availiable menu options.
    fn cursor_pointer_update(&mut self, _owner: &NinePatchRect) {
        let cursor_pointer_sprite = unsafe { self.cursor_pointer.unwrap().assume_safe() 
            .cast::<Sprite>()
            .unwrap() };

        let desired_menu_option = unsafe { 
            self.menu_labels.get(self.current_menu_option) 
            .try_to_object::<Label>()
            .unwrap().assume_safe() };

        cursor_pointer_sprite.set_global_position(
            Vector2::new(
                cursor_pointer_sprite.global_position().x,
                desired_menu_option.global_position().y + 5.0
                )
            );
    }

    #[export]
    /// Takes care about connect the Menu custom signals to our PlayerCharacter
    fn connect_to_player(&self, _owner: TRef<NinePatchRect>) {
        let player_character = unsafe { self.player_node_ref.unwrap().assume_safe() };
        _owner.connect("menu_opened", player_character, "handle_interaction",
            VariantArray::new_shared(), 0).unwrap();
        _owner.connect("menu_closed", player_character, "handle_interaction",
            VariantArray::new_shared(), 0).unwrap();
    }

    /// Changes the scene to a designed one when a menu option is selected by the player
    #[export]
    fn menu_option_to_scene(&self, owner: &NinePatchRect, menu_option: i32) {
        match menu_option + 1 {
            1 => utils::change_scene(owner, "res://godot/Game/Pokedex.tscn".to_string()),
            _ => godot_print!("Menu option implemented yet!")
        }

        // let new_node = { owner.get_node(path).}
        // owner.get_scene_instance_load_placeholder()
    }

}

/*

Adds a child node. Nodes can have any number of children, but every child must 
have a unique name. Child nodes are automatically deleted when the parent node is 
deleted, so an entire scene can be removed by deleting its topmost node. 
If legible_unique_name is true, the child node will have an human-readable 
name based on the name of the node being instanced instead of its type. 
Note: If the child node already has a parent, the function will fail. 
Use [method remove_child](method remove_child) first to remove the node 
from its current parent. For example:

if child_node.get_parent():
    child_node.get_parent().remove_child(child_node)
add_child(child_node)
Note: If you want a child to be persisted to a PackedScene, you must set [member owner](member owner) in addition to calling [method add_child](method add_child). This is typically relevant for

*/