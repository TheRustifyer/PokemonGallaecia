//! The [`player.rs`] file
//! 
//! Holds the type that represents the player controlled character in game
//! Holds the `gdext` bindings exposed to `Godot 4` 

use godot::prelude::*;
use godot::engine::{ICharacterBody2D, CharacterBody2D};

use super::player_direction::PlayerDirection;
use super::player_status::PlayerStatus;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    #[export(enum = (Idle, Walking, Running, Interacting))]
    status: i32,
    #[export(enum = (Downwards, Upwards, Left, Right))]
    direction: i32,
    #[base]
    character: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(character: Base<CharacterBody2D>) -> Self {
        godot_print!("Player Character initialized");
        
        Self {
            status: PlayerStatus::Idle as i32, // There's no other possible state in the initialization stage
            direction: PlayerDirection::default() as i32, // ! TODO: change it when the persistance is ready
            character
        }
    }

    fn ready(&mut self) {
        godot_print!("Player ready");
    }

    fn physics_process(&mut self, _delta: f64) {
    }
}
