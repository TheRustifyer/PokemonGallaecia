use gdnative::prelude::*;

use super::pokemon_specie::PokemonSpecie;
use crate::utils::all_pokemon_species::all_pokemon_species_data;

/// Acts as a wrapper over some type T, representing a DB Row of elements of that T type
pub struct DBRow<'a, T> {
    pub object: &'a T,
}

impl<'a, T> DBRow<'a, T> {
    fn new(object: &'a T) -> Self {
        Self {
            object: object
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
/// Using Nodes as tables, as more nodes as rows, creates a simulation of a real database by using nodes as the main objects.
pub struct PokemonDB {
    // TRefs to the Nodes that simulates real DB tables
    pokemon_table: TRef<'static, Node2D>,
    moves_table: TRef<'static, Node2D>,
    items_table: TRef<'static, Node2D>,
}

#[methods]
impl PokemonDB {
    pub fn new(_owner: &Node2D) -> Self { 
        Self {
            pokemon_table: unsafe { Node2D::new().assume_shared().assume_safe() },
            moves_table: unsafe { Node2D::new().assume_shared().assume_safe() },
            items_table: unsafe { Node2D::new().assume_shared().assume_safe() },
        } 
    }
    /// An static way to access the PokemonSpecie struct as a Godot Node
    fn get_pokemon_class_as_resource() -> TRef<'static, Node> { 
        let pkm_resource = unsafe { ResourceLoader::godot_singleton()
            .load("res://godot/Game/PokemonSpecieNode.tscn", "", false)
            .unwrap().assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .instance(0)
            .unwrap().assume_safe() };

        pkm_resource
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        //Load db, create it's structure and checks it's integrity
        self.create_database_structure(owner);
    }

    fn create_database_structure(&mut self, owner: &Node2D) {
        // Pokémon
        self.pokemon_table = self.create_table(owner, "Pokemons");
        self.populate_pokemon_table(owner);
        // Moves
        self.moves_table = self.create_table(owner, "Moves");
        // Game Items
        self.items_table = self.create_table(owner, "Items");
    }

    /// Creates a new node that simulates a real database table, by creating that node as a container for another nodes.
    fn create_table(&self, owner: &Node2D, table_name: &str) -> TRef<'static, Node2D> {
        let new_table = unsafe { Node2D::new().assume_shared().assume_safe() };
        // Add the new table as a child of the DB
        owner.add_child(new_table, true);
        // Gives a real nable to the Node
        new_table.set_name(table_name);
        // A TRef to the new created Node is returned
        new_table
    }

    /// Method that retrieves all the data of all the Pokémon Species availiable in the game.
    /// 
    /// Gets the data from an external (to this crate) function, where all the values are handcoded (say tnx for the work... bit..)
    /// which return a Vec<PokemonSpecie>. For each one, creates a new node with the instance data.
    fn populate_pokemon_table(&mut self, owner: &Node2D) {
        for pokemon_specie in all_pokemon_species_data().iter() {
            let pokemon_as_row = DBRow::<PokemonSpecie>::new(
                pokemon_specie
                );
            self.create_pokemon_row(owner, pokemon_as_row)
        }
    }

    /// Creates a new NODE2D representing a row inside a DB table
    fn create_pokemon_row(&self, _owner: &Node2D, row: DBRow<PokemonSpecie>) {
        // Creates a new node to hold the PokémonSpecies instances data
        let new_row = PokemonDB::get_pokemon_class_as_resource();
        
        // Add that node as a child of the desingned table
        self.pokemon_table.add_child(new_row, true);
        
        // Sets the Node's name == Pokemon's Specie name
        new_row.set_name(&row.object.name);
        
        // Sets the values of the node that holds the Pokémon Species attributes
        new_row.set("name", &row.object.name);
        new_row.set("id", &row.object.id);
        // godot_print!("Pokemon name: {:?}", new_row.get("name").to_string());
    }
}