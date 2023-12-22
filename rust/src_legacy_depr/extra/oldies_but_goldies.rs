use gdnative::prelude::*;

// Pool of methods that posibily will be implemented

fn _on_player_input_text_entered(&self, _owner: &Node, new_text: GodotString) {
    let line_edit = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/PlayerInput") }.unwrap();
    let label = unsafe { _owner.get_node_as::<Label>("VBoxContainer/Label") }.unwrap();
    label.set_text(new_text);
    line_edit.clear();
}


fn print_player_input(&self, _owner: &Node) {
    godot_print!("_owner on print_player_input: {:?}", _owner);
    let line_edit = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/PlayerInput") }.unwrap();
    godot_print!("Player input: {:?}", line_edit);
    line_edit.set_placeholder("Hello");
}



fn _on_player_input_text_entered(&self, _owner: &Node, new_text: GodotString) {
    let username = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/HBoxContainer/Username") }.unwrap();
    let password = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/HBoxContainer/Username") }.unwrap();
    let label = unsafe { _owner.get_node_as::<Label>("VBoxContainer/Label") }.unwrap();
    godot_print!("Content: {:?}", &new_text);
    label.set_text(new_text);
    // line_edit.clear(); 
    
}

/// Returns a TRef<Node> of the body that is colliding with our player
fn get_collision_body(&self, collision: TRef<KinematicCollision2D, Shared>) -> TRef<Node> {
    unsafe { collision
        .collider()
        .unwrap()
        .assume_safe()
        }.cast::<Node>().unwrap()
}


// if Input::is_action_pressed(&input, "Jump") && owner.is_on_floor() {
//     self.motion.y -= in_game_constant::JUMP_SPEED
// }

// fn apply_gravity(&mut self, owner: &KinematicBody2D) {
//     if owner.is_on_floor() {
//         self.motion.y = 0.0;
//     } else {
//         self.motion.y += in_game_constant::GRAVITY;
//     }
// }



// pub fn save_player_absolute_position(player_current_position: (f32, f32)) -> String {
//     let (file, json) = open_json_file(GodotString::from_str("gamestate"), File::READ_WRITE);
    
//     let json_parse_result = json.parse(file.get_as_text()).unwrap();
    
//     let my_data = unsafe { &json_parse_result.assume_safe().result().to_dictionary() }; 
//     let player_position = my_data.get("player_position").to_dictionary();

//     let player_x = player_current_position.0;
//     let player_y = player_current_position.1;
    
//     player_position.update("x", player_x);
//     player_position.update("y", player_y);

//     my_data.update("player_position", &player_position);

//     let _my_modified_json = my_data;

//     file.store_string(_my_modified_json.to_json().to_owned());

//     //*! REMEBER TO CLOSE THE OPENED FILE HERE
//     file.close();
//     String::from("Saved player abs position at x: ".to_owned() + &player_x.to_string() + ", y: " + &player_y.to_string())
// }

// pub fn save_player_direction(player_current_direction: &PlayerDirection) {
//     let (file, json) = open_json_file(GodotString::from_str("gamestate"), File::READ_WRITE);
    
//     let json_parse_result = json.parse(file.get_as_text()).unwrap();
    
//     let my_data = unsafe { &json_parse_result.assume_safe().result().to_dictionary() }; 

//     let mut player_direction_string: String = String::new();

//     if player_current_direction.to_owned() == PlayerDirection::Upwards {
//         player_direction_string = "Upwards".to_string();
//     } else if player_current_direction.to_owned()  == PlayerDirection::Left {
//         player_direction_string = "Left".to_string();
//     } else if player_current_direction.to_owned()  == PlayerDirection::Right {
//         player_direction_string = "Right".to_string();
//     } else {
//         player_direction_string = "Downwards".to_string();
//     }

//     my_data.update("player_direction", &player_direction_string);

//     let _my_modified_json = my_data;

//     file.store_string(_my_modified_json.to_json().to_owned());

//     //*! REMEBER TO CLOSE THE OPENED FILE HERE
//     file.close();
// }