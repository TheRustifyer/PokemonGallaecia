use gdnative::prelude::*;

mod login_screen;

fn init(handle: InitHandle) {
    // Here we register the Rust Structs that will register as classes on GDScript

    //First just will register a class that only prints a greet on the Godot Engine console
    //to ensure that all bindings, export and boilerplate stuff are done correctly
    handle.add_class::<login_screen::LoginScreen>();
}

godot_init!(init);