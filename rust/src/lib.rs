use gdnative::prelude::*;

pub mod game;
pub mod game_client;
pub mod utils;

use game_client::login_screen::LoginScreen;

use game::map::Map;
use game::player::{PlayerAnimation, PlayerCharacter};
use game::dialogue_box::DialogueBox;

use game::map_elements__galicia::{
    area1_pueblo_de_teo,
    // area1_ames
};


fn init(handle: InitHandle) {
    // Here we register the Rust Structs that will register as classes on GDScript

    //First just will register a class that only prints a greet on the Godot Engine console
    //to ensure that all bindings, export and boilerplate stuff are done correctly
    handle.add_class::<LoginScreen>();
    handle.add_class::<PlayerCharacter>();
    handle.add_class::<PlayerAnimation>();
    handle.add_class::<Map>();
    handle.add_class::<DialogueBox>();
    
    handle.add_class::<area1_pueblo_de_teo::truck::Truck>();
}

godot_init!(init);