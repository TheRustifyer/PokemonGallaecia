use gdnative::{api::{HTTPClient, JSON, JSONParseResult, http_request::{HTTPRequest, HttpRequestResult}}, prelude::*};

use serde::{Deserialize, Serialize};

use crate::utils::{utils, networking};
use crate::game::player::{PlayerData, PlayerDirection};

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub enum Status {
    Unfinished,
    Finished
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GameExternalData {
    // Game real time when the game starts
    real_time_when_game_starts: String,
    current_wheather: String,
    todays_sunrise_time: String,
    todays_sunset_time: String,
}

impl GameExternalData {
    fn new() -> Self {
        Self {
            real_time_when_game_starts: "".to_string(),
            current_wheather: "".to_string(),
            todays_sunrise_time: "".to_string(),
            todays_sunset_time: "".to_string(),
        }
    }
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

    // Game real time when the game starts
    game_external_data: GameExternalData,

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
            // Game
            game_external_data: GameExternalData::new(),
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

        // Current date-time
        self.convert_from_unix_timestamp();
    }

    fn convert_from_unix_timestamp(&self){
        // Creates a new SystemTime from the specified number of whole seconds
        let d = UNIX_EPOCH + Duration::from_secs(1620384858 + 7200);
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(d);
        // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
        godot_print!{"SUNRISE: {:?}",timestamp_str};
        godot_print!{"SUNRISE: {:?}",datetime};
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


    // <--------------------------- HTTP GAME ZONE --------------------------------------->    

    /// Creates a new HTTP Godot Node and insert it into a tree. When a url is specified, performs an HTTP request, and if `connect_to`
    /// parameter is specified, connect a signal to this class, that it's where the HTTP response comes.
    fn new_http_node(&self, owner: &Node2D, url: &str, connect_to: &str) -> Result<(), GodotError> {
        let http_request: Ref<HTTPRequest, Unique> = HTTPRequest::new();
        let http_request_as_node = unsafe { http_request.assume_safe_unchecked().assume_shared().assume_safe() };
        
        owner.add_child(http_request_as_node, true);
        
        http_request_as_node.connect("request_completed", self.game_node.unwrap(), connect_to,
            VariantArray::new_shared(), 0).unwrap();

        // Performs an http request, returning Result<(), GodotError>
        http_request.request(url, TypedArray::new(),
            true, HTTPClient::METHOD_GET, "")
    }

    /// Retrieves the current real world in this Santiago de Compostela TimeZone
    fn get_time_data(&self, owner: &Node2D) {
        match self.new_http_node(owner, "https://worldtimeapi.org/api/timezone/Europe/Madrid", "_get_real_time_response")
        {
            Ok(response) => response,
            Err(err) => godot_print!("Err => {:?}", err)
        }
    }

    // <-------------------- HTTP METHODS where signals send the data with the requested RESPONSES ------------------------->
    #[export]
    fn _get_real_time_response(&mut self, _owner: &Node2D, _result: Variant, _response_code: i64, _headers: Variant, body: ByteArray) {
        let response = networking::http_body_to_string(body);

        godot_print!("Get time data {:?}",  &response.get("datetime"))
    }

}