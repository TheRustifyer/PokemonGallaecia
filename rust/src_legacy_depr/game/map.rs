use gdnative::prelude::*;

// use crate::game::code_abstractions::signals::RegisterSignal;
#[derive(GodotClass)]
#[class(base=Node2D)]
#[derive(Debug)]
// #[register_with(Self::register_signal)]
pub struct Map;


#[methods]
impl Map {
    
    fn new(_owner: &Node2D) -> Self {       
        Self
    }
}