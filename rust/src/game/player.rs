//! Holds the type that represents the player-controlled character within the game
//! 
//! This type is an specialization of the [`super::Character`] class

use godot::bind::{GodotClass, godot_api};
use godot::obj::{Base, Gd};
use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::log::godot_print;
use crate::game::character::character::CharacterState;

use super::character::{status::CharacterStatus, direction::CharacterDirection};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    /// A smart pointer that holds the details about the current state of the player `Character`
    #[allow(unused)] state: Gd<CharacterState>,
    #[base] base: Base<CharacterBody2D>
}

// #[godot_api]
// impl PlayerCharacter {
//    
// }

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("<PlayerCharacter>' initialized");
        Self {
            state: Gd::from_init_fn(|character_status_base| {
                CharacterState {
                    status: CharacterStatus::Idle as i32, // There's no other possible state in the initialization stage for player-controlled characters
                    direction: CharacterDirection::default() as i32, // ! TODO: change it when the persistance is ready
                    base: character_status_base
                }
            }),
            base
        }
    }
}
