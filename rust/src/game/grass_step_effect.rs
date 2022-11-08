use gdnative::prelude::*;
use gdnative::api::AnimatedSprite;

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
#[derive(Debug)]
pub struct GrassStepEffect;

#[methods]
impl GrassStepEffect {
    pub fn new(_owner: &AnimatedSprite) -> Self { 
        Self { }
    }

    #[method]
    fn _ready(&self, #[base] base: &AnimatedSprite) {
        base.set_frame(0);
        base.play("", false);
    }

    #[method]
    fn _on_grass_step_effect_animation_finished(&self, #[base] base: &AnimatedSprite) {
        base.queue_free()
    }

}