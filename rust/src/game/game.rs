use gdnative::prelude::*;

use serde::{Deserialize, Serialize};

use crate::utils::utils;
use crate::game::player::{PlayerData, PlayerDirection};

pub enum Status {
    Unfinished,
    Finished
}


#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    player_data: PlayerData,

    received_signals: i32,
    total_registered_signals: i32,
    // game_data: HashMap<String, _>
}

#[gdnative::methods]
impl Game {
    
    fn new(_owner: &Node2D) -> Self {
        Self {
            player_data: PlayerData::new(),
            received_signals: 0,
            total_registered_signals: 2
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        owner.set_process(true);
        owner.add_to_group("save_game_data", false);
        godot_print!("GAME DATA: {:?}", utils::retrieve_game_data())
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
        let input: &Input = Input::godot_singleton();
        if Input::is_action_just_pressed(&input, "Menu") {
            unsafe { owner.get_tree().unwrap().assume_safe().call_group(
                    "save_game_data", "save_game_data", &[]
                ) 
            };
            godot_print!("Received signasl: {:?}", self.received_signals);
        }
        if self.received_signals == self.total_registered_signals {
            utils::save_game_data(self);
            self.received_signals = 0;
        }
    }

    #[export]
    fn _save_player_position(&mut self, _owner: &Node2D, player_current_position: VariantArray) {
        let player_current_position: (f64, f64) = (player_current_position.get(0).to_f64(), player_current_position.get(1).to_f64());
        self.player_data.set_player_position(player_current_position.0, player_current_position.1);
        self.received_signals += 1;
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
        self.received_signals += 1;
    }
}