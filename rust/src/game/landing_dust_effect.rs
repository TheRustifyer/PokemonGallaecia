use gdnative::prelude::*;
use gdnative::api::AnimatedSprite;

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
#[derive(Debug)]
pub struct LandingDustEffect;

#[methods]
impl LandingDustEffect {
    pub fn new(_owner: &AnimatedSprite) -> Self { 
        Self { }
    }

    #[export]
    fn _ready(&self, owner: &AnimatedSprite) {
        owner.set_frame(0);
        owner.play("dust_effect", false);
    }

    #[export]
    fn _on_landing_dust_effect_animation_finished(&self, owner: &AnimatedSprite) {
        owner.queue_free()
    }

}