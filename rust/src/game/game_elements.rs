/// A base blueprint that defines the behaviour of the game characters.
///
/// UnderDevelopment: This will grow while the game evolves!
///
/// The interesting thing here it's to make certain abstractions for the core game elements
/// and help another dev/devs that wants to join the project

/// Most of the things coded in this game are used like wrappers over the `gdnative API`
/// in a try of making the process of writting the game repetitive code less verbose

// pub mod character {
//     use super::signals::Signal;
//     use gdnative::prelude::ClassBuilder;
//     /// Base Struct that represents a Kinematic Body as a Player Character.
//     /// This "class" can represent the Player owned by the gamer, an enemy character, a person as character in game...
//     pub struct Character {}

//     impl Signal for Character {
//         fn register_signal(_t: &ClassBuilder<Self>, signal: Signal) -> () {
//             _t.add_signal(signal)
//         }
//     }
// }


pub mod signals {
    use gdnative::prelude::*;
    use gdnative::prelude::ClassBuilder;

    // type Character = <type>;

    /// **Signal** -> Zero cost abstraction for handling the `Godot signals` in a custom approach
    /// Method for register a new signal to a designed class. You can find on the GUI Godot
    /// that signal registered on the Node panel on the same way if the signal was created directly on the GUI.
    /// The name of the method is completly arbitrary, is just a way to encapsulate the info passed to the builder object and transport it back to Godot.
    ///
    /// Remember to implement the `#[register_with(Self::N)]` attribute where `N` is the name of the method that is registering the signal
    pub trait RegisterSignal {
        
        /// Registers a signal on `Godot`directly from the Rust code.
        fn register_signal<T>(_builder: &ClassBuilder<T>) -> ();

        // /// Set the name of the signals that you want to register
        // fn set_signal_name(name: &str) -> &str {
        //     name
        // }

        // /// S
    }


}

pub mod dialog_box {

    use gdnative::prelude::*;
    use gdnative::api::NinePatchRect;
    /// Dialogue Box it's build to manage all the text interactions in the game
    #[derive(NativeClass)]
    #[inherit(NinePatchRect)]
    pub struct DialogueBox {
        printing: bool,
        timer: u8,
        text_to_print: GodotString,

        current_char: u8
    }

    #[gdnative::methods]
    impl DialogueBox {
        
        fn new(owner: &NinePatchRect) -> Self {
            Self {
                printing: false,
                timer: 0,
                text_to_print: Default::default(),
                current_char: 0,
            }
        }

        #[export]
        fn _ready(&self, owner: &NinePatchRect) {
            owner.set_process(true);
        }

        #[export]
        fn _fixed_process(&self, owner: &NinePatchRect, _delta: f32) {

        }
    }

    

}