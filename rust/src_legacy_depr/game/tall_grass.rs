use godot::{bind::{GodotClass, godot_api}, engine::{AnimationPlayer, Sprite2D, Texture, INode2D, Node2D}, obj::{Gd, Base}, builtin::Signal};
use godot::private::class_macros::builder::ClassBuilder;

use crate::game::code_abstractions::signals::RegisterSignal;

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct TallGrass {
    // animation_player: Gd<AnimationPlayer>,
    // grass_overlay: Gd<Sprite2D>,
    // grass_overlay_texture: Gd<Texture>,
    // #[base] grass_overlay: Base<Sprite2D>, // ! sprite or node?
    #[base] grass_overlay: Base<Node2D>,
    animation_player: Gd<AnimationPlayer>,
    grass_overlay_texture: Gd<Texture>,
}

impl RegisterSignal<Self> for TallGrass {
    fn register_signal(_builder: &Signal) {
        _builder.signal( "").done();
    }
}

#[godot_api]
impl INode2D for TallGrass {
    fn init(sprite: Base<Node2D>) -> Self {

        Self {
            grass_overlay: sprite,
            animation_player: sprite.get_node("AnimationPlayer".into())
                .unwrap().cast::<AnimationPlayer>(),
            grass_overlay_texture: ResourceLoader::godot_singleton()
                .load("res://gfx/Tilemaps/Grass/stepped_tall_grass.png", "", false)
                .unwrap().assume_safe()
                .cast::<Texture>()
                .unwrap()
                .assume_shared(),
        }
        
        // Self {
        //     speed: 400.0,
        //     angular_speed: std::f64::consts::PI,
        //     sprite
        // }
    }
}


#[methods]
impl TallGrass {
    fn new(_owner: &Node2D) -> Self {
        Self {
            animation_player: None,
            grass_overlay: unsafe { Sprite::new().assume_shared().assume_safe() },
            grass_overlay_texture: None,
        }
    }

    
    fn _ready(&mut self, base: TRef<Node2D>) {

        self.animation_player = Some(unsafe { base.get_node("AnimationPlayer")
            .unwrap().assume_safe().cast::<AnimationPlayer>().unwrap() });

        self.grass_overlay_texture = Some(unsafe { ResourceLoader::godot_singleton()
            .load("res://gfx/Tilemaps/Grass/stepped_tall_grass.png", "", false)
            .unwrap().assume_safe()
            .cast::<Texture>()
            .unwrap()
            .assume_shared() });
    }

    
    /// Receives a signal when a body enteres the TallGrass (connected on the Godot GUI)
    fn _on_area2d_body_entered(&mut self, base: TRef<Node2D>, _body: Variant) {
        self.player_in_grass(base);
        self.animation_player.unwrap().play("Stepped", 0.0, 1.0, false);
    }

    
    // Receives a signal when a body leaves the TallGrass (connected on the Godot GUI)
    fn _on_area2d_body_exited(&mut self, base: &Node2D, _body: Variant) {
        if unsafe { self.grass_overlay.assume_shared().is_instance_sane() } {
            self.grass_overlay.queue_free();
            base.remove_child(self.grass_overlay);
        }  
    }

    
    fn player_in_grass(&mut self, base: TRef<Node2D>) {
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
            None => base.add_child(grass_step_effect, true),
            Some(_) => ()
        }
        
        match &self.grass_overlay.get_parent() {
            None => { 
                self.grass_overlay =  unsafe { Sprite::new().assume_shared().assume_safe() };
                self.grass_overlay.set_name("GrassOverlay");
                self.grass_overlay.set_texture(unsafe { self.grass_overlay_texture.as_ref().unwrap().assume_safe() });

                base.add_child(self.grass_overlay, true);
                base.move_child(self.grass_overlay, base.get_child_count() );
                self.grass_overlay.set("z_index", 2);
                self.grass_overlay.set_position(Vector2::new(8.0, 8.0));

                let player_node = unsafe { base.get_node("/root/Game/Player").expect("Bad route to Game/Player").assume_safe().cast::<Node2D>().unwrap() };
                player_node.set("z_index", 1);
            },

            Some(_x) => ()
        }
        // Just for debug purposes
        for children in 0..base.get_child_count() {
            godot_print!("Children NAME: {:?}", unsafe { base.get_child(children).unwrap().assume_safe().name() })
        }
    }
}