use gdnative::prelude::*;
use gdnative::api::{Label, LineEdit};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct LoginScreen;

#[gdnative::methods]
impl LoginScreen {

    // The "constructor of the class"
    fn new(_owned: &Node) -> Self {
        LoginScreen
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("_owner on _ready: {:?}", _owner);
        let label = unsafe { _owner.get_node_as::<Label>("VBoxContainer/Label") }.unwrap();
        label.set_text("Modified from _ready");
        self.set_label_text(_owner, "VBoxContainer/Label".to_string(), "HELLOOOOOOOO".to_string());
        self.print_player_input(_owner);
    }

    #[export]
    fn set_label_text(&self, _owner: &Node, _label_identifier: String, text: String) {
        godot_print!("Set Label got called! again! :)");
        godot_print!("_owner on set_label: {:?}", _owner);
        godot_print!("_label_identifier: {:?}, text: {:?}", _label_identifier, text);
        let label = unsafe { _owner.get_node_as::<Label>(&_label_identifier) }.unwrap();
        // Here code panics, on line 28
        godot_print!("Label var value on set_label_text: {:?}", label);
        label.set_text(text);
    }

    #[export]
    fn print_player_input(&self, _owner: &Node) {
        godot_print!("_owner on print_player_input: {:?}", _owner);
        let line_edit = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/PlayerInput") }.unwrap();
        godot_print!("Player input: {:?}", line_edit);
    }
    
}