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