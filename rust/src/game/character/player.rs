//! Holds the type that represents the player-controlled character within the game
//! 
//! This type is an specialization of the [`super::Character`] class

use godot::{bind::{GodotClass, godot_api}, obj::Base, engine::{CharacterBody2D, ICharacterBody2D}, log::godot_print};
use crate::game::character::character::Character;

use super::{status::CharacterStatus, direction::CharacterDirection};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    // #[export] character: Gd<Character>,
    #[var(get, set = set_status)]
    #[export(enum = (Idle, Walking, Running, Interacting))]
    status: i32,

    #[var(get, set = set_direction)]
    #[export(enum = (Downwards, Upwards, Left, Right))]
    direction: i32,

    // character: Character, // TODO try to make it exportable, so we can reduce
    // it's cognitive complexity // Or just delegate behaviour to new traits, and
    // avoid have a composite type for nothing, since we can't make it exportable
    #[base] base: Base<CharacterBody2D>
}

#[godot_api]
impl PlayerCharacter {
    #[func]
    fn set_status(&mut self, value: i32) {
        let new_status = CharacterStatus::from(value);
        godot_print!("Setting player character status to: {new_status}");
        self.status = new_status as i32;
    }
    #[func]
    fn set_direction(&mut self, value: i32) {
        let new_direction = CharacterDirection::from(value);
        godot_print!("Setting player character status to: {new_direction}");
        self.direction = new_direction as i32;
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let character = Character::new(
            CharacterStatus::Idle, // There's no other possible state in the initialization stage for player-controlled characters
            CharacterDirection::default() // ! TODO: change it when the persistance is ready
        );

        Self {
            status: character.status as i32,
            direction: character.direction as i32,
            // character,
            base
        }
    }
}
