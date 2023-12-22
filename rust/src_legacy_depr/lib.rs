use gdnative::prelude::*;

pub mod game;
pub mod game_client;
pub mod utils;

use game_client::login_screen::LoginScreen;

use game::game::Game;
use game::pokemon::Pokemon;
use game::pokemon_specie::PokemonSpecie;
use game::pokemon_database::PokemonDB;
use game::area_scene_switcher::AreaSceneSwitcher;
use game::tall_grass::TallGrass;
use game::grass_step_effect::GrassStepEffect;
use game::landing_dust_effect::LandingDustEffect;
use game::map::Map;
use game::player::{PlayerAnimation, PlayerCharacter};
use game::dialogue_box::DialogueBox;
use game::menu::menu::Menu;
use game::menu::pokedex::pokedex::Pokedex;

use game::map_elements__galicia::{
    area1_pueblo_de_teo,
    // area1_ames
};


fn init(handle: InitHandle) {
    // Here we register the Rust Structs that will register as classes on GDScript

    //First just will register a class that only prints a greet on the Godot Engine console
    //to ensure that all bindings, export and boilerplate stuff are done correctly
    handle.add_class::<LoginScreen>();
    handle.add_class::<PlayerCharacter>();
    handle.add_class::<PlayerAnimation>();

    handle.add_class::<Game>();
    handle.add_class::<Pokemon>();
    handle.add_class::<PokemonSpecie>();
    handle.add_class::<PokemonDB>();
    handle.add_class::<Map>();
    handle.add_class::<AreaSceneSwitcher>();
    handle.add_class::<TallGrass>();
    handle.add_class::<GrassStepEffect>();
    handle.add_class::<LandingDustEffect>();
    
    handle.add_class::<Menu>();
    handle.add_class::<DialogueBox>();
    handle.add_class::<Pokedex>();
    
    handle.add_class::<area1_pueblo_de_teo::truck::Truck>();
}

godot_init!(init);