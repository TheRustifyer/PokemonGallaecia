//! Holds the type that represents the player-controlled character within the game
//! 
//! This type is an specialization of the [`super::Character`] class

use godot::bind::{GodotClass, godot_api};
use godot::obj::{Base, Gd};
use godot::engine::{CharacterBody2D, ICharacterBody2D, Input};
use godot::log::godot_print;
use crate::game::character::character::CharacterState;

use super::character::{status::CharacterStatus, direction::CharacterDirection};
use super::game::engine::input::{INPUT_EVENT_MOVE_UP, INPUT_EVENT_MOVE_RIGHT, INPUT_EVENT_MOVE_DOWN, INPUT_EVENT_MOVE_LEFT};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    /// A smart pointer that holds the details about the current state of the player `Character`
    #[allow(unused)] state: Gd<CharacterState>,
    #[base] base: Base<CharacterBody2D>
}

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

    fn physics_process(&mut self, _delta: f64) {
        // TODO consider to avoid having the Character component as a Gd ptr,
        // since we don't need to hold a reference to the base node of such component
        // and we could avoing calling bind() for get interior access
        if self.state.bind().get_character_status() != CharacterStatus::Interacting {
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