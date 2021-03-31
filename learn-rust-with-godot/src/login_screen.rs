use gdnative::prelude::*;
use gdnative::api::{Label};

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
        let label = unsafe { _owner.get_node_as::<Label>("Label") }.unwrap();
        label.set_text("Modified from _ready")
    }

    #[export]
    fn set_label_text(&self, _owner: &Node, _label_identifier: String, text: String) {
        godot_print!("Set Label got called! again! :)");
        godot_print!("_owner on set_label: {:?}", _owner);
        godot_print!("_label_identifier: {:?}, text: {:?}", _label_identifier, text);
        let label = unsafe { _owner.get_node_as::<Label>(&_label_identifier) }.unwrap();
        // Here code panics, on line 28
        godot_print!("Label var value on set_label_text: {:?}", label);
        label.set_text(text)
    }
    
}