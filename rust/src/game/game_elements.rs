/// A base blueprint that defines the behaviour of the game characters.
///
/// UnderDevelopment: This will grow while the game evolves!
///
/// The interesting thing here it's to make certain abstractions for the core game elements
/// and help another dev/devs that wants to join the project

/// Most of the things coded in this game are used like wrappers over the `gdnative API`
/// in a try of making the process of writting the game repetitive code less verbose

/// **Signal** -> Zero cost abstraction for handling the `Godot signals` in a custom approach
pub trait Signal {
    /// Register a signal on `Godot`directly from the Rust code.
    fn register_signal<T>(t: T) -> ();
}

/// Base Struct that represents a Kinematic Body as a Player Character.
/// Can represent the Player owned by the gamer, an enemy character, a person as character in game...
pub struct Character {}

impl Signal for Character {
    fn register_signal<T>(_t: T) -> () {
        todo!()
    }
}