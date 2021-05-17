use gdnative::{api::{AnimationPlayer, NinePatchRect, TextureRect}, prelude::*};

use crate::game::code_abstractions::signals::RegisterSignal;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
pub struct TallGrass {
    animation_player: Option<TRef<'static, AnimationPlayer>>,
    grass_overlay: TRef<'static, Sprite>,
    grass_overlay_texture: Option<Ref<Texture>>,
}

impl RegisterSignal<Self> for TallGrass {
    fn register_signal(_builder: &ClassBuilder<Self>) {
        _builder.add_signal( Signal {
            name: "",
            args: &[],
        });
    }
}

#[gdnative::methods]
impl TallGrass {
    fn new(_owner: &Node2D) -> Self {
        Self {
            animation_player: None,
            grass_overlay: unsafe { Sprite::new().assume_shared().assume_safe() },
            grass_overlay_texture: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {

        self.animation_player = Some(unsafe { owner.get_node("AnimationPlayer")
            .unwrap().assume_safe().cast::<AnimationPlayer>().unwrap() });

        self.grass_overlay_texture = Some(unsafe { ResourceLoader::godot_singleton()
            .load("res://gfx/Tilemaps/Grass/stepped_tall_grass.png", "", false)
            .unwrap().assume_safe()
            .cast::<Texture>()
            .unwrap()
            .assume_shared() });
    }

    #[export]
    /// Receives a signal when a body enteres the TallGrass (connected on the Godot GUI)
    fn _on_area2d_body_entered(&mut self, owner: TRef<Node2D>, _body: Variant) {
        self.player_in_grass(owner);
        self.animation_player.unwrap().play("Stepped", 0.0, 1.0, false);
    }

    #[export]
    // Receives a signal when a body leaves the TallGrass (connected on the Godot GUI)
    fn _on_area2d_body_exited(&mut self, owner: &Node2D, _body: Variant) {
        if unsafe { self.grass_overlay.assume_shared().is_instance_sane() } {
            self.grass_overlay.queue_free();
            owner.remove_child(self.grass_overlay);
        }  
    }

    #[export]
    fn player_in_grass(&mut self, owner: TRef<Node2D>) {
        // Creates a new grass step effect
        let grass_step_effect = unsafe { ResourceLoader::godot_singleton()
            .load("res://godot/Game/GrassStepEffect.tscn", "", false)
            .unwrap().assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .instance(0)
            .unwrap().assume_safe()
            .cast::<Node2D>()
            .unwrap()
        };
        
        match grass_step_effect.get_parent() {
            None => owner.add_child(grass_step_effect, true),
            Some(_) => ()
        }
        
        match &self.grass_overlay.get_parent() {

            None => { self.grass_overlay =  unsafe { Sprite::new().assume_shared().assume_safe() };
                self.grass_overlay.set_name("GrassOverlay");
                self.grass_overlay.set_texture(unsafe { self.grass_overlay_texture.as_ref().unwrap().assume_safe() });

                owner.add_child(self.grass_overlay, true);
                owner.move_child(self.grass_overlay, owner.get_child_count() );
                self.grass_overlay.set("z_index", 2);
                self.grass_overlay.set_position(Vector2::new(8.0, 8.0));

                let player_node = unsafe { owner.get_node("/root/Game/Player").expect("Bad route to Game/Player").assume_safe().cast::<Node2D>().unwrap() };
                player_node.set("z_index", 1);
         
            },
            Some(_x) => ()
        }
        // Just for debug purposes
        for children in 0..owner.get_child_count() {
            godot_print!("Children NAME: {:?}", unsafe { owner.get_child(children).unwrap().assume_safe().name() })
        }
    }
}