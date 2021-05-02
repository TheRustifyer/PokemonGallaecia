use gdnative::prelude::*;
use gdnative::api::{File, JSON, Node};

use crate::game::game::Game;

use crate::game_client::gamer::Gamer;
use crate::game::player::PlayerDirection;

/// For debug purposes, it's an easy way to check on stdout the provided credentials
pub fn print_login_credentials(credentials_tup: (&String, &String)) {
    godot_print!("Username: {:?}", credentials_tup.0);
    godot_print!("Password: {:?}", credentials_tup.1);
}

/// Prints on console the current data on the Player struct
/// This can be useful to debug the "in place" current values of Player attributes
pub fn show_player_attributes(player: &Gamer) {
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

pub fn get_player_absolute_position() -> (f32, f32) {
    let (file, json) = open_json_file(GodotString::from_str("gamestate"), File::READ);

    let my_data = json.parse(file.get_as_text()).expect("SI, error parseando el JSON");
    let my_json = unsafe { &my_data.assume_safe().result().to_dictionary() }; 
    let player_position = my_json.get("player_data").to_dictionary()
        .get("player_position").to_dictionary();

    let player_x = player_position.get("x").to_f64() as f32;
    let player_y = player_position.get("y").to_f64() as f32;

    //*! REMEBER TO CLOSE THE OPENED FILE HERE
    file.close();
    (player_x, player_y)
}

pub fn get_player_direction() -> PlayerDirection {
    let (file, json) = open_json_file(GodotString::from_str("gamestate"), File::READ);

    let my_data = json.parse(file.get_as_text()).expect("SI, error parseando el JSON");
    let my_json = unsafe { &my_data.assume_safe().result().to_dictionary() }; 
    
    let player_direction = my_json.get("player_data").to_dictionary().get("player_direction").to_string();

    //*! REMEBER TO CLOSE THE OPENED FILE HERE
    file.close();

    if player_direction == "Upwards".to_string() {
        PlayerDirection::Upwards
    } else if player_direction == "Downwards".to_string() {
        PlayerDirection::Downwards
    } else if player_direction == "Left".to_string() {
        PlayerDirection::Left
    } else if player_direction == "Right".to_string() {
        PlayerDirection::Right
    } else {
        PlayerDirection::default()
    }
}

pub fn retrieve_game_data() -> Game {
    let (file, _) = open_json_file(GodotString::from_str("gamestate"), File::READ);
    let json_game_data = &file.get_as_text().to_string()[..];

    let game_data: Game = serde_json::from_str(&json_game_data).unwrap();

    return game_data;
}

pub fn save_game_data(player_data: &Game) {
    let (file, _) = open_json_file(GodotString::from_str("gamestate"), File::WRITE);

    let j = serde_json::to_string_pretty(&player_data).unwrap();

    file.store_string(&j);
    file.close();
}

pub fn open_json_file(file_name: GodotString, mode: i64) -> (Ref<File, Unique>, &'static JSON ){
    let file = File::new();
    let json = JSON::godot_singleton();

    let file_name: String = "res://godot/".to_string() + &file_name.to_string() + &".json".to_string();

    let gamestate = file.open(file_name, mode);
    match gamestate {
        Ok(()) => (),
        Err(err) => godot_print!("Error. File not found!: {:?}", err)
    }

    (file, json)
}