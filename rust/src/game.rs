//! Contains the ROOT elements of the project, which are the components and
//! bindings for the base of all nodes in our games hierarchy

use godot::{bind::{GodotClass, godot_api}, engine::{INode, Node}, obj::Base, log::godot_print};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Game {
    #[base] this: Base<Node> 
}

#[godot_api]
impl INode for Game {
    fn init(game: Base<Node>) -> Self {
        godot_print!("Game initialized");
        Self {
            this: game
        }
    }

    fn ready(&mut self) {
        godot_print!("Game ready");
    }
}
