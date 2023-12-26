use gdnative::prelude::*;

use crate::game::code_abstractions::{dialogue_connections::DialogueBoxActions, signals::RegisterSignal};

#[derive(GodotClass)]
#[class(base=Sprite)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct Truck {
    times_signal_emitted: i32,
}

// Implements the necesary methods that make this struct able to print text on screen.
impl DialogueBoxActions for Truck { }

impl RegisterSignal<Self> for Truck {
    fn register_signal(_builder: &ClassBuilder<Self>) -> () {
        _builder.signal( "print_to_dialogue_box").done();
    }
}

#[methods]
impl Truck {
    
    fn new(_owner: &Sprite) -> Self {
        Self {
            times_signal_emitted: 0,
        }
    }

    
    fn _ready(&self, base: TRef<Sprite>) {
        // Looking for interactions with the player
        self.connect_to_player(base);
        // Connects this object with the dialogue box
        self.connect_signal_to_dialogue_box(&base)
    }

    
    /// Method that receives the signal that the player it's interacting, so this object can emit the text to print via signal.
    ///
    /// The content it's passed to the `Dialogue Box` struct via tuple to avoid corrupt the data
    /// converting the struct fields to Variant data. When arrives to Dialogue Box, data gets parsed into the 
    /// `DialogueElection<T>` struct, and finally gets ready to print and interact.
    fn emit_object_signal(&self, base: TRef<Sprite>) {
        let dialogue_data = (
            1, 
            vec!["Si", "No"],
            vec![
                "Soy el camión de pueblo de Teo".to_owned()
                + &"\nQuiero contarte un secreto sobre RUST.".to_owned()
                + &"\nQuieres saberlo?",
                "El compilador de RUST está to broken, bro".to_owned(),
                "Pues quédate con JAJAJAJAJAVA, pringao.".to_owned()
            ]
        );

        base.emit_signal("print_to_dialogue_box", &[
                dialogue_data.to_variant()
            ]
        );
    }
}
