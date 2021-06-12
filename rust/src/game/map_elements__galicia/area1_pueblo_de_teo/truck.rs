use gdnative::prelude::*;

use crate::game::code_abstractions::{dialogue_connections::DialogueBoxActions, signals::RegisterSignal};

#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct Truck {
    times_signal_emitted: i32,
}

// Default impl
impl DialogueBoxActions for Truck { }

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
        // Connects this object with the dialogue box
        self.connect_signal_to_dialogue_box(&_owner)
    }

    #[export]
    /// Method that receives the signal that the player it's interacting, so this object can emit the text to print via signal.
    fn emit_object_signal(&self, _owner: TRef<Sprite>) {
        _owner.emit_signal("print_to_dialogue_box", &[Variant::from_godot_string(
            &GodotString::from_str(
                "Soy el camión de pueblo de Teo".to_owned()
                + &"\nEsto sólo es una línea de prueba.".to_owned()
                + &"\nY esta otra más".to_owned()
                + &"\nY esta otra más".to_owned()
                + &"\nY esta otra más".to_owned()
                + "\nY esta otra más"
                )
            )
        ]);
    }

}
