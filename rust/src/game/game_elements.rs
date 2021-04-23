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

    const DIALOGUE_SPEED: f64 = 0.05;
    /// Dialogue Box it's build to manage all the text interactions in the game
    #[derive(NativeClass)]
    #[inherit(NinePatchRect)]
    
    #[derive(Debug)]
    pub struct DialogueBox {
        printing: bool,
        timer: f64,
        text_to_print: String,

        current_char: i32,

        dialogue_text_label: Option<Ref<RichTextLabel>>,

    }

    #[gdnative::methods]
    impl DialogueBox {

        fn new(_owner: &NinePatchRect) -> Self {
            Self {
                printing: false,
                timer: 0.0,
                text_to_print: Default::default(),
                current_char: 0,
                dialogue_text_label: None,
            } 
        }

        #[export]
        fn _ready(&mut self, owner: &NinePatchRect) {
            owner.set_process(true);
            self.dialogue_text_label = Some(unsafe { Node::get_tree(&owner).unwrap()
                .assume_safe().root()
                .unwrap().assume_safe()
                .get_node("Game/Player/Camera2D/DialogueBox/DialogueTextLabel")
                .unwrap().assume_safe().cast::<RichTextLabel>().unwrap().assume_shared() });
        }

        #[export]
        fn _process(&mut self, _owner: &NinePatchRect, _delta: f64) {
            if self.printing {
                self.timer += _delta;
                if self.timer > DIALOGUE_SPEED {

                    self.timer = 0.0;
                    let dialogue_text_label = unsafe { self.dialogue_text_label.unwrap().assume_safe() };

                    // Make visible the Pokémon Dialog Box
                    _owner.set_visible(true);
                    
                    if self.current_char < self.text_to_print.len() as i32 - 1 {                       
                        dialogue_text_label.set_bbcode(dialogue_text_label.bbcode() + 
                            GodotString::from(String::from(self.text_to_print.chars().nth(self.current_char as usize).expect("No more chars to print"))));                       
                        // If there still chars remaining to print, move next
                        self.current_char += 1;
                    } else {
                        self.current_char = 0;
                        self.printing = false;
                        self.timer = 0.0;
                        _owner.set_visible(false);
                        dialogue_text_label.set_bbcode("");
                    } 
                }
            }
        }

        #[export]
        fn _print_dialogue(&mut self, _owner: &NinePatchRect, text: GodotString) {
            self.printing = true;
            self.text_to_print = text.to_string();
        }
    }

}


pub mod in_game_interactions {
    use gdnative::prelude::*;

    // use gdnative::api::{Tree, NinePatchRect, RichTextLabel};
    use super::signals::RegisterSignal;
    #[derive(NativeClass)]
    #[inherit(Sprite)]
    #[register_with(Self::register_signal)]
    #[derive(Debug)]
    pub struct Truck {
        times_signal_emitted: i32,
    }

    impl RegisterSignal<Self> for Truck {
        fn register_signal(_builder: &ClassBuilder<Self>) -> () {
            _builder.add_signal( Signal {
                name: "print_to_dialogue_box",
                args: &[],
            })
        }
    }

    #[gdnative::methods]
    impl Truck {
        
        fn new(_owner: &Sprite) -> Self {
            Self {
                times_signal_emitted: 0,
            }
        }

        #[export]
        fn _ready(&self, _owner: TRef<Sprite>) {
            // _owner.set_process(true);
            // Looking for interactions with the player
            self.connect_to_player(_owner);
            self.connect_signal_to_dialogue_box(&_owner)
        }

        // #[export]
        // fn _process(&mut self, _owner: TRef<Sprite>, _delta: f32) {   
        //     if self.times_signal_emitted < 2 {
        //         _owner.emit_signal("print_to_dialogue_box", &[Variant::from_godot_string(
        //             &GodotString::from_str("Soy el camión de Pueblo de Teo!!"))]);
        //             self.times_signal_emitted += 1
        //     }    
        // }

        #[export]
        fn emit_struct_signal(&self, _owner: TRef<Sprite>) {
            _owner.emit_signal("print_to_dialogue_box", &[Variant::from_godot_string(
                &GodotString::from_str("Soy el camión de Pueblo de Teo!!"))]);
        }

        #[export]
        fn connect_to_player(&self, _owner: TRef<Sprite>) {
            let player_signal = unsafe { Node::get_tree(&_owner).unwrap()
                .assume_safe().root()
                .unwrap().assume_safe()
                .get_node("Game/Player")
                .unwrap().assume_safe() };

                player_signal.connect("player_interacting", _owner, 
                "emit_struct_signal", VariantArray::new_shared(), 0).unwrap();
        }

        #[export]
        fn connect_signal_to_dialogue_box(&self, _owner: &Sprite) {
            let receiver = unsafe { Node::get_tree(_owner).unwrap()
                .assume_safe().root()
                .unwrap().assume_safe()
                .get_node("Game/Player/Camera2D/DialogueBox")
                .unwrap().assume_safe() };
            
            // if !receiver.is_connected("print_to_dialogue_box", receiver, "_print_dialogue") {
                let my_signal_connected = _owner.connect("print_to_dialogue_box", 
                receiver, "_print_dialogue", VariantArray::new_shared(), 0);
            
                match my_signal_connected {
                    Ok(()) => my_signal_connected.unwrap(),
                    Err(e) => godot_error!("{}", e)
                };   
            // }
        }

    }

}   
