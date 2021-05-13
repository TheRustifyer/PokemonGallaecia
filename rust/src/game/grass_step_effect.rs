use gdnative::prelude::*;
use gdnative::api::AnimatedSprite;

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
#[derive(Debug)]
pub struct GrassStepEffect;

#[gdnative::methods]
impl GrassStepEffect {
    pub fn new(_owner: &AnimatedSprite) -> Self { 
        Self { }
    }

    #[export]
    fn _ready(&self, owner: &AnimatedSprite) {
        owner.set_frame(0);
        owner.play("", false);
    }

    #[export]
    fn _on_grass_step_effect_animation_finished(&self, owner: &AnimatedSprite) {
        owner.queue_free()
    }

}