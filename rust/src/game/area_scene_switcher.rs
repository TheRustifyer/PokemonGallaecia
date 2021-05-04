use gdnative::prelude::*;
use gdnative::api::Area2D;

use crate::utils::utils;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[derive(Debug)]
/// Class that dinamycally forms a path to the next scene based on how Nodes are routed on the GodotEditor,
/// detects where the Area is located on the tree and automatically loads the scene relative to the event triggerd by the player
/// when he entered on any structure on the game.
pub struct AreaSceneSwitcher {
    scene_to_switch: String,
    parent_name: String,
    owner_node: String,
}

#[gdnative::methods]
impl AreaSceneSwitcher {
    
    fn new(_owner: &Area2D) -> Self {
        Self {
            scene_to_switch: String::from(""),
            parent_name: String::from(""),
            owner_node: String::from(""),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        //Name of the parent object that this Area2D is attached
        self.parent_name = unsafe { owner.get_parent().unwrap().assume_safe().name().to_string() };

        // Name of the root node inside the scene where 'onwer' has been created!
        self.owner_node = unsafe { owner.owner().unwrap().assume_safe().name().to_string() };

        // Sets the attribute that stores the final full path to the new scene based on what area the player have entered!
        self.set_path_to_scene_to_switch();
    }

    fn set_path_to_scene_to_switch(&mut self) {
        self.scene_to_switch = "res://godot/Game/WorldElements/".to_string() + 
            &self.owner_node + &"/Scenes/".to_string() + &"/Interior".to_string() + &self.parent_name + &".tscn".to_string();
    }

    #[export]
    fn _on_area2d_body_entered(&self, owner: &Area2D, _body: Variant) {
        utils::change_scene(owner, self.scene_to_switch.to_owned())
    }

}