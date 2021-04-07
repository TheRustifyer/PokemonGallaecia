use gdnative::prelude::*;
use gdnative::api::Node;

pub fn print_login_credentials(credentials_tup: (&String, &String)) {
    godot_print!("Username: {:?}", credentials_tup.0);
    godot_print!("Password: {:?}", credentials_tup.1);
}

// pub fn get_node_info() -> Option<Ref<SceneTree, Shared>> {
//     Node::get_tree()
// }

pub fn go_next_scene(_owner: &Node, next_scene_path: String) -> () {
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