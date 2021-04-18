use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]

pub struct Map;

#[gdnative::methods]
impl Map {
    
    fn new(_owner: &Node2D) -> Self {       
        Self
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        
        let scene_tree_ref = 
        unsafe { Node::get_tree(owner)
        .unwrap().assume_safe() };
        godot_print!("Map SCENE: {:?}", scene_tree_ref);

    }
    
}