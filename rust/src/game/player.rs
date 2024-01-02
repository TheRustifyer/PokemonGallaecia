//! Holds the type that represents the player-controlled character within the game
//! 
//! This type is an specialization of the [`super::Character`] class

use godot::bind::{GodotClass, godot_api};
use godot::builtin::Vector2;
use godot::obj::{Base, Gd, UserClass, GdMut};
use godot::engine::{CharacterBody2D, ICharacterBody2D, Input};
use godot::log::godot_print;

use crate::game::character::character::CharacterState;
use crate::game::game::constants::player;
use super::character::animation::CharacterAnimation;
use super::character::direction::CharacterDirection;
use super::character::status::CharacterStatus;
use super::game::engine::input::{INPUT_EVENT_MOVE_UP, INPUT_EVENT_MOVE_RIGHT, INPUT_EVENT_MOVE_DOWN, INPUT_EVENT_MOVE_LEFT};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    /// A smart pointer that holds the details about the current state of the player `Character`
    state: Gd<CharacterState>,
    /// Tracks the current 2 dimensional space point where the player is moving to
    #[export] #[var(get)] motion: Vector2,
    /// A reference counter smart pointer to the engine's node that hosts the player controlled character animation
    animation: Gd<CharacterAnimation>,
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
            animation: CharacterAnimation::alloc_gd(), // Mocked initial value to avoid using an Option wrapper
            base
        }
    }

    fn ready(&mut self) {
        // TODO make some sort of constant values or an enumerated type to avoid using a numerical literal to get the child
        self.animation = self.base
            .get_child(1)
            .map(|node| node.cast())
            .expect("Not found the Player's character animation where we expected");
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state.bind().get_status() != CharacterStatus::Interacting {
            self.process_player_input(delta);
        }
    }
}

#[godot_api]
impl PlayerCharacter {
   fn process_player_input(&mut self, delta: f64) {
        let input = Input::singleton();
        let speed = player::WALK_SPEED * delta as f32;
        let mut state: GdMut<'_, CharacterState> = self.state.bind_mut();
        let mut animation: GdMut<'_, CharacterAnimation> = self.animation.bind_mut();

        // if input.is_key_pressed(keycode) // For better performance?¿!
        if input.is_action_pressed(INPUT_EVENT_MOVE_UP.into()) {
            self.motion.y -= speed;
            state.set_direction(CharacterDirection::Upwards);
            animation.play_anim("walking upwards");
        } else if input.is_action_pressed(INPUT_EVENT_MOVE_DOWN.into()) {
            self.motion.y += speed;
            state.set_direction(CharacterDirection::Downwards);
            animation.play_anim("walking downwards");
        } else if input.is_action_pressed(INPUT_EVENT_MOVE_LEFT.into()) {
            self.motion.x -= speed;
            state.set_direction(CharacterDirection::Left);
            animation.play_anim("walking left");
        } else if input.is_action_pressed(INPUT_EVENT_MOVE_RIGHT.into()) {
            self.motion.x += speed;
            state.set_direction(CharacterDirection::Right);
            animation.play_anim("walking right");
        } else {
            self.motion.x = 0.0;
            self.motion.y = 0.0;

            animation.play_anim( // TODO refactor into a method of CharacterMotion?? New type or what?
                match state.get_direction() {
                    CharacterDirection::Downwards => "idle downwards",
                    CharacterDirection::Upwards => "idle upwards",
                    CharacterDirection::Left => "idle left",
                    CharacterDirection::Right => "idle right",
                }
            );
            // animation.play_anim("walking dead"); <-- XD LOL JOKE
        }

        self.base.move_and_collide(self.motion);
   }
}