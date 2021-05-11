use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
/// The base class for a Pokemon data representation and data manipulation
pub struct Pokemon {
    id: i32,
    name: String,
    type1: String,
    type2: String,
    height: f64,
    tall: f64,
    // description: String,
}

#[gdnative::methods]
impl Pokemon {
    pub fn new(_owner: &Node2D) -> Self { 
            Self { 
                id: 0, 
                name: "".to_string(),
                type1: "".to_string(),
                type2: "".to_string(),
                height: 0.0, 
                tall: 0.0, 
                // description
                } 
            }
}