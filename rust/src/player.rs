//! The [`player.rs`] file
//! 
//! Holds the type that represents the player controlled character in game
//! Holds the `gdext` bindings exposed to `Godot 4` 

use godot::prelude::*;
use godot::engine::{ISprite2D, Sprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct PlayerCharacter {
    #[export] speed: f64,
    #[export] angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for PlayerCharacter {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite
        }
    }

    fn ready(&mut self) {
        godot_print!("From ready");
    }

    fn process(&mut self, _delta: f64) {
        godot_print!("From process");

        let mut _velocity = Vector2::new(0.0, 0.0);
    }
}
