use gdnative::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
#[derive(Debug)]
/// The base class for a Pokemon data representation and data manipulation
pub struct Pokemon {
    #[property]
    pub id: i32,
    #[property]
    pub name: String,
    #[property]
    pub type1: String,
    #[property]
    pub type2: String,
    #[property]
    pub height: f64,
    #[property(default = 40.0)]
    pub tall: f64,
    // description: String,
}

#[methods]
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

    pub fn new_pokemon(id: i32, name: String, type1: String, type2: String, height: f64, tall: f64) -> Self {
        Self {
            id: id,
            name: name,
            type1: type1,
            type2: type2,
            height: height,
            tall: tall
        }
    }
}