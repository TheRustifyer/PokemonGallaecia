use gdnative::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::utils;

use super::game::Weather;

#[derive(PartialEq, Clone, Debug, ToVariant, Serialize, Deserialize)]
pub enum GameCity {
    CampoDePruebas,
    PuebloDeTeo,
    SantiagoDeCompostela,
    ACoruna,
    Ames,
}

impl GameCity {
    pub fn values() -> Vec<GameCity> {
        vec![Self::CampoDePruebas, Self::PuebloDeTeo, Self::SantiagoDeCompostela, Self::ACoruna, Self::Ames]
    }

    pub fn to_fmt_string(&self) -> &'static str {
        match self {
            Self::CampoDePruebas => "Campo de Pruebas",
            Self::PuebloDeTeo => "Pueblo de Teo",
            Self::SantiagoDeCompostela => "Santiago de Compostela",
            Self::ACoruna => "A Coruña",
            Self::Ames => "Ames"
        }
    }
}


#[derive(Debug, Clone)]
pub struct City {
    name: &'static str,
    pub weather: Option<CityWeather>
}

impl City {
    pub fn new(name: &'static str, weather: Option<CityWeather>) -> Self { Self { name, weather } }

    // Convert the name of the cities that has lower case prepositions
    pub fn get_as_node_path(&self) -> String {
        let mut location_name_to_node_path: String = String::new();
        
        self.name.to_string()
            .split(" ")
            .for_each(|word| {
                if word == "de" { 
                    location_name_to_node_path.push_str(
                        utils::uppercase_first_letter(
                            (word.chars().nth(0).unwrap().to_string() + "e").as_str()
                        ).as_str())
                } else {
                    location_name_to_node_path.push_str(word)
                }
            }
        );
        location_name_to_node_path
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_weather(&self) -> &Option<CityWeather> {
        &self.weather
    }

    pub fn set_weather(&mut self, weather: CityWeather) {
        self.weather = Some(weather);
    }

    /// Utilery function that ncapsulates the process of set the weather
    /// for all the cities of the game, given a container with the already
    /// generated instances of [`City`]
    pub fn set_cities_weather(
        game_cities: std::slice::IterMut<'_, City>,
        response: &Dictionary
    ) -> bool {
        let current_weather = response.get("gameCities")
            .expect("No gameCities entry")
            .to::<VariantArray>()
            .expect("Panic converting the gameCities entry to VariantArray");

        godot_print!("\nWEATHER: {:?}\n", &current_weather);

        // Iterate all over the game cities / towns
        let mut idx: i32 = 0;
        // ! IMPORTANT: Our REST API always send the cities ordered by ID. The `self.game_cities` attribute stores cities created
        // in base the order that the cities are hardcoded in the vector returned by the `GameCity::values()` associated fn.
        // That order maps the ID of the cities on the JSON.
        for location in game_cities {
            let current_idx_data = current_weather.get(idx)
                .to::<Dictionary>()
                .expect("Fail to get the current_idx_data");
            if location.get_name() == current_idx_data.get("name")
                .unwrap()
                .to_string() {
            
                let external_weather_data = current_idx_data
                    .get("weather")
                    .unwrap()
                    .to::<Dictionary>()
                    .unwrap();
                
                let location_weather_instance = CityWeather::new(
                    external_weather_data.get("weatherIDCode").unwrap().to::<i32>().unwrap(),
                    external_weather_data.get("mainCode").unwrap().to::<String>().unwrap(),
                    external_weather_data.get("description").unwrap().to::<String>().unwrap(),
                    external_weather_data.get("icon").unwrap().to::<String>().unwrap()
                );

                location.set_weather(location_weather_instance);
                return true;
            } else {
                godot_print!("Something went wrong retriving data and matching it with the correct city at a given index");
            }

            idx += 1;
        }

        return false;
    }
}

/// Data model for store the weather data received for a city
#[derive(Debug, Clone, Default)]
pub struct CityWeather {
    weather_id_code: i32,
    main_code: String,
    description: String,
    icon: String
}

impl CityWeather {
    pub fn new(weather_id_code: i32, main_code: String, description: String, icon: String) -> Self { 
        Self { 
            weather_id_code, 
            main_code, 
            description, 
            icon
        } 
    }

    pub fn get_weather_id_code(&self) -> i32 {
        self.weather_id_code
    }

    pub fn get_main_code(&'static self) -> &'static str {
        self.main_code.as_str()
    }

    pub fn get_main_code_as_weather_variant(&self) -> Weather {
        Weather::from_string(self.main_code.as_str())
    }

    pub fn get_description(&'static self) -> &'static str {
        self.description.as_str()
    }

    pub fn get_icon(&'static self) -> &'static str {
        self.icon.as_str()
    }
}