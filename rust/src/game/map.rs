use gdnative::prelude::*;
use super::game_elements::signals::RegisterSignal;
#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
// #[register_with(Self::register_signal)]
pub struct Map;


#[gdnative::methods]
impl Map {
    
    fn new(_owner: &Node2D) -> Self {       
        Self
    }

    // #[export]
    // fn _ready(&mut self, owner: &Node2D) {
        
    //     let scene_tree_ref = 
    //     unsafe { Node::get_tree(owner)
    //     .unwrap().assume_safe() };
    //     godot_print!("Map SCENE: {:?}", scene_tree_ref);

    // }
    
}