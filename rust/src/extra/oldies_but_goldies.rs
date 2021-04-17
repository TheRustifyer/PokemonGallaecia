use gdnative::prelude::*;

// Pool of methods that posibily will be implemented
#[export]
fn _on_player_input_text_entered(&self, _owner: &Node, new_text: GodotString) {
    let line_edit = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/PlayerInput") }.unwrap();
    let label = unsafe { _owner.get_node_as::<Label>("VBoxContainer/Label") }.unwrap();
    label.set_text(new_text);
    line_edit.clear();
}

#[export]
fn print_player_input(&self, _owner: &Node) {
    godot_print!("_owner on print_player_input: {:?}", _owner);
    let line_edit = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/PlayerInput") }.unwrap();
    godot_print!("Player input: {:?}", line_edit);
    line_edit.set_placeholder("Hello");
}


#[export]
fn _on_player_input_text_entered(&self, _owner: &Node, new_text: GodotString) {
    let username = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/HBoxContainer/Username") }.unwrap();
    let password = unsafe { _owner.get_node_as::<LineEdit>("VBoxContainer/HBoxContainer/Username") }.unwrap();
    let label = unsafe { _owner.get_node_as::<Label>("VBoxContainer/Label") }.unwrap();
    godot_print!("Content: {:?}", &new_text);
    label.set_text(new_text);
    // line_edit.clear(); 
    
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