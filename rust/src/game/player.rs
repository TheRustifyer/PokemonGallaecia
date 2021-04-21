use std::collections::HashMap;

use gdnative::prelude::*;
use gdnative::api::{AnimatedSprite, KinematicBody2D, KinematicCollision2D, NinePatchRect};

use crate::game::*;
use super::game_elements::signals::GodotSignal;
use self::game_elements::signals::RegisterSignal;

use crate::utils::consts::in_game_constant;
// Signal {
//     name: "animate",
//     args: &[ SignalArgument {
//         name: "motion",
//         default: Variant::from_vector2(&Vector2::new(0.0, 0.0)),
//         export_info: ExportInfo::new(VariantType::Vector2),
//         usage: PropertyUsage::DEFAULT,
//     }],
// });
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct PlayerCharacter {
    // A Vector2, which is a Godot type, in this case representing the (x, y) coordinates on 2D space
    motion: Vector2,
    signals: HashMap<String, GodotSignal<'static>>,
}

impl RegisterSignal<Self> for PlayerCharacter {

    fn register_signal(builder: &ClassBuilder<Self>) {

        builder.add_signal( Signal {
            name: "animate",
            args: &[ SignalArgument {
                name: "motion",
                default: Variant::from_vector2(&Vector2::new(0.0, 0.0)),
                export_info: ExportInfo::new(VariantType::Vector2),
                usage: PropertyUsage::DEFAULT,
                    }
                ],
            }
        );

    }
}


#[gdnative::methods]
impl PlayerCharacter {  

    // The constructor
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
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

        // Manage the player motion
        if Input::is_action_pressed(&input, "Left") {
            self.motion.x = in_game_constant::VELOCITY * -1.0;
            self.motion.y = 0.0;    
        } 
        else if Input::is_action_pressed(&input, "Right") {
            self.motion.x = in_game_constant::VELOCITY;
            self.motion.y = 0.0;
        } 
        else if Input::is_action_pressed(&input, "Up") {
            self.motion.y = in_game_constant::VELOCITY * - 1.0;
            self.motion.x = 0.0;
        } 
        else if Input::is_action_pressed(&input, "Down") {
            self.motion.y = in_game_constant::VELOCITY;
            self.motion.x = 0.0;
        }
        else {
            self.motion.x = 0.0;
            self.motion.y = 0.0;
        }

        let player_movement = owner.move_and_collide(
            self.motion * _delta, false, false, false);
        
        if Input::is_action_pressed(&input, "Interact") {
            self.interact(owner, player_movement);
        }
    }

    fn interact(&self, _owner: &KinematicBody2D, pl_mov: Option<Ref<KinematicCollision2D>>) {

        match pl_mov {
            Some(pl_mov) => { 
                let collision: TRef<KinematicCollision2D, Shared> = unsafe { pl_mov.assume_safe() }; 
                godot_print!("collision: {:?}",  &collision);

                let coll_body: TRef<Object> = unsafe { collision.collider().unwrap().assume_safe() };
                godot_print!("collision with: {:?}", coll_body);

                godot_print!("Has node: {:?}", coll_body.cast::<Node>().unwrap().has_node("Interact"));

                let my_label = unsafe { _owner.get_node_as::<NinePatchRect>("Camera2D/DialogueBox")
                    .unwrap()
                     };
                my_label.set_visible(true);
                
                    
                // let powered_label = my_label.cast::<RichTextLabel>().unwrap();
                godot_print!("Label visibily: {:?}", my_label.is_visible())
            } ,
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
    current_player_motion: PlayerMotionStatus,
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
                { self.current_player_direction = PlayerDirection::Right; self.current_player_motion = PlayerMotionStatus::Walking },

            x if x.x < 0.0 => 
                { self.current_player_direction = PlayerDirection::Left; self.current_player_motion = PlayerMotionStatus::Walking }, 

            x if x.y < 0.0 => 
                { self.current_player_direction = PlayerDirection::Upwards; self.current_player_motion = PlayerMotionStatus::Walking },
            
            x if x.y > 0.0 => 
                { self.current_player_direction = PlayerDirection::Downwards; self.current_player_motion = PlayerMotionStatus::Walking },
            
            _ => 
                { self.current_player_motion = PlayerMotionStatus::Idle }
                
        }


        if self.current_player_motion == PlayerMotionStatus::Idle {
            match self.idle_player_direction {
                PlayerDirection::Downwards => { character_animated_sprite.play("idle front", false); }//godot_print!("Idle front") }
                PlayerDirection::Upwards => { character_animated_sprite.play("idle back", false); }//godot_print!("Idle back") }
                PlayerDirection::Left => { character_animated_sprite.play("idle left", false); }//godot_print!("Idle left") }
                PlayerDirection::Right => { character_animated_sprite.play("idle right", false); }//godot_print!("Idle right") }
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
enum PlayerMotionStatus {
    Idle,
    Walking,
    Running
}

impl Default for PlayerMotionStatus {
    fn default() -> Self { PlayerMotionStatus::Idle }
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