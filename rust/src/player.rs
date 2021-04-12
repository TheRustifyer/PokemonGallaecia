use gdnative::prelude::*;
use gdnative::api::{AnimatedSprite, KinematicBody2D};

use crate::consts::in_game_constant;

/// Base class for that holds the user's account related data of the real person
///
/// This one allows to create new Gamer instances mapping the `client actions`: 
/// * account level -> Global level of the Gamer Player
/// * see your points/coins
/// * unlocked features
/// * choose avatars 
/// * track languages progress -> Player Character level
/// * character designs...
#[derive(Debug)]
pub struct Gamer {
    username: Option<String>,
    password: Option<String>,
    level: Option<i8>, // This should be a hash map that tracks language : level
}

impl Gamer {
    
    /// Method that login into the client a registered gamer
    pub fn gamer_login(
        username: String, 
        password: String, 
        level: i8) -> Self {

        let gamer: Gamer = Gamer { 
            username: Some(username), 
            password: Some(password), 
            level: Some(level),
        };
        gamer
    }

    pub fn check_credentials(username: Option<&String>, password: Option<&String>) -> (bool, bool) {

        let mut credentials_flag: (bool, bool) = (false, false);

        // Upgraded flat String credentials to std::option:Option, in order to use pattern matching
        //to make an ez way to scale future options when will be checked on a REST-backend
        match username {
            Some(usnm) if usnm == "root" || usnm == "Root" => credentials_flag.0 = true,
            Some(usnm) if usnm == "" => godot_print!("Provide an username"), // While insert an informative label as a child isn't implemented
            Some(_) => (),
            None => panic!(),
        }

        match password {
            Some(pswd) if pswd == "root" || pswd == "Root" => credentials_flag.1 = true,
            Some(pswd) if pswd == "" => godot_print!("Provide a password"),  // While insert an informative label as a child isn't implemented
            Some(_) => (),
            None => panic!() 
        }
        // Returns a tuple representing the checked status of each credential
        credentials_flag
    }

    /// Little method to convert the credentials (retrieved as a tuple of GodotStrings) into a tuple of Strings
    pub fn credentials_to_rust_string(cred_tup: (GodotString, GodotString)) -> (String, String) {
        let credentials = cred_tup;
        (credentials.0.to_string(), credentials.1.to_string())
    }
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct PlayerCharacter {
    // A Vector2, which is a Godot type, which represents the (x, y) coordinates on 2D space
    motion: Vector2,
}

// impl ToVariant for Player {
//     fn to_variant(&self) -> Variant {
//         todo!()
//     }
// }

#[gdnative::methods]
impl PlayerCharacter {  

    /// Method for register a new signal to a designed class. You can find on the GUI Godot
    /// that signal registered on the Node panel on the same way if the signal was created directly on the GUI.
    /// The name of the method is completly arbitrary, is just a way to encapsulate the info passed to the builder object and transport it back to Godot 
    fn register_signal(builder: &ClassBuilder<Self>) {
        
        builder.add_signal( Signal {
            name: "animate",
            args: &[ SignalArgument {
                name: "motion",
                default: Variant::from_vector2(&Vector2::new(0.0, 0.0)),
                export_info: ExportInfo::new(VariantType::Vector2),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    // The constructor
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            motion: Vector2::new(0.0, 0.0)
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

        if Input::is_action_pressed(&input, "Jump") && owner.is_on_floor() {
            self.motion.y -= in_game_constant::JUMP_SPEED
        }
        if Input::is_action_pressed(&input, "Left") && 
            !Input::is_action_pressed(&input, "Right") &&
            !Input::is_action_pressed(&input, "Up") &&
            !Input::is_action_pressed(&input, "Down") {
            self.motion.x = in_game_constant::VELOCITY * -1.0;
        } 
        else if Input::is_action_pressed(&input, "Right") && 
            !Input::is_action_pressed(&input, "Left") &&
            !Input::is_action_pressed(&input, "Up") &&
            !Input::is_action_pressed(&input, "Down") {
            self.motion.x = in_game_constant::VELOCITY;
        } 
        else if Input::is_action_pressed(&input, "Up") &&
            !Input::is_action_pressed(&input, "Down") &&
            !Input::is_action_pressed(&input, "Right") &&
            !Input::is_action_pressed(&input, "Left") {
            self.motion.y = in_game_constant::VELOCITY * - 1.0;
        } 
        else if Input::is_action_pressed(&input, "Down") &&
            !Input::is_action_pressed(&input, "Up") &&
            !Input::is_action_pressed(&input, "Left") &&
            !Input::is_action_pressed(&input, "Right") {
            self.motion.y = in_game_constant::VELOCITY;
        }
        else {
            self.motion.x = 0.0;
            self.motion.y = 0.0;
        }

        owner.move_and_slide(
            self.motion,
            in_game_constant::UP,
            false,
            4,
            0.785398,
            false
        );

    }

    // fn apply_gravity(&mut self, owner: &KinematicBody2D) {
    //     if owner.is_on_floor() {
    //         self.motion.y = 0.0;
    //     } else {
    //         self.motion.y += in_game_constant::GRAVITY;
    //     }
    // }

    fn animate_character(&self, owner: &KinematicBody2D) {
        owner.emit_signal("animate", &[self.motion.to_variant()]);
    }
}

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
pub struct PlayerAnimation {
    current_player_direction: PlayerDirection,
    idle_player_direction: PlayerDirection
}
#[derive(PartialEq, Clone, Debug)]
enum PlayerDirection {
    Idle,
    Upwards,
    Downwards,
    Left,
    Right
}

impl Default for PlayerDirection {
    fn default() -> Self { PlayerDirection::Idle }
}


#[gdnative::methods]
impl PlayerAnimation {
    fn new(_owner: &AnimatedSprite) -> Self {
        Self {
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
            x if x.x > 0.0 => self.current_player_direction = PlayerDirection::Right,
            x if x.x < 0.0 => self.current_player_direction = PlayerDirection::Left,
            y if y.y < 0.0 => self.current_player_direction = PlayerDirection::Upwards,
            y if y.y > 0.0 => self.current_player_direction = PlayerDirection::Downwards,
            z => if z.x == 0.0 && z.y == 0.0 {
                { self.current_player_direction = PlayerDirection::Idle; }
            }
        }

        if self.current_player_direction == PlayerDirection::Idle {
            match self.idle_player_direction {
                PlayerDirection::Downwards => { character_animated_sprite.play("idle front", false); godot_print!("Idle front") }
                PlayerDirection::Upwards => { character_animated_sprite.play("idle back", false); godot_print!("Idle back") }
                PlayerDirection::Left => { character_animated_sprite.play("idle left", false); godot_print!("Idle left") }
                PlayerDirection::Right => { character_animated_sprite.play("idle right", false); godot_print!("Idle right") }
                _ => { character_animated_sprite.play("idle front", false);
                    godot_print!("This should never be reached, but pattern matching in Rust is always an exhaustive action.") }
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