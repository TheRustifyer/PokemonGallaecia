use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloGodot;

#[gdnative::methods]
impl HelloGodot {

    // The "constructor of the class"
    fn new(_owned: &Node) -> Self {
        HelloGodot
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Hello Godot from Rust! :)");
        // godot_print!("All is working fine. Keep on going.");
        // godot_print!("{:?}", _owner);
        godot_print!("{:?}", _owner.get_node("Label"));
        
        let label = unsafe { _owner.get_node_as::<Label>("Label") }.unwrap();
        label.set_text("Hello to the wonderful godot-rust people");

        godot_print!("Label.text {:?}", label.text());
        // godot_print!("{:?}", _owner.get_meta_list());
        self._edit_text(_owner);
    }

    fn _edit_text(&self, _owner: &Node) {
        godot_print!("I am the _edit_text");
    }
}