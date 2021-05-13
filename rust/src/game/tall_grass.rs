use gdnative::{api::{AnimationPlayer, TextureRect}, prelude::*};

use crate::game::code_abstractions::signals::RegisterSignal;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
pub struct TallGrass {
    animation_player: Option<TRef<'static, AnimationPlayer>>,
    grass_overlay: TRef<'static, TextureRect>,
    grass_overlay_texture: Option<Ref<Texture>>,
    // grass_step_effect: TRef<'static, Node2D>,
    player_inside_tallgrass: bool,
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
            grass_overlay: unsafe { TextureRect::new().assume_shared().assume_safe() },
            grass_overlay_texture: None,
            // grass_step_effect: 
            player_inside_tallgrass: false,
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

        godot_print!("Grass Overlay: {:?}", &self.grass_overlay_texture);
// 
        // self.connect_to_player_moving(owner);
        // self.connect_to_player_stopped(owner);
    }

    #[export]
    // Receives the on_area2d_body_entered signal, connected on the Godot GUI
    fn _on_area2d_body_entered(&mut self, owner: TRef<Node2D>, _body: Variant) {
        godot_print!("Signal received!");
        self.player_inside_tallgrass = true;
        self.player_in_grass(owner);
        self.animation_player.unwrap().play("Stepped", 0.0, 1.0, false)
    }

    #[export]
    /// Connects the game data signal with the Game Node
    fn player_exitting_grass(&mut self, owner: TRef<Node2D>) {
        self.player_inside_tallgrass = false;
        if unsafe { self.grass_overlay.assume_shared().is_instance_sane() } {
            self.grass_overlay.queue_free();
        }
        godot_print!("Player IN grass");
    }

    #[export]
    /// Connects the game data signal with the Game Node
    fn player_in_grass(&mut self, owner: TRef<Node2D>) {
        // Grass step effect
        let grass_step_effect = unsafe { ResourceLoader::godot_singleton()
            .load("res://godot/Game/GrassStepEffect.tscn", "", false)
            .unwrap().assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .instance(0)
            .unwrap().assume_safe()
            // .cast::<Node2D>()
            // .unwrap()
        };
        // grass_step_effect.set_position(owner.position());
        owner.add_child(grass_step_effect, true);


        self.grass_overlay.set_texture(unsafe { self.grass_overlay_texture.as_ref().unwrap().assume_safe() });
        self.grass_overlay.set_position(owner.position(), false);
        
        // unsafe { self.grass_overlay.get_parent().expect("No tiene parent").assume_safe().remove_child(self.grass_overlay) };
        // owner.add_child(self.grass_overlay, true);
    }

    // <-------------- METHODS TO REVERSE CONNECT SIGNALS FROM PLAYER TO HERE ----------------------->

    #[export]
    /// Connects the game data signal with the Game Node
    fn connect_to_player_moving(&self, owner: TRef<Node2D>) {
        let player = unsafe { owner.get_node("/root/Game/Player").unwrap().assume_safe() };
        player.connect("player_moving", owner, "player_exitting_grass",
            VariantArray::new_shared(), 0).unwrap();
    }

    #[export]
    /// Connects the game data signal with the Game Node
    fn connect_to_player_stopped(&self, owner: TRef<Node2D>) {
        let player = unsafe { owner.get_node("/root/Game/Player").unwrap().assume_safe() };
        player.connect("player_moving", owner, "player_in_grass",
            VariantArray::new_shared(), 0).unwrap();
    }

}