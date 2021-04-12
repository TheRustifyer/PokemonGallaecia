use gdnative::prelude::*;
use gdnative::api::{LineEdit, Node};

use crate::utils;
use crate::player::Gamer;
use crate::consts::{labels, line_edit, scenes};
#[derive(NativeClass)]
#[inherit(Node)]
pub struct LoginScreen {
    gamer: Option<Gamer>,
}

#[gdnative::methods]
impl LoginScreen {

    // The "constructor of the class"
    fn new(_owned: &Node) -> Self {
        Self { 
            gamer: None
        }
    }

    // /// Get a reference to the login screen's player.
    // pub fn get_player(&self) -> &Option<Player> {
    //     &self.player
    // }
    /// Setter for the logged player
    fn set_player(&mut self, player: Option<Gamer>) {
        self.gamer = player;
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        //Setting the intro of the app :)
        utils::set_label_text(_owner, 
            &labels::APP_TITLE_LABEL_PATH.to_string(), 
            &labels::APP_TITLE_LABEL.to_string()
            );
    }

    /// Gets the inputed credentials on the Login Screen Line Edits
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

        // Returns a tuple with the credentials converted from GodotString to Rust String
        Gamer::credentials_to_rust_string((get_username_on_input, get_password_on_input))
    }

    #[export]
    /// The receiver of the signal from Godot when the login button gets pressed
    fn _on_login_button_pressed(&mut self, _owner: &Node) {

        let (username, password): (String, String) = self.retrieve_credentials(_owner);

        let credentials_status = 
            Gamer::check_credentials(
                Option::Some(&username), 
                Option::Some(&password));

        let new_player: Gamer;
        match credentials_status {
            (true, true) =>  {
                // Credentials are correct, so a new Gamer is instanciated
                new_player = Gamer::gamer_login(username, password, 1);
                utils::show_player_attributes(&new_player);
                
                // Storing a reference to the new player as the current Gamer for the "game session"
                &mut self.set_player(Some(new_player));
                
                // Finally, with the new player creaded we can move to the main scene
                utils::change_scene(_owner, scenes::LEVEL_1.to_string());
            },
            // This should be changed for on screen labels on the future. Fine for now ;)
            (true, false) => godot_print!("Wrong password. Try again."),
            _ => godot_print!("Wrong credentials. Try again.")
        }     
    }
}