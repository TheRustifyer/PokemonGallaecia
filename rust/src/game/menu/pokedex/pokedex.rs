use gdnative::prelude::*;
use gdnative::api::{NinePatchRect, PackedScene, Resource};

use crate::utils::utils;

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

    current_pokedex_entry_selected: i32,

    times_pressed: f64,

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
            current_pokedex_entry_selected: 1, 
            times_pressed: 0.0,
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

        // Enable processing
        _owner.set_process(true);

    }

    #[export]
    fn _process(&mut self, owner: &Control, delta: f64) {
        self.handle_pokedex_input_events(owner, delta);
    }

    fn handle_pokedex_input_events(&mut self, owner: &Control, delta: f64) {
        // Gets an input singleton to point to the input events
        let input: &Input = Input::godot_singleton();

        // Set a variable that let us to directly access the PokédexHolderOfEntries Node methods
        let pokedex_entry_node = unsafe {
            self.pokedex_items_holder_node
            .unwrap()
            .assume_safe()
            .cast::<Node2D>()
            .unwrap() 
        };

        // Moves the PokédexEntries all along the screen, acting as an scrollable
        if Input::is_action_pressed(&input, "ui_up") || Input::is_action_pressed(&input, "ui_down") {
            self.times_pressed += delta;
            if self.times_pressed > 0.3 {
                if Input::is_action_pressed(&input, "ui_up") {
                    match self.current_pokedex_entry_selected {
                        x if x <= 1 => (),
                        _ => { self.current_pokedex_entry_selected -= 1;
                                    pokedex_entry_node.set_global_position(
                                        pokedex_entry_node.global_position() + 
                                        Vector2::new(0.0, 150.0)
                                    ) 
                            }
                    }
                }
                else if Input::is_action_pressed(&input, "ui_down") {
                        godot_print!("Delta from AP: {}", &delta);
                    match self.current_pokedex_entry_selected {
                        x if x < 151 => { 
                            self.current_pokedex_entry_selected += 1;
                            pokedex_entry_node.set_global_position(
                                pokedex_entry_node.global_position() - 
                                Vector2::new(0.0, 150.0)
                            )
                        }
                        _ => ()
                    }
                } 
            } else {
                if Input::is_action_just_pressed(&input, "ui_up") {
                    match self.current_pokedex_entry_selected {
                        x if x <= 1 => (),
                        _ => { self.current_pokedex_entry_selected -= 1;
                                    pokedex_entry_node.set_global_position(
                                        pokedex_entry_node.global_position() + 
                                        Vector2::new(0.0, 150.0)
                                    ) 
                            }
                    }
                }
                else if Input::is_action_just_pressed(&input, "ui_down") {
                        godot_print!("Delta from AJP: {}", &delta);
                    match self.current_pokedex_entry_selected {
                        x if x < 151 => { 
                            self.current_pokedex_entry_selected += 1;
                            pokedex_entry_node.set_global_position(
                                pokedex_entry_node.global_position() - 
                                Vector2::new(0.0, 150.0)
                            )
                        }
                        _ => ()
                    }
                }
            }
            godot_print!("self.times_pressed: {}", &self.times_pressed);
        } else if Input::is_action_just_released(&input, "ui_up") || Input::is_action_just_released(&input, "ui_down") {
            self.times_pressed = 0.0;
        }

        
        if Input::is_action_just_pressed(&input, "Exit") {
            utils::change_scene(owner, "res://godot/Game/Game.tscn".to_string())
        }
    }

    /// Method than when gets called, iterates all over a pre-designed list of all Pokémons, creating as much Pokédex entries as Pokémons are in the game
    fn init_pokedex(&mut self) {

        // Fills the attribute (Vec<PokedexEntries>) that holds all the PokedexEntries availiables
        self.availiable_pokemon_list();
        // For each Pokémon inside that attribute Pokémon's data...
        // let mut pkm;
        for (pokecounter, pokemon) in self.pokedex_entries.iter().enumerate() {
            // Still not to much. An EZ way to tracks and dynamically creates the Pokédex index of a Pokémon
            // pkm = pokemon.clone();
            self.create_new_pokedex_entry(&pokemon, pokecounter as i32);
            // Don't forget to updates the Y coordinate that will be passed in to the future (on NEXT iteration 'till exhaust) instance of the NinePatchRect PokedexEntry 
            self.y_entry_position += 150.0;
        }
    }

    /// Method 
    ///
    /// Quite large and complicated? method that in the end just creates a new PokedexEntry.
    fn create_new_pokedex_entry(&self, pokemon: &PokedexEntry, _pokecounter: i32) {
        
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

        let pokeball_sprite = unsafe { 
            pokedex_item_box.get_node("Captured")
            .unwrap()
            .assume_safe()
            .cast::<Sprite>()
            .unwrap()
        };

        // Now that we got the references to those crazy Pokedata labels, we set it's text passing the entries data
        if pokemon.captured_by_player == true && pokemon.spotted_by_player == true {
            pokemon_number_label.set_text("N.º".to_owned() + &pokemon.pokedex_entry_number.to_string());
            pokemon_name_label.set_text(&pokemon.name);
        } else if !pokemon.captured_by_player && pokemon.spotted_by_player {
            pokemon_number_label.set_text("N.º".to_owned() + &pokemon.pokedex_entry_number.to_string());
            pokemon_name_label.set_text(&pokemon.name);
            pokeball_sprite.set_visible(false);
        } else {
            pokemon_number_label.set_text("N.º".to_owned() + &pokemon.pokedex_entry_number.to_string());
            pokemon_name_label.set_text("?????");
            pokeball_sprite.set_visible(false);
        }
            

        // Well, well be fine be able to dynamically modify the color of the text depending of the properties of the Pokemon variety
        // and rarity, isn't it? ;)
        pokemon_number_label.add_color_override("font_color", Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 });
        pokemon_name_label.add_color_override("font_color", Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 });
        
        // Finally, we add a new child (the new instance of the data container with his labels) representing the desired
        // PokédexEntry!
        unsafe { 
            self.pokedex_items_holder_node.unwrap().assume_safe().add_child(pokedex_item_box, true);
            // Debug info
            // godot_print!("Total childs appended: {:?}", &self.pokedex_items_holder_node.unwrap().assume_safe().get_child_count());
            // godot_print!("Childs: {:?}", &self.pokedex_items_holder_node.unwrap().assume_safe().get_children()) 
        };


    }

    fn availiable_pokemon_list(&mut self) {
        
        let bulbasaur = PokedexEntry::new(
            001, 
            "Bulbasaur".to_string(), 
            "Planta".to_string(), 
            "Veneno".to_string(), 
            0.8,
            1.5,
            "Cacho de tipo planta".to_string(), 
            true, 
            true,
        );

        let charmander = PokedexEntry::new(
            004, 
            "Charmander".to_string(), 
            "Fuego".to_string(), 
            "".to_string(), 
            0.8,
            1.5,
            "Escupo fuego".to_string(), 
            true, 
            false,
        );

        let squirtle = PokedexEntry::new(
            007, 
            "Squirtle".to_string(), 
            "Agua".to_string(), 
            "".to_string(), 
            0.8,
            1.5,
            "Amo a calmarno".to_string(), 
            false, 
            false,
        );

        // let mut pokemon_vec = vec![];

        for number in 1..=151 {
            if number == 1 {
                self.pokedex_entries.push(bulbasaur.clone())
            } else if number == 4 {
                self.pokedex_entries.push(charmander.clone())
            } else if number == 7 {
                self.pokedex_entries.push(squirtle.clone())
            } else {
                let pokemon: PokedexEntry = PokedexEntry::new(
                    number, 
                    "".to_string(), 
                    "".to_string(), 
                    "".to_string(), 
                    0.0,
                    0.0,
                    "".to_string(), 
                    false, 
                    false,
                );
                self.pokedex_entries.push(pokemon)
            }  
        }
    }
}

/// Struct that representes every Pokédex entry with the correlative Pokémon 
#[derive(Clone, Debug)]
pub struct PokedexEntry {
    pokedex_entry_number: i32,
    name: String,
    type1: String,
    type2: String,
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
        type1: String,
        type2: String,
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
                type1,
                type2,
                height,
                weight,
                description,
                // scream,
                spotted_by_player,
                captured_by_player,
            }
        }
}