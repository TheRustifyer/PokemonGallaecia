use gdnative::prelude::*;
use gdnative::api::Node;

pub fn print_login_credentials(credentials_tup: (&String, &String)) {
    godot_print!("Username: {:?}", credentials_tup.0);
    godot_print!("Password: {:?}", credentials_tup.1);
}

// pub fn get_node_info() -> Option<Ref<SceneTree, Shared>> {
//     Node::get_tree()
// }