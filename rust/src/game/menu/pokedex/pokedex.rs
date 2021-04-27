use gdnative::prelude::*;
use gdnative::api::{NinePatchRect, PackedScene, Resource};

// use crate::game::pokemon::Pokemon;

// use crate::game::code_abstractions::signals::RegisterSignal;
// use crate::game::code_abstractions::node_operations::NodeReferences;

// use crate::utils::utils;

#[derive(NativeClass)]
#[inherit(Control)]
// #[register_with(Self::register_signal)]
#[derive(Debug)]
/// The code representation on the `LEGENDARY` Pokémon's Pokédex
pub struct Pokedex {
    pokedex_entries: Vec<PokedexEntry>,
    
    pokedex_items_holder_node: Option<Ref<Node>>,
    pokedex_item_scene_resource: Option<Ref<Resource>>,

    x_entry_position: f64,
    y_entry_position: f64,

}

#[gdnative::methods]
impl Pokedex {
    fn new(_owner: &Control) -> Self {
        Self {
            pokedex_entries: Vec::<PokedexEntry>::new(),
            pokedex_items_holder_node: None,
            pokedex_item_scene_resource: None,
            x_entry_position: 300.0,
            y_entry_position: 300.0,  
        }
    }

    
    #[export]
    fn _ready(&mut self, _owner: &Control) {
        // First of all, we need to initialize our references out of the Struct constructor
        
        // This one get get Pokedex Item scene as a resource, to later transform it into a TRef<NinePatchRect> that holds the labels
        // with the Pokémon data 
        self.pokedex_item_scene_resource = ResourceLoader::godot_singleton()
            .load("res://godot/Game/PokedexItem.tscn", "", false);
        
        // And this pretty one is a reference to a child node of the Pokédex that will get as many childs as Pokémon are in the game
        self.pokedex_items_holder_node = _owner.get_node("PokedexItems");

        // Currently just makes as much entries as availiable Pokémons are.
        self.init_pokedex();

    }

    /// Method than when gets called, iterates all over a pre-designed list of all Pokémons, creating as much Pokédex entries as Pokémons are in the game
    fn init_pokedex(&mut self) {
        // For each Pokémon inside a future list that will be made of dynamically retrieved Pokémon's data...
        for (pokecounter, pokemon) in self.availiable_pokemon_list().iter().enumerate() {
            // Still not to much. An EZ way to tracks and dynamically creates the Pokédex index of a Pokémon
            let pokecounter = pokecounter + 1;

            self.create_new_pokedex_entry(pokemon, pokecounter);
        }
    }

    /// Method 
    ///
    /// Quite large and complicated? method that in the end just creates a new PokedexEntry.
    fn create_new_pokedex_entry(&mut self, pokemon: &GodotString, pokecounter: usize) {
        
        // This marvelous one is the responsable of instanciate a new NinePatchRect, that will contains labels
        // with the Pokédex data of every Pokémon availiable in the game, acting as a kind of graphical container.
        let pokedex_item_box = unsafe { 
            self.pokedex_item_scene_resource
            .to_owned()
            .unwrap()
            .assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .instance(0) //*! <-----  Here is where the magic happens
            .unwrap()
            .assume_safe()
            .cast::<NinePatchRect>()
            .unwrap()
        };

        // Given a position of every new box, place it on screen with the previous modificated `y` coord
        pokedex_item_box.set_global_position(
            Vector2::new(self.x_entry_position as f32, self.y_entry_position as f32), 
            false);

        // For every new instance, we should get the new labels inside of the instace and pass it the info
        // NOTE: Those labels are placed as a child inside the Godot's graphical interface. A mode coherente desing w'd be probably
        // create a new labels resource and add it a a child of the box... For today will remains as a todo!
        let pokemon_number_label = unsafe { 
            pokedex_item_box.get_node("PokemonNumber")
            .unwrap()
            .assume_safe()
            .cast::<Label>()
            .unwrap()
        };

        let pokemon_name_label = unsafe { 
            pokedex_item_box.get_node("PokemonName")
            .unwrap()
            .assume_safe()
            .cast::<Label>()
            .unwrap()
        };

        // Now that we got the references to those crazy Pokedata labels, we set it's text passing the entries data
        pokemon_number_label.set_text(pokecounter.to_string());
        pokemon_name_label.set_text(pokemon.to_string());

        // Well, well be fine be able to dynamically modify the color of the text depending of the properties of the Pokemon variety
        // and rarity, isn't it? ;)
        pokemon_number_label.add_color_override("font_color", Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 });
        pokemon_name_label.add_color_override("font_color", Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 });
        
        // Finally, we add a new child (the new instance of the data container with his labels) representing the desired
        // PokédexEntry!
        unsafe { 
            self.pokedex_items_holder_node.unwrap().assume_safe().add_child(pokedex_item_box, true);
            // Debug info
            godot_print!("Total childs appended: {:?}", &self.pokedex_items_holder_node.unwrap().assume_safe().get_child_count());
            godot_print!("Childs: {:?}", &self.pokedex_items_holder_node.unwrap().assume_safe().get_children()) 
        };

        // Don't forget to updates the Y coordinate that will be passed in of the future (on NEXT iteration 'till exhaust) instance of the NinePatchRect PokedexEntry 
        self.y_entry_position += 150.0;
    }

    fn availiable_pokemon_list(&self) -> Vec<GodotString> {
        
        // Now it's a hardcoded version of Pokémon instances with just &str's! Be patient my padawans... or trainers!! :)
        
        let bulbasaur = GodotString::from_str("Bulbasaur");
        let charmander = GodotString::from_str("Charmander");
        let squirtle = GodotString::from_str("Squirtle");
        let pikachu = GodotString::from_str("Pikachu");
        let mewtwo = GodotString::from_str("Mewtwo");

        let pokemon_vec = vec![bulbasaur, charmander, squirtle, pikachu, mewtwo];

        return pokemon_vec

        // Meh, may the Pokédex be with you.
    }
}

/// Struct that representes every Pokédex entry with the correlative Pokémon 
#[derive(Debug)]
pub struct PokedexEntry {
    pokedex_entry_number: i32,
    name: String,
    height: f64,
    weight: f64,
    description: String,
    // scream: AudioStreamSample,
    
    //*! Player related attributes
    spotted_by_player: bool,
    captured_by_player: bool,
}

impl PokedexEntry {
    pub fn new(
        pokedex_entry_number: i32,
        name: String,
        height: f64,
        weight: f64,
        description: String,
        // scream: AudioStreamSample,
        spotted_by_player: bool,
        captured_by_player: bool
    ) -> Self {
            Self {
                pokedex_entry_number,
                name,
                height,
                weight,
                description,
                // scream,
                spotted_by_player,
                captured_by_player
            }
        }
}