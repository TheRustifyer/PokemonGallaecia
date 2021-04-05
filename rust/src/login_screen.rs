use gdnative::prelude::*;
use gdnative::api::{Label, LineEdit, Node};

use crate::player::player_mod::Player;
use crate::utils;
use crate::consts::labels;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct LoginScreen {
    app_title: Option<String>,
    // player: Player,
    // label: Ref<Label>,
}

#[gdnative::methods]
impl LoginScreen {

    // The "constructor of the class"
    fn new(_owned: &Node) -> Self {
        Self { 
            app_title: None,
            // player: Player { username: "".to_string(), password: "".to_string(), level: 1 } 
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        //Setting the intro of the app :)
        &self.set_label_text(_owner, 
            labels::APP_TITLE_LABEL_PATH.to_string(), 
            labels::APP_TITLE_LABEL.to_string()
            );
        // Prints on console real time info sended FROM Rust
        &self.get_tree_node_info(_owner);

    }

    #[export]
    fn set_label_text(&self, _owner: &Node, _label_path: String, text: String) {
        let app_title_label = unsafe { 
            _owner.get_node_as::<Label>(&_label_path) }
            .unwrap();
            
        app_title_label.
            set_text(&self.app_title
                .as_ref()
                .unwrap_or(&text)
            );
    }


    #[export]
    fn _on_login_button_pressed(&self, _owner: &Node) {
        let username_input = unsafe 
            { _owner.get_node_as::<LineEdit>("VBoxContainer/HBoxContainer/UsernameInput") }
            .unwrap()
            .text();
        let password_input = unsafe 
            { _owner.get_node_as::<LineEdit>("VBoxContainer/HBoxContainer/PasswordInput") }
            .unwrap()
            .text();
        
        let (username, password): (String, String) = (
            username_input.to_string(), 
            password_input.to_string()
        );
        
        // Just for debug  purposes
        utils::print_login_credentials((&username, &password));

        let credentials_status = 
            Player::check_credentials(Option::Some(&username), Option::Some(&password));

            if credentials_status.0 && credentials_status.1 {
                let new_player: Player = Player::create_new_player(
                    username, password, 1);
                godot_print!("New Player is: {:?}", new_player)

            } else {
                if !credentials_status.0 {
                    godot_print!("Wrong username. Try again.")
                } else if !credentials_status.1 {
                    godot_print!("Incorrect password. Try again.")
                }
            }
    }

    #[export]
    fn get_tree_node_info(&self, _owner: &Node) -> Option<Ref<SceneTree, Shared>> {
        if let Some(node_tree_info) = Node::get_tree(_owner) {
            godot_print!("Node info => {:?}", node_tree_info);
        };
        Node::get_tree(_owner).
    }

    
}