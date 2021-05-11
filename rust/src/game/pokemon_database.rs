use gdnative::{api::EditorScript,  prelude::*};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
// #[derive(Tool)]
pub struct PokemonDB;

#[gdnative::methods]
impl PokemonDB {
    pub fn new(_owner: &Node2D) -> Self { Self {} }

    fn get_pokemon_class_as_resource() -> TRef<'static, Node> { 
        let pkm_resource = unsafe { ResourceLoader::godot_singleton()
            .load("res://godot/Game/Pokemon.gdns", "", false)
            .unwrap().assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .instance(0)
            .unwrap().assume_safe() };

        pkm_resource
    }
    #[export]
    fn _ready(&self, _owner: &Node2D) {
        let this_scene = unsafe { _owner.get_tree().unwrap().assume_safe() };
        godot_print!("Saludos desde una escena autocorrida en el editor! Scene: {:?}", unsafe { this_scene.root().unwrap().assume_safe().name() })
    }
}