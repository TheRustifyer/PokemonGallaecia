use gdnative::prelude::*;
use gdnative::api::AnimatedSprite;

#[derive(GodotClass)]
#[class(base=AnimatedSprite)]
#[derive(Debug)]
pub struct GrassStepEffect;

#[methods]
impl GrassStepEffect {
    pub fn new(_owner: &AnimatedSprite) -> Self { 
        Self { }
    }

    
    fn _ready(&self, base: &AnimatedSprite) {
        base.set_frame(0);
        base.play("", false);
    }

    
    fn _on_grass_step_effect_animation_finished(&self, base: &AnimatedSprite) {
        base.queue_free()
    }

}