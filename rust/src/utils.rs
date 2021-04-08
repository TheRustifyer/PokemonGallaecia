use gdnative::prelude::*;
use gdnative::api::Node;

use crate::player::Player;

/// For debug purposes, it's an easy way to check on stdout the provided credentials
pub fn print_login_credentials(credentials_tup: (&String, &String)) {
    godot_print!("Username: {:?}", credentials_tup.0);
    godot_print!("Password: {:?}", credentials_tup.1);
}

/// Prints on console the current data on the Player struct
/// This can be useful to debug the "in place" current values of Player attributes
pub fn show_player_attributes(player: &Player) {
    godot_print!("New Player is: {:?}", player);
}

/// Changes the text of a label, if an _owner, a text and a path are provided.
/// The path to the label are a String like "res://path_to_the_label"
pub fn set_label_text(_owner: &Node, _label_path: &String, text: &String) {
    let app_title_label = unsafe { 
        _owner.get_node_as::<Label>(&_label_path) }
        .unwrap();
        
    app_title_label.
        set_text(text);
}

/// Convenient function to change scene just passing the _owner and a path as a String
pub fn change_scene(_owner: &Node, next_scene_path: String) -> () {
    
    let scene_tree_ref = 
        unsafe { Node::get_tree(_owner)
        .unwrap().assume_safe() };
    
    let new_scene = SceneTree::change_scene(
        &scene_tree_ref, next_scene_path);
    
    match new_scene {
        Ok(()) => (),
        Err(err) => println!("{}", err)
    }
}