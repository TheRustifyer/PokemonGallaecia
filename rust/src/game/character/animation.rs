use godot::{engine::{AnimatedSprite2D, IAnimatedSprite2D}, bind::{GodotClass, godot_api}, obj::Base, log::godot_print};

/// [`godot::bind::GodotClass`] for hold abstractions about animated sprites in 2D worlds.
/// 
/// This type main purpose is to hold the data tracked of any animated character in the game
/// along with its status and behaviour
#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct CharacterAnimation {
    #[base] pub base: Base<AnimatedSprite2D>
}

#[godot_api]
impl CharacterAnimation {
    pub fn play_anim(&mut self, animation_name: &str) {
        // TODO introduce enumerated type?
        self.base.set_animation(animation_name.into());
        self.base.play();
    }
}

#[godot_api]
impl IAnimatedSprite2D for CharacterAnimation {
    fn init(base: Base<AnimatedSprite2D>) -> Self {
        godot_print!("<CharacterAnimation> initialized");
        Self { base }
    }
}
