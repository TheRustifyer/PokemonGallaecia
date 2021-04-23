use std::collections::HashMap;

use gdnative::prelude::*;
use gdnative::api::{AnimatedSprite, KinematicBody2D, KinematicCollision2D};

use crate::game::*;
use super::game_elements::signals::GodotSignal;
use self::game_elements::signals::RegisterSignal;

use crate::utils::consts::in_game_constant;
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct PlayerCharacter {
    player_status: PlayerStatus,
    // A Vector2, which is a Godot type, in this case representing the (x, y) coordinates on 2D space
    motion: Vector2,
    signals: HashMap<String, GodotSignal<'static>>,
}

impl RegisterSignal<Self> for PlayerCharacter {

    fn register_signal(builder: &ClassBuilder<Self>) {
        // The signal that indicates that the Player is moving
        builder.add_signal( Signal {
            name: "animate",
            args: &[ SignalArgument {
                name: "motion",
                default: Variant::from_vector2(&Vector2::new(0.0, 0.0)),
                export_info: ExportInfo::new(VariantType::Vector2),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        // Indicates that the Player is interacting
        builder.add_signal( Signal {
            name: "player_interacting",
            args: &[],
        });
    }
}


#[gdnative::methods]
impl PlayerCharacter {  

    // The constructor
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            player_status: Default::default(),
            motion: Vector2::new(0.0, 0.0),
            signals: HashMap::new()
        }
    }
    
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        // First of all, we need a reference to our singleton(scene, node, value that exists through out the game) Input 
        let input: &Input = Input::godot_singleton();

        // All Y axis motions are affected first by the gravity
        // self.apply_gravity(&owner);
        // Calling the method who animates the sprite when KinematicBody2D is moving
        self.animate_character(&owner);

        if self.player_status != PlayerStatus::Interacting {
            // Manage the player motion
            if Input::is_action_pressed(&input, "Left") {
                self.motion.x = in_game_constant::VELOCITY * -1.0;
                self.motion.y = 0.0;
                self.player_status = PlayerStatus::Walking    
            } 
            else if Input::is_action_pressed(&input, "Right") {
                self.motion.x = in_game_constant::VELOCITY;
                self.motion.y = 0.0;
                self.player_status = PlayerStatus::Walking 
            } 
            else if Input::is_action_pressed(&input, "Up") {
                self.motion.y = in_game_constant::VELOCITY * - 1.0;
                self.motion.x = 0.0;
                self.player_status = PlayerStatus::Walking 
            } 
            else if Input::is_action_pressed(&input, "Down") {
                self.motion.y = in_game_constant::VELOCITY;
                self.motion.x = 0.0;
                self.player_status = PlayerStatus::Walking 
            }
            else {
                self.motion.x = 0.0;
                self.motion.y = 0.0;
                self.player_status = PlayerStatus::Idle
            }
        }

        let player_movement = owner.move_and_collide(
            self.motion * _delta, false, false, false);
        
        if Input::is_action_pressed(&input, "Interact") {
            if self.player_status != PlayerStatus::Interacting {
                self.interact(owner, player_movement);
            }
        }
    }

    #[export]
    fn handle_interaction(&mut self, _owner: &KinematicBody2D, info: String) {
        // godot_print!("INFO: {:?}", text);
        if info == "on_dialogue" {
            godot_print!("Player on dialogue");
            self.player_status = PlayerStatus::Interacting;
            self.motion.x = 0.0;
            self.motion.y = 0.0;
        } else {
            godot_print!("Player released");
            self.player_status = PlayerStatus::default();
        }
    }

    fn interact(&mut self, _owner: &KinematicBody2D, pl_mov: Option<Ref<KinematicCollision2D>>) {
        match pl_mov {
            Some(pl_mov) => { 
                let collision: TRef<KinematicCollision2D, Shared> = unsafe { pl_mov.assume_safe() }; 

                let coll_body: TRef<Node> = unsafe { 
                    collision
                    .collider()
                    .unwrap()
                    .assume_safe()
                 }.cast::<Node>().unwrap();

                // Notifies the game that the player is interacting
                if coll_body.has_node("Interact") {
                    self.player_status = PlayerStatus::Interacting;
                    _owner.emit_signal("player_interacting", &[]);
                }
            },
            _ => ()
        }
    }

    fn animate_character(&self, owner: &KinematicBody2D) {
        owner.emit_signal("animate", &[self.motion.to_variant()]);
    }


}

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
pub struct PlayerAnimation {
    current_player_motion: PlayerStatus,
    current_player_direction: PlayerDirection,
    idle_player_direction: PlayerDirection
}

#[gdnative::methods]
impl PlayerAnimation {
    fn new(_owner: &AnimatedSprite) -> Self {
        Self {
            current_player_motion: Default::default(),
            current_player_direction: Default::default(),
            idle_player_direction: Default::default()
        }
    }

    #[export]
    fn _on_player_animate(&mut self, _owner: &AnimatedSprite, _motion: Vector2) {
        
        let character_animated_sprite = unsafe 
        { _owner.get_node_as::<AnimatedSprite>(
            "."
            ) }
            .unwrap();

        match _motion {
            x if x.x > 0.0 => 
                { self.current_player_direction = PlayerDirection::Right; self.current_player_motion = PlayerStatus::Walking },

            x if x.x < 0.0 => 
                { self.current_player_direction = PlayerDirection::Left; self.current_player_motion = PlayerStatus::Walking }, 

            x if x.y < 0.0 => 
                { self.current_player_direction = PlayerDirection::Upwards; self.current_player_motion = PlayerStatus::Walking },
            
            x if x.y > 0.0 => 
                { self.current_player_direction = PlayerDirection::Downwards; self.current_player_motion = PlayerStatus::Walking },
            
            _ => 
                { self.current_player_motion = PlayerStatus::Idle }
                
        }


        if self.current_player_motion == PlayerStatus::Idle {
            match self.idle_player_direction {
                PlayerDirection::Downwards => { character_animated_sprite.play("idle front", false); }
                PlayerDirection::Upwards => { character_animated_sprite.play("idle back", false); }
                PlayerDirection::Left => { character_animated_sprite.play("idle left", false); }
                PlayerDirection::Right => { character_animated_sprite.play("idle right", false); }
                // The starting position when the Player spawns on the screen
                _ => character_animated_sprite.play("idle front", false)
            }; 

        } else if self.current_player_direction == PlayerDirection::Right {
            character_animated_sprite.play("walk right", false);
            self.idle_player_direction = PlayerDirection::Right;

        } else if PlayerDirection::Left == self.current_player_direction {
            character_animated_sprite.play("walk left", false);
            self.idle_player_direction = PlayerDirection::Left;

        } else if PlayerDirection::Downwards == self.current_player_direction {
            character_animated_sprite.play("walk downwards", false);
            self.idle_player_direction = PlayerDirection::Downwards;

        } else if PlayerDirection::Upwards == self.current_player_direction {
            character_animated_sprite.play("walk upwards", false);
            self.idle_player_direction = PlayerDirection::Upwards;

        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum PlayerStatus {
    Idle,
    Walking,
    // Running
    Interacting
}

impl Default for PlayerStatus {
    fn default() -> Self { PlayerStatus::Idle }
}

#[derive(PartialEq, Clone, Debug)]
enum PlayerDirection {
    Idle, // De momento necesitamos esto
    Upwards,
    Downwards,
    Left,
    Right
}

impl Default for PlayerDirection {
    fn default() -> Self { PlayerDirection::Idle }
}