//! The [`Character.rs`] file
//! 
//! Holds a generic binding class for any representable character in the game,
// //! whether is the player-controllable character or an NPC

use godot::prelude::*;

use super::direction::CharacterDirection;
use super::status::CharacterStatus;

/// A general purpose type for holding data driven behaviour and encapsulate state details
/// about properties that has in common both player-controlled characters and NPCs
#[derive(Debug, GodotClass)]
#[class(base=Node)]
pub struct CharacterState {
    #[var(get, set = set_status_from_discriminant)]
    #[export(enum = (Idle, Walking, Running, Interacting))]
    pub status: i32,
    
    #[var(get, set = set_direction_from_discriminant)]
    #[export(enum = (Downwards, Upwards, Left, Right))]
    pub direction: i32,

    // TODO refcount to the parent?

    #[base] pub base: Base<Node>
}

#[godot_api]
impl CharacterState {
    #[func]
    fn set_status_from_discriminant(&mut self, value: i32) {
        let new_status = CharacterStatus::from(value);
        godot_print!("Setting <CharacterState> 'status' to: {new_status}");
        self.status = new_status as i32;
    }

    #[func]
    fn set_direction_from_discriminant(&mut self, value: i32) {
        let new_direction = CharacterDirection::from(value);
        godot_print!("Setting <CharacterState> 'status' to: {new_direction}");
        self.direction = new_direction as i32;
    }
}

#[godot_api]
impl INode for CharacterState {
    fn init(base: Base<Node>) -> Self {
        godot_print!("<CharacterState> initialized");
        
        Self {
            status: CharacterStatus::Idle as i32, // There's no other possible state in the initialization stage
            direction: CharacterDirection::default() as i32, // ! TODO: change it when the persistance is ready
            base
        }
    }

    fn ready(&mut self) {
        godot_print!("`Character State` ready");
    }

    fn physics_process(&mut self, _delta: f64) {
    }
}
