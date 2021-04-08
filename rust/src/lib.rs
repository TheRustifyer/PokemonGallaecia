use gdnative::prelude::*;

pub mod player;
mod login_screen;
mod consts;
pub mod utils;

fn init(handle: InitHandle) {
    // Here we register the Rust Structs that will register as classes on GDScript

    //First just will register a class that only prints a greet on the Godot Engine console
    //to ensure that all bindings, export and boilerplate stuff are done correctly
    handle.add_class::<login_screen::LoginScreen>();
    handle.add_class::<player::Player>();
}

godot_init!(init);