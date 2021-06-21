use std::fmt::Display;

use gdnative::{api::CanvasModulate, prelude::*};
use gdnative::api::{AnimationPlayer, Particles2D};
use gdnative::api::{HTTPClient, HTTPRequest};

use serde::{Deserialize, Serialize};

use crate::utils::{consts::game_consts, networking, utils};
use crate::game::player::{PlayerData, PlayerDirection};

use chrono::{Duration, NaiveTime};

use super::code_abstractions::database::Database;
use super::city::{GameCity, City, CityWeather};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Game {
    // Flag to quickly control if we are working on development or in production
    #[serde(skip)]
    in_development: bool,

    // Urls of dev and production backends
    #[serde(skip)]
    development_url: &'static str,
    #[serde(skip)]
    production_url: &'static str,

    // The struct that will hold all necesary Player Data
    player_data: PlayerData,

    // A list storing all the availiable locations on the game
    #[serde(skip)]
    game_cities: Vec<City>,

    // Some "trackers"
    received_signals: i32,
    total_registered_signals: i32,
    number_of_process: i32,

    // Current Scene Type
    current_scene_type: CurrentSceneType,
    // CurrentScenePath
    current_scene_path: String,

    // Game real time when the game starts
    game_external_data: GameExternalData,
    // When the game will ask for new data, updating the old one to the REST API
    next_api_call: NaiveTime,

    //References to the most important nodes of the game
    #[serde(skip)]
    game_node: Option<Ref<Node>>,
    #[serde(skip)]
    world_map_node: Option<Ref<Node>>,
    #[serde(skip)]
    current_scene: Option<Ref<Node>>,
    #[serde(skip)]
    database: Option<TRef<'static, Node>>,

    // The current real time in GTM + 1. When game it's saved, stores the time when game has succesfully saved.
    current_time: NaiveTime,
    // Tracks the current weather in the place that the player is
    #[serde(skip)]
    current_weather: Weather,
    
    //Flag for control when all external data are fully loaded into `game_external_data: GameExternalData` object
    #[serde(skip)]
    full_data_retrieved: bool,
    
    // Binding to the Input singleton
    #[serde(skip)]
    input: Option<&'static Input>
}


// Impl of database will use the "default implementation of the trait methods"
impl Database for Game {}

#[gdnative::methods]
impl Game {
    
    fn new(_owner: &Node2D) -> Self {
        Self {
            // Development or production flag
            in_development: true,
            // Backend server addresses
            development_url: "http://localhost:8080/api/Game",
            production_url: "",
            // Initializes a new `PlayerData` struct 
            player_data: PlayerData::new(),
            // Locations
            game_cities: Vec::new(),
            // Next API call
            next_api_call: NaiveTime::from_hms(0, 0, 0),
            // Counters that sync arriving times of different signals
            received_signals: 0,
            total_registered_signals: 2,
            // TTimes that the process function is called
            number_of_process: 0,
            // User define enum to represent in which type of world scene the player is
            current_scene_type: Default::default(),
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
            // Current Weather
            current_weather: Weather::Sun,
            // Flag to control when the data it's fully loaded into the game
            full_data_retrieved: false,
            // Input 
            input: Some(Input::godot_singleton()),
            // Database
            database: Some(Game::get_database_as_resource())
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        owner.set_process(true);
        owner.add_to_group("save_game_data", false);

        // Load the database and add it as a node
        let database = self.database.unwrap();
        owner.add_child(database, true);
        // Print the Database status, with tables, rows... Debug only
        self.debug_database_info(database);

        // Gets references to the core nodes of the game
        self.game_node = owner.get_node(".");
        self.world_map_node = owner.get_node("Map");

        // While the new values are coming, load the most recent saved (last one stored), avoiding nulling data
        let todays_date = utils::get_todays_date();
        self.game_external_data.todays_day_of_the_week = todays_date.0;
        self.game_external_data.todays_date = todays_date.2;

        // Deactivate de main game nodes when data isn't still retrieved from the REST Api's
        // Should this one better just as a grey or loading screen??
        unsafe { self.world_map_node.unwrap().assume_safe().cast::<Node2D>().unwrap().set_visible(false) };
        unsafe { owner.get_node_as::<Node2D>("Player").unwrap().set_visible(false) };

        // Loads all the availiable cities/towns in the game
        for game_city in GameCity::values() {
            let place = City::new(game_city.to_fmt_string(), None);
            self.game_cities.append(&mut vec![place]);
        }

        // Calls our Java Spring backend to retrieve the real tiem information
        self.get_external_game_data(owner);
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
        // godot_print!("CURRENT SCENE TYPE FROM PROCESS: {:?}", &self.current_scene_type);
        // Updates the counter that help to reduce the amount of times that a function gets triggered by this _process callback
        self.number_of_process += 1;
        // Resets the counter when designed with an arbitrary value
        if self.number_of_process > 1000 {
            self.number_of_process = 0
        }
        
        // 1º -> Notifies all the node that had info to persist that it's time to save that data
        if Input::is_action_just_pressed(&self.input.unwrap(), "Menu") {
            self.call_save_game_data_group(owner);
        }
        // 2º -> When all signals are safetly stored in the class attributes, just call the data persistence method
        if self.received_signals == self.total_registered_signals {
            self.save_game();
        }
        
        if !self.full_data_retrieved {
            let game_data = utils::retrieve_game_data();
            // ! This should be enable on development only!
            if self.game_external_data.spring_backend_response_code != 200 {
                godot_print!("OpenWeather API limit reached. Gonna use default data!");
                self.current_weather = Weather::Sun;
                self.game_external_data.todays_sunrise_time = "08:00:00".to_string(); // Handcoded values now
                self.game_external_data.todays_sunset_time = "21:32:50".to_string(); // IDEM
                self.game_external_data.cities_weather_loaded = true;
            } else {
                if self.number_of_process % 10 == 0 {
                    godot_print!("Esperando la respuesta del servidor...");
                }
            }

            // When data finally arrives after the above callbacks...
            if self.game_external_data.all_external_data_arrived() {
                // Sets the initial luminic and weather conditions
                self.control_day_phases(owner);
                // Loads the correct scene from where the player was the last time that saved the game
                self.load_initial_scene(owner, game_data.current_scene_path);
                // self.current_scene_type = game_data.current_scene_type;
                // This is where the loading screen should be working!!!
                unsafe { self.world_map_node.unwrap().assume_safe().cast::<Node2D>().unwrap().set_visible(true) };
                unsafe { owner.get_node_as::<Node2D>("Player").unwrap().set_visible(true) };
                // All data loaded, change the flag to avoid enter this piece of code
                self.full_data_retrieved = true;
                self.current_weather = Weather::Rain; //*! DEBUG!! Spawned manually to check rain conditions
                self.weather_control(owner);
                self.next_api_call = NaiveTime::from(utils::get_current_time().overflowing_add_signed(Duration::minutes(15)).0);
            } else {
                if self.number_of_process % 10 == 0 {
                    godot_print!("Aún no se han recuperado todos los datos...");
                }  
            }
        } else {
            // Reduces the nº of interactions, instead of every frame, every % of x
            if self.number_of_process % 100 == 0 {
                self.control_day_phases(owner);
                if utils::get_current_time() > self.next_api_call {
                    self.weather_control(owner)
                }
            }
        }
    } 

    #[export]
    fn control_day_phases(&mut self, owner: &Node2D) {
        if unsafe { self.world_map_node.unwrap().assume_safe().is_inside_tree() } {
            // Get's a reference to the CanvasModulate Day-Night simulator
            let day_night_node: TRef<CanvasModulate> = unsafe { owner.get_node_as::<CanvasModulate>("./Map/DayNight").unwrap() };

            // Current time
            let ctime: NaiveTime = utils::get_current_time();
            // godot_print!("CT from control_day_phases: {:?}", &ctime);
            // godot_print!("DayNightCycle: {:?}", &self.game_external_data.current_dn_cycle);

            // Sets the DayNightCycle to a concrete variant by comparing current time with another one...
            if ctime > NaiveTime::from_num_seconds_from_midnight(0, 0) && 
                !utils::time_comparator(ctime, &self.game_external_data.todays_sunrise_time) {
                    self.game_external_data.current_dn_cycle = DayNightCycle::Night;
            } else if utils::time_comparator(ctime, &self.game_external_data.todays_sunrise_time) && 
                !utils::time_comparator(ctime, &self.game_external_data.todays_sunset_time) {
                    self.game_external_data.current_dn_cycle = DayNightCycle::Day;
            } else if utils::time_comparator(ctime, &self.game_external_data.todays_sunset_time) &&
                ctime > NaiveTime::from_num_seconds_from_midnight(0, 0) {
                    self.game_external_data.current_dn_cycle = DayNightCycle::Night;
            }

            
            match self.game_external_data.current_dn_cycle {
                DayNightCycle::Day => { if self.current_weather == Weather::Rain {
                        day_night_node.set_deferred("color",Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 })
                    } else { // Setting when current weather is Weather::Sun-
                        day_night_node.set_deferred("color",Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })
                    }    
                },
                DayNightCycle::Night => day_night_node.set_deferred("color",Color { r: 0.2, g: 0.2, b: 0.3, a: 1.0 }),
                DayNightCycle::NoData => ()
            }  
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
            self.current_scene_type = CurrentSceneType::Indoors;

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
        } else {
            self.current_scene_type = CurrentSceneType::Outdoors;
        }
    }

    #[export]
    /// This method it's the receiver of the signal that notifies that the game detected the player on an area designed to switch him
    /// from the outside world to a building interior, and VICEVERSA
    fn change_world_scene(&mut self, owner: &Node2D, path: Variant) {
        // Gets a TRef to the Node that makes the transition between scenes animation
        let scene_transition_animation = unsafe { owner.get_node_as::<CanvasLayer>("SceneTransition")
            .unwrap().get_node("AnimationPlayer").unwrap().assume_safe().cast::<AnimationPlayer>().unwrap()
        };

        // Stores a path to a scene provided by a signal triggered for a collision between an area and a player
        self.current_scene_path = path.to_string();

        // Going from indoors to outdoors...
        if self.current_scene_path.ends_with("Map.tscn") {
            self.current_scene_type = CurrentSceneType::Outdoors;

            scene_transition_animation.play("FadeToBlack", -1.0, 0.5, false);

            unsafe { owner.call_deferred("remove_child", &[self.current_scene.unwrap().to_variant()]) };
            unsafe { owner.call_deferred("add_child", &[self.world_map_node.unwrap().to_variant()]) };
            
            scene_transition_animation.play("FadeToNormal", -1.0, 1.0, false);
        
        // Changing to an inside scene...
        } else {
            //Sets what type of scene it's being player now
            self.current_scene_type = CurrentSceneType::Indoors;
            
            // Plays the fade anim
            scene_transition_animation.play("FadeToBlack", -1.0, 0.5, false);
            
            // Now let's gonna remove the Map from the SceneTree
            unsafe { owner.call_deferred("remove_child", &[self.world_map_node.unwrap().to_variant()]) };

            // In order to go to a new scene, we must first load it as a resource
            let new_scene = ResourceLoader::godot_singleton()
                .load(path.to_string(), "", false).unwrap();

            // Convert the scene resource to a Node
            self.current_scene = unsafe { 
                new_scene.assume_safe().cast::<PackedScene>().unwrap().instance(0) };
            
            // Finally we insert our new Node, setting Game as it's parent
            unsafe { owner.call_deferred("add_child", &[self.current_scene.unwrap().to_variant()]) };
            
            // Gets back the screen without fades
            scene_transition_animation.play("FadeToNormal", -1.0, 1.0, false);
            
            // ! Normalize node position, relative to player. Here, for every new indoors scene that it's being player, 
            // automatically moves it (set it's position) taking the player position as reference and then modifing with certain offset.
            // Indoors scenes in Pokémon usually starts on a red carpet, spawing the player there, so basically we are moving the scene to fit that condition.
            let player_pos = unsafe { owner.get_node("Player").unwrap().assume_safe().cast::<KinematicBody2D>().unwrap().position() };
            unsafe { self.current_scene.unwrap().assume_safe().cast::<Node2D>().unwrap().set_position(
                player_pos - Vector2::new(192.5, 422.0)
            ) };
                
        }
    }


    // <--------------------------- HTTP ZONE CONTROL --------------------------------------->    

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

    /// Retrieves all the external game data, like city's weather and game's sunrise and sunset hours
    /// from our Java Spring backend server
    fn get_external_game_data(&self, owner: &Node2D) {
        let url: &'static str;
        
        if self.in_development { url = self.development_url } else { url = self.production_url }
        
        match self.new_http_node(owner, url, "_get_java_spring_backend_response")
        {
            Ok(response) => response,
            Err(err) => godot_print!("Err => {:?}", err)
        }
    }

    #[export]
    /// The method that receives and Http Response with all the external game data
    fn _get_java_spring_backend_response(&mut self, _owner: &Node2D, _result: Variant, _response_code: i64, _headers: Variant, body: ByteArray) {
        godot_print!("Spring backend response code: {:?}", _response_code);
        if _response_code == 200 {
            self.game_external_data.spring_backend_response_code = _response_code;

            // ! Here comes the http response, parsed as a Variant<Dictionary>
            let response = networking::http_body_to_string(body);

            self.set_sunrise_sunset_hours(&response);
            self.set_cities_weather(&response);
           
        } else {
            self.game_external_data.spring_backend_response_code = _response_code;
            godot_print!("FAILED to get backend information. Response code: {:?}", _response_code);
        }
    }

    // Encapsulates the process of sets the Sunrise/Sunset hours from the Http Response
    fn set_sunrise_sunset_hours(&mut self, response: &Dictionary) {
        let sunrise_hour = response.get("sunriseHour").to_string().parse::<i32>().unwrap();
        self.game_external_data.todays_sunrise_time = utils::convert_from_unix_timestamp(
            sunrise_hour + game_consts::UNIX_TIMESTAMP_OFFSET);

        let sunset_hour = response.get("sunsetHour").to_string().parse::<i32>().unwrap();
        self.game_external_data.todays_sunset_time = utils::convert_from_unix_timestamp(
            sunset_hour + game_consts::UNIX_TIMESTAMP_OFFSET);
    }

    // Encapsulates the process of sets the weather of all the cities of the game
    fn set_cities_weather(&mut self, response: &Dictionary) {
        let current_weather = response.get("gameCities").to_array();

        godot_print!("\nWEATHER: {:?}\n", &current_weather);

        // Iterate all over the game cities / towns
        let mut idx: i32 = 0;
        // ! IMPORTANT: Our REST API always send the cities ordered by ID. The `self.game_cities` attribute stores cities created
        // in base the order that the cities are hardcoded in the vector returned by the `GameCity::values()` associated fn.
        // That order maps the ID of the cities on the JSON.
        for location in self.game_cities.iter_mut() {
            let current_idx_data = current_weather.get(idx).to_dictionary();
            if location.get_name() == current_idx_data.get("name").to_string() {
            
                let external_weather_data = current_idx_data
                        .get("weather")
                        .to_dictionary();
                
                let location_weather_instance = CityWeather::new(
                    external_weather_data.get("weatherIDCode").to_i64() as i32,
                    external_weather_data.get("mainCode").to_string(),
                    external_weather_data.get("description").to_string(),
                    external_weather_data.get("icon").to_string()
                );

                location.set_weather(location_weather_instance);
                idx += 1;
                self.game_external_data.cities_weather_loaded = true;
            } else {
                godot_print!("Something went wrong retriving data and matching it with the correct city at a given index");
            }
        }
        godot_print!("\n************\nGAME CITIES and it's data: {:?}", &self.game_cities);
    }

    // <------------------------- WEATHER CONTROL ----------------------->
    #[export]
    fn weather_control(&mut self, owner: &Node2D) {
        if self.current_scene_type == CurrentSceneType::Outdoors {
            for location in self.game_cities.iter_mut() {
                godot_print!("City name as node path {:?}", &location.get_as_node_path());
                
                let mut node_path: String = "Map/".to_owned() + location.get_as_node_path().as_str() + "/Weather";
                godot_print!("Full final node path {:?}", &node_path);
                match location.get_weather().as_ref().unwrap().get_main_code_as_weather_variant() {
                    Weather::Thunderstorm => node_path.push_str("/Thunderstorm"),
                    Weather::Drizzle => node_path.push_str("/Drizzle"),
                    Weather::Rain => node_path.push_str("/Rain"),
                    Weather::Snow => node_path.push_str("/Snow"),
                    Weather::Sun => node_path.push_str("/Sun"),
                    Weather::Clouds => node_path.push_str("/Clouds"),
                };
                
                if let Some(weather_node) = unsafe { owner.get_node_as::<Particles2D>(node_path.as_str()) } {
                    weather_node.set_emitting(true);
                }
            }
        }   
    }

}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct GameExternalData {
    todays_date: String,
    todays_day_of_the_week: String,
    cities_weather_loaded: bool,
    spring_backend_response_code: i64,
    todays_sunrise_time: String,
    todays_sunset_time: String,
    current_dn_cycle: DayNightCycle,
}

impl GameExternalData {
    fn new() -> Self {
        Self {
            todays_date: "".to_string(),
            todays_day_of_the_week: "".to_string(),
            cities_weather_loaded: false,
            spring_backend_response_code: 200,
            todays_sunrise_time: "".to_string(),
            todays_sunset_time: "".to_string(),
            current_dn_cycle: DayNightCycle::NoData,
        }
    }

    /// Returns true if all of his attributes are not in the initial/default state, that means, when all the 
    /// REST Api calls to retrieve data are succesfully, and already stored data on this struct
    fn all_external_data_arrived(&self) -> bool {
        if !self.cities_weather_loaded || self.todays_sunrise_time == "" || self.todays_sunset_time == "" {
                false
            } else { true }
    }
}



#[derive(PartialEq, Clone, Debug, ToVariant, Serialize, Deserialize)]
pub enum CurrentSceneType {
    Indoors,
    Outdoors,
    Battle,
    NoData
}

impl Default for CurrentSceneType {
    fn default() -> Self { CurrentSceneType::NoData }
}


#[derive(PartialEq, Clone, Debug, ToVariant, Serialize, Deserialize)]
pub enum Weather {
    Thunderstorm, // 2xx
    Drizzle, // 3xx
    Rain, // 5xx 
    Snow, // 6xx
    Sun, // 800, called "Clear"
    Clouds // 8xx
 }

impl Default for Weather {
    fn default() -> Self { Weather::Sun }
}

impl Weather {
    // Returns a Vec<Weather> with all the Variants
    fn values(&self) -> Vec<Weather> {
        vec![Self::Thunderstorm, Self::Drizzle, Self::Rain, Self::Snow, Self::Sun, Self::Clouds]
    }

    // Associated fn that converts any weather as string to his equivalent Weather counterpart
    pub fn from_string<S: AsRef<str> + Into<String> + Display>(string: S) -> Weather {
        match string.as_ref() {
            "Thunderstorm" => Self::Thunderstorm,
            "Drizzle" => Self::Drizzle,
            "Rain" => Self::Rain,
            "Snow" => Self::Snow,
            "Sun" => Self::Sun,
            "Clouds" => Self::Clouds,
            _ => Default::default()
        }
    }

    // Given a Weather, returns his variant name as `&'static str`.
    pub fn to_str_slice(&self) -> &'static str {
        match self {
            Self::Thunderstorm => "Thunderstorm",
            Self::Drizzle => "Drizzle",
            Self::Rain => "Rain",
            Self::Snow => "Snow",
            Self::Sun => "Sun",
            Self::Clouds => "Clouds",
        }
    }

    // Returns the weather's main code as a str slice with the weather's name translated into spanish
    pub fn to_spanish_str(&self) -> &'static str {
        match self {
            Self::Thunderstorm => "Tormenta",
            Self::Drizzle => "Granizo",
            Self::Rain => "Lluvia",
            Self::Snow => "Nieve",
            Self::Sun => "Soleado",
            Self::Clouds => "Nublado",
        }
    }

    // Returns the weather's main code as a str slice with the weather's name translated into galician
    pub fn to_galician_str(&self) -> &'static str {
        match self {
            Self::Thunderstorm => "Tormenta",
            Self::Drizzle => "Granizo",
            Self::Rain => "Chuvia",
            Self::Snow => "Neve",
            Self::Sun => "Soleado",
            Self::Clouds => "Nublado",
        }
    }
}


#[derive(PartialEq, Clone, Debug, ToVariant, Serialize, Deserialize)]
pub enum DayNightCycle {
    Day,
    Night,
    NoData
}

impl Default for DayNightCycle {
    fn default() -> Self { DayNightCycle::NoData }
}