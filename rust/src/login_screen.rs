use gdnative::prelude::*;
use gdnative::api::{Label, LineEdit};

use crate::player::player_mod::Player;
use crate::utils;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct LoginScreen {
    app_title: String,
    // player: Player,
    // label: Ref<Label>,
}

#[gdnative::methods]
impl LoginScreen {

    // The "constructor of the class"
    fn new(_owned: &Node) -> Self {
        Self { 
            app_title: String::from("Learn Programming With Godot"),
            // player: Player { username: "".to_string(), password: "".to_string(), level: 1 } 
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        let app_title_label = unsafe { _owner.get_node_as::<Label>("VBoxContainer/Label") }.unwrap();
        app_title_label.set_text(&self.app_title);
        
    }

    // #[export]
    fn _create_player() {}

    #[export]
    fn set_label_text(&self, _owner: &Node, _label_identifier: String, text: String) {
        let label = unsafe { _owner.get_node_as::<Label>(&_label_identifier) }.unwrap();
        godot_print!("Label var value on set_label_text: {:?}", label);
        label.set_text(text);
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
            Player::check_credentials(&username, &password);

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

    
}