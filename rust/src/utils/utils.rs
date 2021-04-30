use gdnative::prelude::*;
use gdnative::api::{File, JSON, Node};

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
    let player_position = my_json.get("player_position").to_dictionary();

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
    
    let player_direction = my_json.get("player_direction").to_string();

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

pub fn save_player_absolute_position(player_current_position: (f32, f32)) -> String {
    let (file, json) = open_json_file(GodotString::from_str("gamestate"), File::READ_WRITE);
    
    let json_parse_result = json.parse(file.get_as_text()).unwrap();
    
    let my_data = unsafe { &json_parse_result.assume_safe().result().to_dictionary() }; 
    let player_position = my_data.get("player_position").to_dictionary();

    let player_x = player_current_position.0;
    let player_y = player_current_position.1;
    
    player_position.update("x", player_x);
    player_position.update("y", player_y);

    my_data.update("player_position", &player_position);

    let _my_modified_json = my_data;

    file.store_string(_my_modified_json.to_json().to_owned());

    //*! REMEBER TO CLOSE THE OPENED FILE HERE
    file.close();
    String::from("Saved player abs position at x: ".to_owned() + &player_x.to_string() + ", y: " + &player_y.to_string())
}

pub fn save_player_direction(player_current_direction: &PlayerDirection) {
    let (file, json) = open_json_file(GodotString::from_str("gamestate"), File::READ_WRITE);
    
    let json_parse_result = json.parse(file.get_as_text()).unwrap();
    
    let my_data = unsafe { &json_parse_result.assume_safe().result().to_dictionary() }; 

    let mut player_direction_string: String = String::new();

    if player_current_direction.to_owned() == PlayerDirection::Upwards {
        player_direction_string = "Upwards".to_string();
    } else if player_current_direction.to_owned()  == PlayerDirection::Left {
        player_direction_string = "Left".to_string();
    } else if player_current_direction.to_owned()  == PlayerDirection::Right {
        player_direction_string = "Right".to_string();
    } else {
        player_direction_string = "Downwards".to_string();
    }

    my_data.update("player_direction", &player_direction_string);

    let _my_modified_json = my_data;

    file.store_string(_my_modified_json.to_json().to_owned());

    //*! REMEBER TO CLOSE THE OPENED FILE HERE
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