//! The [`Character.rs`] file
//! 
//! Holds a generic binding class for any representable character in the game,
// //! whether is the player-controllable character or an NPC

// use godot::prelude::*;
// use godot::engine::{ICharacterBody2D, CharacterBody2D};

use super::direction::CharacterDirection;
use super::status::CharacterStatus;

/// A general purpose base type for characters in the game
#[derive(Debug, Default)]
// #[class(base=CharacterBody2D)]
pub struct Character {
    pub status: CharacterStatus,
    pub direction: CharacterDirection
}

impl Character {
    pub fn new(status: CharacterStatus, direction: CharacterDirection) -> Self {
        Self {
            status, 
            direction
        }
    }

}

// #[godot_api]
// impl ICharacterBody2D for Character {
//     fn init(character_body: Base<CharacterBody2D>) -> Self {
//         godot_print!("Character Character initialized");
        
//         Self {
//             status: CharacterStatus::Idle as i32, // There's no other possible state in the initialization stage
//             direction: CharacterDirection::default() as i32, // ! TODO: change it when the persistance is ready
//             character_body
//         }
//     }

//     fn ready(&mut self) {
//         godot_print!("Character ready");
//     }

//     fn physics_process(&mut self, _delta: f64) {
//     }
// }
