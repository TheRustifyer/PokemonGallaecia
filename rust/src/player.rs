//! The [`player.rs`] file
//! 
//! Holds the type that represents the player controlled character in game
//! Holds the `gdext` bindings exposed to `Godot 4` 

use godot::prelude::*;
use godot::engine::{ICharacterBody2D, CharacterBody2D};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    // #[export] asset: Gd<CharacterBody2D>

    #[base]
    character: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(character: Base<CharacterBody2D>) -> Self {
        godot_print!("Player Character initialized");
        
        Self {
            character
        }
    }

    fn ready(&mut self) {
        godot_print!("Player ready");
    }

    fn physics_process(&mut self, _delta: f64) {
    }
}
