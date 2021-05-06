use gdnative::{api::{HTTPClient, JSON, JSONParseResult, http_request::{HTTPRequest, HttpRequestResult}}, prelude::*};

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

    // CurrentScenePath
    current_scene_path: String,

    //References to Nodes that will be dropped and added as childs during the game
    #[serde(skip)]
    game_node: Option<Ref<Node>>,
    #[serde(skip)]
    world_map_node: Option<Ref<Node>>,
    #[serde(skip)]
    current_scene: Option<Ref<Node>>,
}

#[gdnative::methods]
impl Game {
    
    fn new(_owner: &Node2D) -> Self {
        Self {
            player_data: PlayerData::new(),
            received_signals: 0,
            total_registered_signals: 2,
            current_scene_path: "res://godot/Game/Map.tscn".to_string(),
            game_node: None,
            world_map_node: None,
            current_scene: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        owner.set_process(true);
        owner.add_to_group("save_game_data", false);

        // Gets references to the core nodes of the game
        self.game_node = owner.get_node(".");
        self.world_map_node = owner.get_node("Map");

        // Loads the correct scene from where the player was the last time that saved the game
        let game_data: Game = utils::retrieve_game_data();
        godot_print!("GAME DATA: {:?}", utils::retrieve_game_data());
        self.load_initial_scene(owner, game_data.current_scene_path);

        // Sets the initial TIME and DATA and WEATHER information
        self.get_time_data(owner);
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
        let input: &Input = Input::godot_singleton();
        
        // 1º -> Notifies all the node that had info to persist that it's time to save that data
        if Input::is_action_just_pressed(&input, "Menu") {
            self.call_save_game_data_group(owner);
        }
        // 2º -> When all signals are safetly stored in the class attributes, just call the data persistence method
        if self.received_signals == self.total_registered_signals {
            self.save_game();
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

    /// Method that calls the save game data group. After the call all the nodes attached to the group will send 
    /// the information that should be persisted
    fn call_save_game_data_group(&self, owner: &Node2D) {
        unsafe { owner.get_tree().unwrap().assume_safe().call_group(
        "save_game_data", "save_game_data", &[]) };
    }

    /// ### Method that persist the data stores in the class attributes
    ///
    fn save_game(&mut self) {
        //! Calls the function who takes care about all IO operations to persist the retrieved data.
        utils::save_game_data(self);
        // Resets the counter that acts as a "all data syncronized and ready to be stored"
        self.received_signals = 0;
    }

    ///
    fn get_time_data(&self, owner: &Node2D) {
        let http_request: Ref<HTTPRequest, Unique> = HTTPRequest::new();
        let http_request_as_node = unsafe { http_request.assume_safe_unchecked().assume_shared().assume_safe() };
        
        owner.add_child(http_request_as_node, true);
        
        http_request_as_node.connect("request_completed", self.game_node.unwrap(), "_http_request_completed",
        VariantArray::new_shared(), 0).unwrap();

        let response = http_request.request("http://worldclockapi.com/api/json/utc/now", TypedArray::new(),
         true, HTTPClient::METHOD_GET, "");

        match response {
            Ok(response) => response,
            Err(err) => godot_print!("Err => {:?}", err)
        }

        godot_print!("Response esta si: {:?}", response);

    }

    #[export]
    fn _http_request_completed(&self, owner: &Node2D, result: Variant, response_code: i64, headers: Variant, body: ByteArray) {
        godot_print!("Response esta si: {:?}", &result);
        godot_print!("Response esta si: {:?}", &response_code);

        let json = JSON::godot_singleton();
        let mut vector = Vec::new();

        for number in 0..body.len() {
            let current_byte = body.get(number);
            vector.push(current_byte)
        }

        let final_vec = std::str::from_utf8(&vector).unwrap();
        let final_response = unsafe { json.parse(final_vec)
            .unwrap().assume_safe().result() };
        
        godot_print!("FINAL: {:?}", &final_response);
    }

    /// Method for load the correct scene, based on last saved player Scene
    fn load_initial_scene(&mut self, owner: &Node2D, path: String) {
        if !path.ends_with("Map.tscn") {
            owner.remove_child(self.world_map_node.unwrap());

            // In order to go to a new scene, we must first load it as a resource
            let new_scene = ResourceLoader::godot_singleton()
            .load(path.to_string(), "", false).unwrap();

            // Convert the scene resource to a Node
            self.current_scene = unsafe { 
                new_scene.assume_safe().cast::<PackedScene>().unwrap().instance(0) };
            owner.add_child(self.current_scene.unwrap(), true);
            owner.move_child(self.current_scene.unwrap(), 0)
        }
    }

    #[export]
    /// This method it's the receiver of the signal that notifies that the game detected the player on an area designed to switch him
    /// from the outside world to a building interior, and VICEVERSA
    fn change_map(&mut self, owner: &Node2D, path: Variant) {
        
        // Stores a path to a scene provided by a signal triggered for a collision between an area and a playe
        self.current_scene_path  = path.to_string();

        // Going from outdoors to indoors...
        if self.current_scene_path.ends_with("Map.tscn") {
            owner.remove_child(self.current_scene.unwrap());
            owner.add_child(self.world_map_node.unwrap(), true);
            owner.move_child(self.world_map_node.unwrap(), 0)
        // Changing to an inside scene...
        } else {
            // Now let's gonna remove the Map from the SceneTree
            owner.remove_child(self.world_map_node.unwrap());

            // In order to go to a new scene, we must first load it as a resource
            let new_scene = ResourceLoader::godot_singleton()
            .load(path.to_string(), "", false).unwrap();

            // Convert the scene resource to a Node
            self.current_scene = unsafe { 
                new_scene.assume_safe().cast::<PackedScene>().unwrap().instance(0) };

            // Finally we insert our new Node, setting Game as it's parent
            owner.add_child(self.current_scene.unwrap(), false);
            unsafe { self.current_scene.unwrap().assume_safe().set_owner(self.game_node.unwrap()) };
            
            // To render the Nodes for it's correct superposition one over another, let's move the 
            // new inserted child to the position that fits the "surface" drawing role.
            owner.move_child(self.current_scene.unwrap(), 0)
        }
    }
}