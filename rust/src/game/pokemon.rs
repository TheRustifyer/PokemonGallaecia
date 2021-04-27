/// The base class for a Pokemon data representation and data manipulation

pub struct Pokemon {
    pokedex_number: i32,
    name: String,
    type1: String,
    type2: String,
    height: f64,
    tall: f64,
    // description: String,
}

impl Pokemon {
    pub fn new(
        pokedex_number: i32, 
        name: String, 
        type1: String,
        type2: String,
        height: f64, 
        tall: f64, 
        // description: String
    ) -> Self { 
            Self { 
                pokedex_number, 
                type1,
                type2,
                name, 
                height, 
                tall, 
                // description
                } 
            }
}