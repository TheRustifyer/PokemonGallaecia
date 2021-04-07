use gdnative::prelude::*;
use gdnative::api::{Label, LineEdit, Node};

use crate::player::player_mod::Player;
use crate::utils;
use crate::consts::{labels, line_edit, scenes};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct LoginScreen {
    app_title: Option<String>,
    current_scene: Option<Ref<SceneTree, Shared>>,
    player: Option<Player>,
}

#[gdnative::methods]
impl LoginScreen {

    // The "constructor of the class"
    fn new(_owned: &Node) -> Self {
        Self { 
            app_title: None,
            current_scene: None,
            player: None
        }
    }

    // Getters and setters
    // Get a reference to the login screen's current scene.
    pub fn current_scene(&self) -> &Option<Ref<SceneTree, Shared>> {
        &self.current_scene
        }
    // Set the login screen's current scene.
    pub fn set_current_scene(&mut self, current_scene: Option<Ref<SceneTree, Shared>>) {
        self.current_scene = current_scene;
        }
    
    /// Get a reference to the login screen's player.
    fn get_player(&self) -> &Option<Player> {
        &self.player
    }

    /// Set the login screen's player.
    fn set_player(&mut self, player: Option<Player>) {
        self.player = player;
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        //Setting the intro of the app :)
        &self.set_label_text(_owner, 
            labels::APP_TITLE_LABEL_PATH.to_string(), 
            labels::APP_TITLE_LABEL.to_string()
            );
        // Prints on console real time info sended FROM Rust
        // &mut self.get_current_tree_node(_owner);

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

    fn retrieve_credentials(&self, _owner: &Node) -> (String, String){
        let get_username_on_input = unsafe 
            { _owner.get_node_as::<LineEdit>(
                &line_edit::USERNAME_LINE_EDIT_PATH) }
            .unwrap()
            .text();
        let get_password_on_input = unsafe 
            { _owner.get_node_as::<LineEdit>(
                &line_edit::PASSWORD_LINE_EDIT_PATH) }
            .unwrap()
            .text();

        // Returns a tuple with the credentials converted from GodotString to Rust String Struct
        Player::credentials_to_rust_string(
            (get_username_on_input, get_password_on_input)
            )
    }


    #[export]
    fn _on_login_button_pressed(&self, _owner: &Node) -> Option<Player> {

        let (username, password): (String, String) = self.retrieve_credentials(_owner);

        let credentials_status = 
            Player::check_credentials(
                Option::Some(&username), 
                Option::Some(&password));

        let new_player: Player;
        match credentials_status {
            (true, true) =>  {
                new_player = Player::create_new_player(username, password, 1);
                godot_print!("New Player is: {:?}", new_player);
                utils::go_next_scene(_owner, scenes::MAIN_SCENE.to_string());
                Some(new_player) // Returns a "Some" new player instance
            },
            (true, false) => { godot_print!("Wrong password. Try again."); None },
            _ => { godot_print!("Wrong credentials. Try again."); None }
        }     
    }

    // #[export]
    // fn get_current_tree_node(&mut self, _owner: &Node) -> Option<Ref<SceneTree, Shared>> {
    //     if let Some(node_tree_info) = Node::get_tree(_owner) {
    //         godot_print!("Scene Tree => {:?}", node_tree_info);
    //         &mut self.set_current_scene(Node::get_tree(_owner));
    //         Some(node_tree_info)
    //     } else {
    //         None
    //     }  
    // }

    // #[export]
    // fn go_next_scene(&self, _owner: &Node, next_scene_path: String) -> () {
    //     let scene_tree_ref = 
    //         unsafe { self.current_scene()
    //         .unwrap().assume_safe() };
        
    //     let new_scene = SceneTree::change_scene(
    //         &scene_tree_ref, next_scene_path);
        
    //     match new_scene {
    //         Ok(()) => (),
    //         Err(err) => println!("{}", err)
    //     }
    // }

}