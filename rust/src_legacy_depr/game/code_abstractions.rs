/// A base blueprint that defines the behaviour of the game characters.
///
/// The most basic abstraction of the game core, a character.
/// Character must represent any Kinematic2D Body that it's suppossed to be a human representation.
pub mod character {
    pub trait CharacterTileMovement<O, I> {
        fn process_player_input(&mut self, owner: &O, input: &I);

        fn tilemove_or_collide(&mut self, owner: &O, delta: f32);
        
        fn move_character(&mut self, _owner: &O, delta: f32);
    }

    // Supertrait. Child of CharacterTileMovement. A Pokémon jump it's usually 2 tiles
    pub trait CharacterJump<O, I>: CharacterTileMovement<O, I> {
        fn jump_over_ledge(&mut self, owner: &O, delta: f32);

        fn landing_dust_effect(&mut self, owner: &O);
    }
}


pub mod signals {
    use gdnative::{api::viewport::Usage, prelude::*};
    #[derive(Debug)]
    pub struct GodotSignal<'l> {
        name: &'l str,
        args: (&'l str, Variant, ExportInfo, Usage)
    }

    /// **Signal** -> Zero cost abstraction for handling the `Godot signals` in a custom approach
    /// Method for register a new signal to a designed class. You can find on the GUI Godot
    /// that signal registered on the Node panel on the same way if the signal was created directly on the GUI.
    /// The name of the method is completly arbitrary, is just a way to encapsulate the info passed to the builder object and transport it back to Godot.
    ///
    /// Remember to implement the `#[register_with(Self::N)]` attribute where `N` is the name of the method that is registering the signal
    pub trait RegisterSignal<T> {
        /// Registers a signal on `Godot`directly from the Rust code.
        fn register_signal(_builder: &ClassBuilder<T>) -> ();
    }
}

pub mod node_operations {

    use gdnative::prelude::*;
    
    pub trait NodeReferences<T> {
        /// Given a reference to _owner<T>, (&owner) and a path **from the root** to a Node, brings back a 
        /// Ref<T> to the desired node.
        fn get_node_reference_from_root(&mut self, owner: &T, path: &str) -> Option<Ref<Node>> ;
    }
}

pub mod database {
    use gdnative::prelude::*;

    /// Database on this game are Godot Nodes representing a classic relational DB structure
    ///
    /// This trait provides the methods to operate with and over the game database.
    /// This game obviously needs a Database system, so it's fine to coerce it to make sure that implements it
    pub trait Database {
        // The database will be treated as a static resource
        fn get_database_as_resource() -> TRef<'static, Node> { 
            let db_resource = unsafe { ResourceLoader::godot_singleton()
                .load("res://godot/Game/PokeDB.tscn", "", false)
                .unwrap().assume_safe()
                .cast::<PackedScene>()
                .unwrap()
                .instance(0)
                .unwrap().assume_safe() };

            db_resource
        }

        fn debug_database_info(&self, database: TRef<Node>) {
            for num in 0..database.get_child_count() {
                godot_print!("Database Tables {:?}", unsafe { database.get_child(num).unwrap().assume_safe().name() })
            }
            let pokemon_table = unsafe { database.get_child(0).unwrap().assume_safe() };
            for num in 0..pokemon_table.get_child_count() {
                godot_print!("Pokémon row NODE name: {:?}", unsafe { pokemon_table.get_child(num).unwrap().assume_safe().name() });
                godot_print!("Pokémon ID: {:?}", unsafe { pokemon_table.get_child(num).unwrap().assume_safe().get("id").to::<i64>() })
            }
        }
    }
}

pub mod dialogue_connections {

    use gdnative::prelude::*;
    /// Trait with the methods that allows a Node to print text on the Pokémon Dialogue Box and to spawn interactive dialogues based on elections
    ///
    /// This traits provides a default method implementation, allowing to reduce the amount of code necessary for the hundreds of Nodes with interactions,
    /// and every node with a text to print should implements this methods
    pub trait DialogueBoxActions {

        /// Method designed for connect the player with an element with an interaction
        /// 
        /// When player collides with an object (Node) that has an interaction, player emits a signal that triggers the
        /// emit object signal
        fn connect_to_player(&self, _owner: TRef<Sprite>) {
            let player_signal = unsafe { Node::get_tree(&_owner).unwrap()
                .assume_safe().root()
                .unwrap().assume_safe()
                .get_node("Game/Player")
                .unwrap().assume_safe() };

                player_signal.connect("player_interacting", _owner, 
                "emit_object_signal", VariantArray::new_shared(), 0).unwrap();
        }

        /// Connects the Node that implements this trait and uses th
        fn connect_signal_to_dialogue_box(&self, _owner: &Sprite) {
            let receiver = unsafe { Node::get_tree(_owner).unwrap()
                .assume_safe().root()
                .unwrap().assume_safe()
                .get_node("Game/Player/Camera2D/CanvasLayer/DialogueBox")
                .unwrap().assume_safe() };
            
            
            let my_signal_connected = _owner.connect("print_to_dialogue_box", 
            receiver, "_print_dialogue", VariantArray::new_shared(), 0);
        
            match my_signal_connected {
                Ok(()) => my_signal_connected.unwrap(),
                Err(e) => godot_error!("{}", e)
            };   
        }
    }
}