use std::ffi::OsStr;

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
}

#[derive(Debug, Clone)]
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
            main_code: main_code, 
            description: description, 
            icon: icon } 
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