use gdnative::prelude::*;

use crate::game::code_abstractions::signals::RegisterSignal;

#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct Truck {
    times_signal_emitted: i32,
}

impl RegisterSignal<Self> for Truck {
    fn register_signal(_builder: &ClassBuilder<Self>) -> () {
        _builder.add_signal( Signal {
            name: "print_to_dialogue_box",
            args: &[],
        })
    }
}

#[gdnative::methods]
impl Truck {
    
    fn new(_owner: &Sprite) -> Self {
        Self {
            times_signal_emitted: 0,
        }
    }

    #[export]
    fn _ready(&self, _owner: TRef<Sprite>) {
        // Looking for interactions with the player
        self.connect_to_player(_owner);
        self.connect_signal_to_dialogue_box(&_owner)
    }

    #[export]
    fn emit_object_signal(&self, _owner: TRef<Sprite>) {
        _owner.emit_signal("print_to_dialogue_box", &[Variant::from_godot_string(
            &GodotString::from_str("Soy el camión de Pueblo de Teo!!"))]);
    }

    #[export]
    fn connect_to_player(&self, _owner: TRef<Sprite>) {
        let player_signal = unsafe { Node::get_tree(&_owner).unwrap()
            .assume_safe().root()
            .unwrap().assume_safe()
            .get_node("Game/Player")
            .unwrap().assume_safe() };

            player_signal.connect("player_interacting", _owner, 
            "emit_object_signal", VariantArray::new_shared(), 0).unwrap();
    }

    #[export]
    fn connect_signal_to_dialogue_box(&self, _owner: &Sprite) {
        let receiver = unsafe { Node::get_tree(_owner).unwrap()
            .assume_safe().root()
            .unwrap().assume_safe()
            .get_node("Game/Player/Camera2D/DialogueBox")
            .unwrap().assume_safe() };
        
        
        let my_signal_connected = _owner.connect("print_to_dialogue_box", 
        receiver, "_print_dialogue", VariantArray::new_shared(), 0);
    
        match my_signal_connected {
            Ok(()) => my_signal_connected.unwrap(),
            Err(e) => godot_error!("{}", e)
        };   
    }

}