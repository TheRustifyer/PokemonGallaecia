//! Holds the type that represents the player-controlled character within the game
//! 
//! This type is an specialization of the [`super::Character`] class

use godot::bind::{GodotClass, godot_api};
use godot::builtin::Vector2;
use godot::obj::{Base, Gd};
use godot::engine::{CharacterBody2D, ICharacterBody2D, Input};
use godot::log::godot_print;

use crate::game::character::character::CharacterState;
use crate::game::game::constants::player;
use super::character::status::CharacterStatus;
use super::game::engine::input::{INPUT_EVENT_MOVE_UP, INPUT_EVENT_MOVE_RIGHT, INPUT_EVENT_MOVE_DOWN, INPUT_EVENT_MOVE_LEFT};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    /// A smart pointer that holds the details about the current state of the player `Character`
    state: Gd<CharacterState>,
    /// Tracks the current 2 dimensional space point where the player is moving to
    #[export] #[var(get)] motion: Vector2,
    /// A constrained smart pointer to the engine type of this node
    #[base] base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("<PlayerCharacter>' initialized");
        Self {
            state: CharacterState::new(),
            motion: Vector2::ZERO,
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state.bind().get_character_status() != CharacterStatus::Interacting {
            self.process_player_input(delta);
        }
    }
}

#[godot_api]
impl PlayerCharacter {
   fn process_player_input(&mut self, delta: f64) {
        let input = Input::singleton();
        let speed = player::WALK_SPEED * delta as f32;

        if input.is_action_pressed(INPUT_EVENT_MOVE_UP.into()) {
            self.motion.y -= speed; 
        } else if input.is_action_pressed(INPUT_EVENT_MOVE_DOWN.into()) {
            self.motion.y += speed;
        } else if input.is_action_pressed(INPUT_EVENT_MOVE_LEFT.into()) {
            self.motion.x -= speed;
        } else if input.is_action_pressed(INPUT_EVENT_MOVE_RIGHT.into()) {
            self.motion.x += speed;
        } else {
            self.motion.x = 0.0;
            self.motion.y = 0.0;
        }

        godot_print!("Player position at: {:?}", self.base.get_position());
        self.base.move_and_collide(self.motion);

        // if input.is_key_pressed(keycode) // For better performance?¿!
   }
}