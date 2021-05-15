use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
/// The base class for a Pokemon data representation and data manipulation, holded as a Godot Node using another node as parent and as a DB simulator
pub struct PokemonSpecie {
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
    pokedex_description: String,
}

#[gdnative::methods]
impl PokemonSpecie {
    pub fn new(_owner: &Node2D) -> Self { 
            Self { 
                id: 0, 
                name: "".to_string(),
                type1: "".to_string(),
                type2: "".to_string(),
                height: 0.0, 
                tall: 0.0, 
                pokedex_description: "".to_string(),
                } 
            }

    pub fn new_pokemon(id: i32, name: &str, type1: &str, type2: &str, height: f64, tall: f64,
        pokedex_description: &str
        ) -> Self {
        Self {
            id: id,
            name: name.to_string(),
            type1: type1.to_string(),
            type2: type2.to_string(),
            height: height,
            tall: tall,
            pokedex_description: pokedex_description.to_string(),
        }
    }
}