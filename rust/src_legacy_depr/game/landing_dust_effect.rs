use gdnative::prelude::*;
use gdnative::api::AnimatedSprite;

#[derive(GodotClass)]
#[class(base=AnimatedSprite)]
#[derive(Debug)]
pub struct LandingDustEffect;

#[methods]
impl LandingDustEffect {
    pub fn new(_owner: &AnimatedSprite) -> Self { 
        Self { }
    }

    
    fn _ready(&self, owner: &AnimatedSprite) {
        owner.set_frame(0);
        owner.play("dust_effect", false);
    }

    
    fn _on_landing_dust_effect_animation_finished(&self, owner: &AnimatedSprite) {
        owner.queue_free()
    }

}