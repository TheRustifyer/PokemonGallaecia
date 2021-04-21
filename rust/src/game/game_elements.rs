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
    use gdnative::{api::viewport::Usage, prelude::*};

    // type Character = <type>;
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

        //// Set the name of the signals that you want to register
        // fn get_signal_args(&self) -> (&str, &str) {
        //     (&self.name, 
        // }

        // fn internal_signal_params() -> 

        // /// S
    }


}

pub mod dialog_box {

    use gdnative::{api::RichTextLabel, prelude::*};
    use gdnative::api::NinePatchRect;

    const DIALOGUE_SPEED: f64 = 0.1;
    /// Dialogue Box it's build to manage all the text interactions in the game
    #[derive(NativeClass)]
    #[inherit(NinePatchRect)]
    #[derive(Debug)]
    pub struct DialogueBox {
        printing: bool,
        timer: i32,
        text_to_print: GodotString,

        current_char: i32,
    }

    #[gdnative::methods]
    impl DialogueBox {
        
        fn new(_owner: &NinePatchRect) -> Self {
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
            godot_print!("Label init called");
        }

        #[export]
        fn _fixed_process(&mut self, _owner: &NinePatchRect, _delta: f32) {
            if self.printing {
                self.timer = _delta as i32;
                godot_print!("Trying to set the bbcode");
                if self.timer > DIALOGUE_SPEED as i32 {
                    godot_print!("Trying to set the bbcode inside the check timer");
                    self.timer = 0;
                    
                    let dialogue_text_label =
                        unsafe { _owner.get_node_as::<RichTextLabel>("DialogueTextLabel") }.unwrap();
                    dialogue_text_label.set_bbcode("Hola Pokémon Gallaecia desde Rust!");
                    
                    // dialogue_text_label.set_visible(visible);

                    self.current_char += 1;
                    godot_print!("Trying to set the bbcode, rich text label retrieved.");
                }

                if self.current_char >= self.text_to_print.len() as i32 {
                    self.current_char = 0;
                    self.text_to_print = GodotString::from_str("");
                    self.printing = false;
                    self.timer = 0;
                }
                
            }
        }

        #[export]
        fn _print_dialogue(&mut self, _owner: &NinePatchRect, text: GodotString) {
            self.printing = true;
            self.text_to_print = text;
        }



    }

    

}