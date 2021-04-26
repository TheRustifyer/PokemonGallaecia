use gdnative::prelude::*;
use gdnative::api::{CanvasItem, NinePatchRect};

// use crate::game::code_abstractions::signals::RegisterSignal;
// use crate::game::code_abstractions::node_operations::NodeReferences;

// use crate::utils::utils;

#[derive(NativeClass)]
#[inherit(Control)]
// #[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct Pokedex;

#[gdnative::methods]
impl Pokedex {
    fn new(_owner: &Control) -> Self {
        Self
    }

    #[export]
    fn _ready(&mut self, _owner: &Control) {
        godot_print!("Pokédex working");

        let pokemon_list = VariantArray::new();

        let bulbasaur = GodotString::from_str("Bulbasaur");
        let charmander = GodotString::from_str("Charmander");
        let squirtle = GodotString::from_str("Squirtle");
        let pikachu = GodotString::from_str("Pikachu");
        let mewtwo = GodotString::from_str("Mewtwo");

        let pokemon_vec = vec![bulbasaur, charmander, squirtle, pikachu, mewtwo];

        for pokemon in pokemon_vec.iter() {
            pokemon_list.push(pokemon);
        }

        
        // let pokedex_item_as_resource = unsafe { ResourceLoader::godot_singleton()
        //     .load("res://godot/Game/PokedexItem.tscn", "", false)
        //     .unwrap()
        //     .assume_safe()
        //     .cast_instance::<Node2D>()
        // };

        // let pokedex_holder = unsafe { _owner.get_node("PokedexItems")
        //     .unwrap()
        //     .assume_safe()
        // };

        // let pokedex_items_node = unsafe { Node::get_tree(_owner).unwrap()
        //     .assume_safe().root()
        //     .unwrap().assume_safe()
        //     .get_node("Game/Player/Camera2D/Pokedex/PokedexItems")
        //     .unwrap().assume_safe()
        //     };

        let x = 300.0;
        let y = 300.0;

        
        // let childs: Vec<i32> = Vec::new();

        let poke_item = unsafe { _owner.get_node("PokedexItems")
                .expect("No node PokedexItems").assume_safe() };
        
        for pokemon in pokemon_list.iter() {
            // let pokedex_item = unsafe { 
            //     pokedex_item_as_resource.assume_safe()
            //  };

            
            let my_patch9 = NinePatchRect::new();
            let pokelabel = Label::new();

            my_patch9.add_child(pokelabel, true);
            
            _owner.set_global_position(Vector2::new(x, y), false) ;
            

            poke_item.add_child(my_patch9, true);

        }

        godot_print!("Total childs appended: {:?}", poke_item.get_child_count());
        godot_print!("Childs: {:?}", poke_item.get_children())

    }
}