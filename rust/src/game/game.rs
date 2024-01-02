//! Contains the ROOT elements of the project, which are the components and
//! bindings for the base of all nodes in our games hierarchy
use godot::{bind::{GodotClass, godot_api}, engine::{INode, Node}, obj::Base, log::godot_print};

/// General constant values that won't change though the whole lifecycle of the game
pub mod constants {
    /// Constant configuration values for entities related with the player's controlled character
    pub mod player {
        /// The rate at which the player's controlled character change it's possition in the map
        pub const WALK_SPEED: f64 = 400.0;
    }
}

/// Store constant data that comes from the engine
pub mod engine {
    /// Literals for matching the periphericals inputs
    pub mod input {
        pub const INPUT_EVENT_MOVE_UP: &'static str = "up";
        pub const INPUT_EVENT_MOVE_DOWN: &'static str = "down";
        pub const INPUT_EVENT_MOVE_LEFT: &'static str = "left";
        pub const INPUT_EVENT_MOVE_RIGHT: &'static str = "right";

    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Game {
    #[export] development_mode: bool,
    #[base] game: Base<Node>
}

#[godot_api]
impl INode for Game {
    fn init(game: Base<Node>) -> Self {
        godot_print!("<Game> initialized");
        
        Self {
            development_mode: true,
            game
        }
    }

    fn ready(&mut self) {
        godot_print!("<Game> ready");
        // let mut player_character_scene = load::<PackedScene>("res://scenes/player.tscn")
        //     .instantiate_as::<PlayerCharacter>()
        //     .upcast::<Node>();
        
        // self.game.add_child(player_character_scene.clone());
        // player_character_scene.set_owner(
        //     self.game.get_node(self.game.get_path()).unwrap()
        // );
        

        // for child in self.game.get_children().iter_shared() {
        //     let c = child.get_name();
        //     godot_print!("Child name: {:?}", c);
        // }
        
    }
}
