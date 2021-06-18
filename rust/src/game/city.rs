use gdnative::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, ToVariant, Serialize, Deserialize)]
pub enum GameCity {
    SantiagoDeCompostela,
    ACoruna,
    Ames,
}

impl GameCity {
    pub fn values() -> Vec<GameCity> {
        vec![Self::SantiagoDeCompostela, Self::ACoruna, Self::Ames]
    }

    pub fn to_fmt_string(&self) -> &'static str {
        match self {
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
}