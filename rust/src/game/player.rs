//! Holds the type that represents the player-controlled character within the game
//! 
//! This type is an specialization of the [`super::Character`] class

use godot::bind::{GodotClass, godot_api};
use godot::obj::Base;
use godot::engine::{CharacterBody2D, ICharacterBody2D, Input};
use godot::log::godot_print;

use crate::game::character::character::CharacterState;
use super::character::status::CharacterStatus;
use super::game::engine::input::{INPUT_EVENT_MOVE_UP, INPUT_EVENT_MOVE_RIGHT, INPUT_EVENT_MOVE_DOWN, INPUT_EVENT_MOVE_LEFT};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    /// A smart pointer that holds the details about the current state of the player `Character`
    state: CharacterState,
    #[base] base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("<PlayerCharacter>' initialized");
        Self {
            state: CharacterState::new(),
            base
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.state.get_character_status() != CharacterStatus::Interacting {
            self.process_player_input();
        }
    }
}

#[godot_api]
impl PlayerCharacter {
   fn process_player_input(&self) {
        let input = Input::singleton();

        if input.is_action_pressed(INPUT_EVENT_MOVE_UP.into()) {
            godot_print!("move up pressed");
        }
        if input.is_action_pressed(INPUT_EVENT_MOVE_DOWN.into()) {
            godot_print!("move down pressed");
        }
        if input.is_action_pressed(INPUT_EVENT_MOVE_LEFT.into()) {
            godot_print!("move left pressed");
        }
        if input.is_action_pressed(INPUT_EVENT_MOVE_RIGHT.into()) {
            godot_print!("move right pressed");
        }

        // if input.is_key_pressed(keycode) // For better performance?¿!
   }
}