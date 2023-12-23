//! The [`player.rs`] file
//! 
//! Holds the type that represents the player controlled character in game
//! Holds the `gdext` bindings exposed to `Godot 4` 

use godot::prelude::*;
use godot::engine::{ISprite2D, Sprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct PlayerCharacter {
    // #[export] asset: Gd<Sprite2D>

    #[base]
    sprite: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for PlayerCharacter {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Player initialized");
        
        Self {
            sprite
        }
    }

    fn ready(&mut self) {
        godot_print!("Player ready");
        self.sprite.set_texture(
            load("res://gfx/character/character.png")
        );
    }

    fn process(&mut self, _delta: f64) {
        // godot_print!("From process");

        // let mut _velocity = Vector2::new(0.0, 0.0);
    }
}
