use std::str::FromStr;

use gdnative::{api::CanvasModulate, prelude::*};
use gdnative::api::{HTTPClient, HTTPRequest};

use serde::{Deserialize, Serialize};

use crate::utils::{networking, secret, utils, consts::game_consts};
use crate::game::player::{PlayerData, PlayerDirection};

use chrono::{NaiveDate, NaiveTime, prelude::DateTime};

#[derive(PartialEq, Clone, Debug, ToVariant, Serialize, Deserialize)]
pub enum DayNightCycle {
    Day,
    Night,
    NoData
}

impl Default for DayNightCycle {
    fn default() -> Self { DayNightCycle::NoData }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameExternalData {
    // Game real time when the game starts
    real_time_when_game_starts: String,
    todays_date: String,
    todays_day_of_the_week: String,
    current_weather: String,
    todays_sunrise_time: String,
    todays_sunset_time: String,
    current_dn_cycle: DayNightCycle,
}

impl GameExternalData {
    fn new() -> Self {
        Self {
            real_time_when_game_starts: "".to_string(),
            todays_date: "".to_string(),
            todays_day_of_the_week: "".to_string(),
            current_weather: "".to_string(),
            todays_sunrise_time: "".to_string(),
            todays_sunset_time: "".to_string(),
            current_dn_cycle: DayNightCycle::NoData,
        }
    }

    /// Returns true if at least one of his attributes are not in the initial state
    fn has_data(&self) -> bool {
        if self.real_time_when_game_starts == "" || self.todays_date == "" || self.todays_day_of_the_week == ""
            || self.current_weather == "" || self.todays_sunrise_time == "" || self.todays_sunset_time == ""
            || self.current_dn_cycle == DayNightCycle::NoData {
                false
            } else { true }
    }

    /// Method that checks when ALL attributes are still on initial state. Returns true when all are NOT on the initial state.
    fn has_all_attr_with_no_default_data(&self) -> bool {
        if self.real_time_when_game_starts == "" && self.todays_date == "" && self.todays_day_of_the_week == ""
            && self.current_weather == "" && self.todays_sunrise_time == "" && self.todays_sunset_time == ""
            && self.current_dn_cycle == DayNightCycle::NoData {
                false
            } else { true }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    player_data: PlayerData,

    received_signals: i32,
    total_registered_signals: i32,

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

    // Stores the current real time in GTM + 1
    current_time: NaiveTime,
    
    //Flag for control when all external data (from IoT) are fully loaded into `game_external_data: GameExternalData` object
    full_data_retrieved: bool,
}

#[gdnative::methods]
impl Game {
    
    fn new(_owner: &Node2D) -> Self {
        Self {
            player_data: PlayerData::new(),
            // Counters that sync arriving times of different signals
            received_signals: 0,
            total_registered_signals: 2,
            // Default path of the game
            current_scene_path: "res://godot/Game/Map.tscn".to_string(),
            // Core nodes to track
            game_node: None,
            world_map_node: None,
            current_scene: None,
            // Game data of non game elements
            game_external_data: GameExternalData::new(),
            // Current time
            current_time: NaiveTime::from_hms(0, 0, 0),
            // Flag to control when the data it's fully loaded into the game
            full_data_retrieved: false,
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
        godot_print!("GAME DATA: {:#?}", &game_data);
        self.load_initial_scene(owner, game_data.current_scene_path);

        let tree = unsafe { owner.get_tree().unwrap().assume_safe().current_scene().unwrap().assume_safe().is_inside_tree() };

        // Sets the initial TIME and DATA and WEATHER information
        // self.get_time_data(owner);

        // While the new values are coming, load the most recent saved (last one stored), avoiding null data
        self.game_external_data.todays_sunrise_time = game_data.game_external_data.todays_sunrise_time;
        self.game_external_data.todays_sunset_time = game_data.game_external_data.todays_sunset_time;

        self.game_external_data.current_dn_cycle = game_data.game_external_data.current_dn_cycle;
        godot_print!("Current time: {:?}", utils::get_current_time());

        // Deactivate de main game nodes when data isn't still retrieved from the REST Api's
        unsafe { self.world_map_node.unwrap().assume_safe().cast::<Node2D>().unwrap().set_visible(false) };
        unsafe { owner.get_node_as::<Node2D>("Player").unwrap().set_visible(false) };
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
        // Sets the current real TIME inside the Game
        self.current_time = utils::get_current_time();
        godot_print!("Hora actual: {:?}", &self.current_time); 
        
        let input: &Input = Input::godot_singleton();
        
        // 1º -> Notifies all the node that had info to persist that it's time to save that data
        if Input::is_action_just_pressed(&input, "Menu") {
            self.call_save_game_data_group(owner);
        }
        // 2º -> When all signals are safetly stored in the class attributes, just call the data persistence method
        if self.received_signals == self.total_registered_signals {
            self.save_game();
        }

        if self.game_external_data.todays_date == "" {
            self.get_time_data(owner);
            self.get_sunrise_sunset_data(owner);
            self.get_weather_data(owner);
        }

        self.control_day_night(owner);

        if !self.full_data_retrieved {
            if self.game_external_data.has_all_attr_with_no_default_data() {
                unsafe { self.world_map_node.unwrap().assume_safe().cast::<Node2D>().unwrap().set_visible(true) };
                unsafe { owner.get_node_as::<Node2D>("Player").unwrap().set_visible(true) };
                
                self.full_data_retrieved = true;
            }
        }

    } 
    
    #[export]
    fn control_day_night(&mut self, owner: &Node2D) {
        let day_night_node;
            // Get's a reference to the CanvasModulate Day-Night simulator
            if let day_night_node = unsafe { owner.get_node_as::<CanvasModulate>("./Map/DayNight").unwrap() };

            // Sets the DayNightCycle to a concrete variant by comparing current time with another one...
            if utils::time_comparator(utils::get_current_time(), &self.game_external_data.todays_sunset_time) {
                // Comparing current time with the sunset time, if current time > sunset time => It's night!
                self.game_external_data.current_dn_cycle = DayNightCycle::Night;
                godot_print!("CDN: ct > sunsettime")
            } else if !utils::time_comparator(utils::get_current_time(), &self.game_external_data.todays_sunset_time) {
                self.game_external_data.current_dn_cycle = DayNightCycle::Day;
                godot_print!("CDN: ct < sunsettime")
            } else if utils::time_comparator(utils::get_current_time(), &self.game_external_data.todays_sunrise_time) {
                self.game_external_data.current_dn_cycle = DayNightCycle::Day;
                godot_print!("CDN: ct > SUNRISEtime")
            } else if !utils::time_comparator(utils::get_current_time(), &self.game_external_data.todays_sunrise_time) {
                self.game_external_data.current_dn_cycle = DayNightCycle::Night;
                godot_print!("CDN: ct < SUNRISEtime")
            }
            
            match self.game_external_data.current_dn_cycle {
                DayNightCycle::Day => day_night_node.set_deferred("color",Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
                DayNightCycle::Night => day_night_node.set_deferred("color",Color { r: 0.2, g: 0.2, b: 0.3, a: 1.0 }),
                DayNightCycle::NoData => ()
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

            // First load it as a resource
            let new_scene = ResourceLoader::godot_singleton()
                .load(path.to_string(), "", false).unwrap();

            // Convert the scene resource to a Node
            self.current_scene = unsafe { 
                new_scene.assume_safe().cast::<PackedScene>().unwrap().instance(0) };

            // Insert it on the SceneTree, and set the order
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

    /// Retrieves the current real world time in Madrid GTM
    fn get_time_data(&self, owner: &Node2D) {
        match self.new_http_node(owner, "https://worldtimeapi.org/api/timezone/Europe/Madrid", "_get_real_time_data_response")
        {
            Ok(response) => response,
            Err(err) => godot_print!("Err => {:?}", err)
        }
    }

    // Retrieves the Sunset/Sunrise data from OpenWeather
    fn get_sunrise_sunset_data(&self, owner: &Node2D) {
        let openweather_url = 
            "https://api.openweathermap.org/data/2.5/weather?q=santiago%20de%20compostela,es&lang=es&appid=".to_owned() +    
            secret::OPENWEATHER_APPID;
        match self.new_http_node(owner, &openweather_url[..], "_get_sunrise_sunset_data_response")
        {
            Ok(response) => response,
            Err(err) => godot_print!("Err => {:?}", err)
        }
    }

    /// Retrieves the weather data from OpenWeather
    fn get_weather_data(&self, owner: &Node2D) {
        let openweather_url = 
            "https://api.openweathermap.org/data/2.5/weather?q=santiago%20de%20compostela,es&lang=es&appid=".to_owned() +    
            secret::OPENWEATHER_APPID;
        match self.new_http_node(owner, &openweather_url[..], "_get_weather_data_response")
        {
            Ok(response) => response,
            Err(err) => godot_print!("Err => {:?}", err)
        }
    }

    // <-------------------- HTTP METHODS where signals send the data with the requested RESPONSES ------------------------->
    #[export]
    fn _get_real_time_data_response(&mut self, _owner: &Node2D, _result: Variant, _response_code: i64, _headers: Variant, body: ByteArray) {
        // Reads the incoming HTTP response as a String
        let response = networking::http_body_to_string(body);
        
        // Sets the current time at the moment that this method gets triggered
        let time = DateTime::parse_from_str(
            &response.get("datetime").to_string()[..],"%+").unwrap().format("%H:%M:%S").to_string();
        self.game_external_data.real_time_when_game_starts = time.to_owned();

        // Sets the current date of today
        let date = DateTime::parse_from_str(
            &response.get("datetime").to_string()[..],"%+").unwrap().format("%e %B %Y").to_string();
        self.game_external_data.todays_date = date.to_owned();
        
        // Sets the day of the week, parsing an String with a integer, to an integer value and gets back a "Day of the week" human-readable.
        let day_of_the_week =  response.get("day_of_week").to_string().parse::<i32>().unwrap();
        self.game_external_data.todays_day_of_the_week = utils::integer_to_day_of_the_week(day_of_the_week);
    }

    #[export]
    fn _get_sunrise_sunset_data_response(&mut self, _owner: &Node2D, _result: Variant, _response_code: i64, _headers: Variant, body: ByteArray) {
        let response = networking::http_body_to_string(body);

        let sunrise_hour = response.get("sys").to_dictionary().get("sunrise")
            .to_string().parse::<i32>().unwrap();
        self.game_external_data.todays_sunrise_time = utils::convert_from_unix_timestamp(
            sunrise_hour + game_consts::UNIX_TIMESTAMP_OFFSET);

        let sunset_hour = response.get("sys").to_dictionary().get("sunset")
            .to_string().parse::<i32>().unwrap();
        self.game_external_data.todays_sunset_time = utils::convert_from_unix_timestamp(
            sunset_hour + game_consts::UNIX_TIMESTAMP_OFFSET);
    }

    #[export]
    fn _get_weather_data_response(&mut self, _owner: &Node2D, _result: Variant, _response_code: i64, _headers: Variant, body: ByteArray) {
        let response = networking::http_body_to_string(body);

        let current_weather = &response.get("weather").to_array().get(0).to_dictionary()
            .get("description").to_string()[..];
        self.game_external_data.current_weather = utils::uppercase_first_letter(current_weather);
    }


 }
