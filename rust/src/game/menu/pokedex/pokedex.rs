use gdnative::prelude::*;
use gdnative::api::{CanvasItem, NinePatchRect, PackedScene};

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

        
        let pokedex_item_as_resource = ResourceLoader::godot_singleton()
            .load("res://godot/Game/PokedexItem.tscn", "", false)
            .unwrap();


        let pokedex_holder = unsafe { _owner.get_node("PokedexItems")
            .unwrap()
            .assume_safe()
        };

        let mut x = 300.0;
        let mut y = 300.0;        
        
        for pokemon in pokemon_list.iter().enumerate() {

            let pokecounter = pokemon.0 + 1;

            let node = unsafe { 
                pokedex_item_as_resource
                .assume_safe()
                .cast::<PackedScene>()
                .unwrap()
                .instance(0)
                .unwrap()
                .assume_safe()
                .cast::<NinePatchRect>()
                .unwrap()
            };

            node.set_global_position(
                Vector2::new(x, y), 
                false);

            let pokemon_number_label = unsafe { 
                node.get_node("PokemonNumber")
                .unwrap()
                .assume_safe()
                .cast::<Label>()
                .unwrap()
            };

            let pokemon_name_label = unsafe { 
                node.get_node("PokemonName")
                .unwrap()
                .assume_safe()
                .cast::<Label>()
                .unwrap()
            };

            pokemon_number_label.set_text(pokecounter.to_string());
            pokemon_name_label.set_text(pokemon.1.to_string());
            // text_label.set_global_position(Vector2::new(x + 20.0, y + 20.0), false);
            // text_label.set_anchor(3, 0.0, false, true);
            // text_label.set_align(0);
            pokemon_number_label.add_color_override("font_color", Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 });
            pokemon_name_label.add_color_override("font_color", Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 });

            pokedex_holder.add_child(node, true);

            // Updates the NEXT NinePatchRect Pokedex entry Y position
            y += 150.0;


        }

        godot_print!("Total childs appended: {:?}", pokedex_holder.get_child_count());
        godot_print!("Childs: {:?}", pokedex_holder.get_children())

    }
}

/*

        // let pokedex_items_node = unsafe { Node::get_tree(_owner).unwrap()
        //     .assume_safe().root()
        //     .unwrap().assume_safe()
        //     .get_node("Game/Player/Camera2D/Pokedex/PokedexItems")
        //     .unwrap().assume_safe()
        //     };
let poke_item = unsafe { _owner.get_node("PokedexItems")
                .expect("No node PokedexItems").assume_safe() };
let my_patch9 = NinePatchRect::new();
            let pokelabel = Label::new();

            my_patch9.add_child(pokelabel, true);
            
            _owner.set_global_position(Vector2::new(x, y), false) ;
            
            poke_item.add_child(my_patch9, true);
*/