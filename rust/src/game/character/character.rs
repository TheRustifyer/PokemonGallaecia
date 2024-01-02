//! The [`character.rs`] file
//! 
//! Holds a generic binding class for any representable character in the game,
// //! whether is the player-controllable character or an NPC

use godot::prelude::*;

use super::direction::CharacterDirection;
use super::status::CharacterStatus;

/// A general purpose type for holding data driven behaviour and encapsulate state details
/// about properties that has in common both player-controlled characters and NPCs
#[derive(Debug, GodotClass, Default)]
#[class(base=Node)]
pub struct CharacterState {
    #[var(get = get_integral_status, set = set_integral_status)]
    #[export(enum = (Idle, Walking, Running, Interacting))]
    status: i32,
    
    #[var(get = get_integral_direction, set = set_integral_direction)]
    #[export(enum = (Downwards, Upwards, Left, Right))]
    direction: i32,

    #[var(get, set)]
    #[export]
    initial_position: Vector2
}

#[godot_api]
impl CharacterState {
    pub fn new() -> Gd<Self> {
        godot_print!("<CharacterState> constructed");
        Gd::from_init_fn(|_base| {
            Self::default()
        })
    }

    /// Retrieves the current [`CharacterStatus`] stored in this node
    pub fn get_status(&self) -> CharacterStatus {
        CharacterStatus::from(self.status)
    }

    /// Retrieves the current [`CharacterDirection`] stored in this node
    pub fn get_direction(&self) -> CharacterDirection {
        CharacterDirection::from(self.direction)
    }

    /// Modifies the current [`CharacterStatus`] stored in this node
    pub fn set_status(&mut self, status: CharacterStatus) {
        self.status = status.into();
    }

    /// Modifies the current [`CharacterDirection`] stored in this node
    pub fn set_direction(&mut self, direction: CharacterDirection) {
        self.direction = direction.into();
    }

    #[func]
    pub fn get_integral_status(&self) -> i32 {
        self.status
    }

    #[func]
    pub fn get_integral_direction(&self) -> i32 {
        self.direction
    }

    #[func]
    pub fn set_integral_status(&mut self, value: i32) {
        let new_status = CharacterStatus::from(value);
        godot_print!("Setting <CharacterState> 'status' to: {new_status}");
        self.status = new_status as i32;
    }

    #[func]
    pub fn set_integral_direction(&mut self, value: i32) {
        let new_direction = CharacterDirection::from(value);
        godot_print!("Setting <CharacterState> 'status' to: {new_direction}");
        self.direction = new_direction as i32;
    }
}

#[godot_api]
impl INode for CharacterState {
    fn init(_base: Base<Node>) -> Self {
        godot_print!("<CharacterState> initialized");
        Self {..Default::default()}
    }

    fn ready(&mut self) {
        godot_print!("`Character State` ready");
    }

    fn physics_process(&mut self, _delta: f64) {
    }
}
