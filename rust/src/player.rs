pub mod player_mod {
    use gdnative::prelude::*;
    use gdnative::api::{AnimatedSprite, Area2D, CollisionShape2D, KinematicBody2D};

    pub const VELOCITY: f32 = 300.0;
    pub const GRAVITY: f32 = 500.0;
    const SCREEN_SIZE: Vector2 = Vector2::new(0.0, 0.0);

    #[derive(NativeClass)]
    #[inherit(KinematicBody2D)]
    #[user_data(user_data::MutexData<Player>)]
    // #[register_with(Self::register_player)]
    #[derive(Debug)]
    pub struct Player {
        username: Option<String>,
        password: Option<String>,
        level: Option<u8>,
    }

    impl ToVariant for Player {
        fn to_variant(&self) -> Variant {
            todo!()
        }
    }
    
    #[gdnative::methods]
    impl Player {  
        // The public constructor
        fn new(_owner: &KinematicBody2D) -> Self {
            godot_print!("CONSTRUCTOR");
            Self {
                username: None,
                password: None,
                level: None,
            }
        }

        pub fn create_new_player(
            username: String, 
            password: String, 
            level: u8) -> Self {

            let player: Player = Player { 
                username: Some(username), 
                password: Some(password), 
                level: Some(level)
            };

            player
        }
        
        #[export]
        fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
            // First of all, we need a reference to our singleton(scene, node, value that exists through out the game) Input 
            let input: &Input = Input::godot_singleton();

            // A Vector2, which is a Godot type, which represents the (x, y) coordinates on 2D space
            let mut motion: Vector2 = Vector2::new(0.0, 0.0);

            // All Y axis motions are affected first by the gravity
            motion.y += GRAVITY;

            if Input::is_action_pressed(&input, "left") {
                motion.x = -VELOCITY;
            } 
            if Input::is_action_pressed(&input, "right") {
                motion.x = VELOCITY;
            } 
            if Input::is_action_pressed(&input, "up") {
                motion.y = -VELOCITY;
            } 
            if Input::is_action_pressed(&input, "down") {
                motion.y = VELOCITY;
            }

            owner.move_and_slide(
                motion,
                motion,
                false,
                4,
                0.785398,
                true
            );
        

        
        // let change = motion * delta;
        // let position = owner.global_position() + change;
        // let position = Vector2::new(
        //     position.x.max(0.0).min(screen_size.x),
        //     position.y.max(0.0).min(screen_size.y),
        // );
        // owner.set_global_position(position);
    }

        // fn register_player(builder: &ClassBuilder<Self>) {
        //     builder.add_signal(Signal {
        //         name: "hit",
        //         args: &[],
        //     });
        // }

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
}