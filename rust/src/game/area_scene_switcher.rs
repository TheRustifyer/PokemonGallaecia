use gdnative::prelude::*;
use gdnative::api::Area2D;

use crate::utils::utils;
use crate::game::code_abstractions::signals::RegisterSignal;
#[derive(Debug)]
pub enum WhereIsPlayer {
    Inside,
    Outside,
    Untracked
}

#[derive(NativeClass)]
#[inherit(Area2D)]
#[derive(Debug)]
#[register_with(Self::register_signal)]
/// Class that dinamycally forms a path to the next scene based on how Nodes are routed on the GodotEditor,
/// detects where the Area is located on the tree and automatically loads the scene relative to the event triggerd by the player
/// when he entered on any structure on the game.
pub struct AreaSceneSwitcher {
    scene_to_switch: String,
    parent_name: String,
    owner_node: String,
    
    player_in_out: WhereIsPlayer,
}

impl RegisterSignal<Self> for AreaSceneSwitcher {
    fn register_signal(_builder: &ClassBuilder<Self>) {
        _builder.add_signal( Signal {
            name: "scene_change",
            args: &[],
        });
    }
}

#[gdnative::methods]
impl AreaSceneSwitcher {
    
    fn new(_owner: &Area2D) -> Self {
        Self {
            scene_to_switch: String::from(""),
            parent_name: String::from(""),
            owner_node: String::from(""),
            player_in_out: WhereIsPlayer::Untracked,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        //
        owner.add_to_group("save_game_data", false);
        //
        self.connect_signal_to_root_node(owner);

        //Name of the parent object that this Area2D is attached
        self.parent_name = unsafe { owner.get_parent().unwrap().assume_safe().name().to_string() };

        // Name of the root node inside the scene where 'onwer' has been created!
        self.owner_node = unsafe { owner.owner().unwrap().assume_safe().name().to_string() };

        // Sets the attribute that stores the final full path to the new scene based on what area the player have entered!
        self.set_path_to_scene_to_switch();

        godot_print!("Path: {:?}", &self.owner_node);
    }

    fn set_path_to_scene_to_switch(&mut self) {
        if self.parent_name == "Exit" {
            self.scene_to_switch = "res://godot/Game/Map.tscn".to_string();
        } else {
            self.scene_to_switch = "res://godot/Game/WorldElements/".to_string() + 
                &self.owner_node + &"/Scenes".to_string() + &"/Interior".to_string() + 
                &self.parent_name + &".tscn".to_string();
        } 
    }

    #[export]
    fn _on_area2d_body_entered(&self, owner: &Area2D, _body: Variant) {
        owner.emit_signal("scene_change", &[self.scene_to_switch.to_owned().to_variant()]);
    }

    #[export]
    /// Connects the game data signal with the Game Node
    fn connect_signal_to_root_node(&self, owner: &Area2D) {
        let game = unsafe { owner.get_node("/root/Game").unwrap().assume_safe() };
        owner.connect("scene_change", game, "change_map",
            VariantArray::new_shared(), 0).unwrap();
    }
}