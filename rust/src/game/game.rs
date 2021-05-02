use gdnative::prelude::*;
use gdnative::api::{File, JSON, Node};

use serde_json::{Error, Map, Number, Value, json};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::utils::utils;

use crate::game::player::{PlayerData, PlayerDirection};
use crate::game::code_abstractions::signals::RegisterSignal;


#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    player_data: PlayerData,
    // game_data: HashMap<String, >
    // game_data: String,
}

impl RegisterSignal<Self> for Game {
    fn register_signal(builder: &ClassBuilder<Self>) {
        builder.add_signal( Signal {
            name: "save_game_data",
            args: &[]
        });
    }
}

#[gdnative::methods]
impl Game {
    
    fn new(_owner: &Node2D) -> Self {
        Self {
            player_data: PlayerData::new(),
            // game_data: "".to_owned()
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        // utils::retrieve_game_data() // Currently not working as desired
    }

    #[export]
    fn _save_player_position(&mut self, _owner: &Node2D, player_current_position: VariantArray) {
        let player_current_position: (f64, f64) = (player_current_position.get(0).to_f64(), player_current_position.get(1).to_f64());
        self.player_data.set_player_position(player_current_position.0, player_current_position.1);
        
    }

    #[export]
    fn _save_player_direction(&mut self, _owner: &Node2D, player_current_direction: Variant) {
        let player_current_direction = player_current_direction.to_string();
        let slice = &player_current_direction[1..player_current_direction.len() - 4];
        match slice {
            "Upwards" => self.player_data.set_player_direction(&PlayerDirection::Upwards),
            "Downwards" => self.player_data.set_player_direction(&PlayerDirection::Downwards),
            "Left" => self.player_data.set_player_direction(&PlayerDirection::Left),
            "Right" => self.player_data.set_player_direction(&PlayerDirection::Right),
            _ => ()
        }
        // godot_print!("Data of Game's Player Direction {:?}", &self.player_data);
        utils::save_game_data(self);
    }


}