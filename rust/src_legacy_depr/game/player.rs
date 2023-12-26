use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde::ser::Serializer;

use gdnative::{api::RayCast2D, prelude::*};
use gdnative::api::{AnimatedSprite, KinematicBody2D};

use crate::{game::dialogue_box::DialogueBoxStatus};
use crate::game::code_abstractions::{
    character::{CharacterTileMovement, CharacterJump},
    signals::RegisterSignal
};

use crate::utils::utils;
use crate::utils::consts::in_game_constant;

use super::menu::menu::MenuStatus;


#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
/// This beautiful struct is the responsable of read the data coming from signals of all 
/// different PLAYER "classes", processing that data and store it on an external resource
/// where the data can persist
pub struct PlayerData {
    name: String,
    player_direction: PlayerDirection,
    player_position: HashMap<String, f64>,
}

impl PlayerData {

    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            player_direction: PlayerDirection::default(),
            player_position: HashMap::new(),
        }
    }

    pub fn set_player_direction(&mut self, player_current_direction: &PlayerDirection) {
        self.player_direction = player_current_direction.to_owned();
    }
    pub fn set_player_position(&mut self, x: f64, y: f64) {
        self.player_position.insert("x".to_owned(), x);
        self.player_position.insert("y".to_owned(), y);
    }
}


#[derive(GodotClass)]
#[class(base=KinematicBody2D)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PlayerCharacter {
    #[serde(skip)]
    // #[property]  // TODO Check this out
    player_status: PlayerStatus,
    #[serde(skip)]
    menu_status: MenuStatus,
    #[serde(skip)]
    dialogue_box_status: DialogueBoxStatus,

    // Keyboard Input as singleton ref
    #[serde(skip)]
    input: Option<&'static Input>,

    // Player Raycasts
    #[serde(skip)]
    blocking_raycast: Option<TRef<'static, RayCast2D>>, // Things that blocks the player movement
    #[serde(skip)]
    ledge_raycast: Option<TRef<'static, RayCast2D>>,
    // Player Shadow
    #[serde(skip)]
    player_shadow: Option<TRef<'static, Sprite>>,
    #[serde(skip)]
    landing_dust_effect_node: Option<TRef<'static, Node>>,
    #[serde(skip)]
    landing_dust_effect: Option<TRef<'static, AnimatedSprite>>,

    // Player Tile-based movement system (under development)
    #[serde(skip)]
    initial_position: Vector2, 
    #[serde(skip)]
    input_direction: Vector2, 
    #[serde(skip)]
    is_moving: bool, 
    #[serde(skip)]
    percent_move_to_next_tile: f64,
    #[serde(skip)]
    jumping_over_ledge: bool,
}

impl RegisterSignal<Self> for PlayerCharacter {
    fn register_signal(builder: &ClassBuilder<Self>) {
        // Indicates that the Player is moving
        builder.signal( "animate")
            .with_param_custom(
                SignalParam { 
                    name: GodotString::from_str("motion"),
                    default: Variant::new(&Vector2::new(0.0, 0.0)),
                    export_info: ExportInfo::new(VariantType::Vector2),
                    usage: PropertyUsage::DEFAULT,
                },
            ).done();

        // Indicates that the Player is interacting
        builder.signal( "player_interacting").done();
        builder.signal( "player_position").done();
        builder.signal( "player_moving").done();
        builder.signal("player_stopped").done();
    }
}

impl CharacterTileMovement<KinematicBody2D, Input> for PlayerCharacter {
    /// The fn that manages the player motion on the `Map`, and updates the `self.player_status: PlayerStatus`, 
    /// which represents the current variant of the player different status and behaviours. 
    fn process_player_input(&mut self, owner: &KinematicBody2D, input: &Input) {
        if self.input_direction.y == 0.0 {
            self.input_direction.x = Input::is_action_pressed(&input, "Right", false) as i32 as f32 - Input::is_action_pressed(&input, "Left", false) as i32 as f32; 
        }
        if self.input_direction.x == 0.0 {
            self.input_direction.y = Input::is_action_pressed(&input, "Down", false) as i32 as f32 - Input::is_action_pressed(&input, "Up", false) as i32 as f32;
        }
        if self.input_direction != Vector2::default() {
            self.initial_position = owner.global_position();
            self.is_moving = true;
        }
        // Check when the player press the `space bar` == "Interact" key binding. If the player isn't interacting with anything else
        // calls the `interact method`.
        if Input::is_action_just_pressed(self.input.unwrap(), "Interact", false) {
            if self.player_status != PlayerStatus::Interacting {
                if let Some(collider) = self.blocking_raycast.unwrap().get_collider() {
                    if let Some(interaction) = unsafe { collider.assume_safe().cast::<Node>() } {
                        self.interact(owner, interaction)
                    }
                }
            }
        } 
    }

    /// Moves the player 1 whole tile for every input command along a 2D surface
    fn tilemove_or_collide(&mut self, owner: &KinematicBody2D, delta: f32) {
        // Variable to store where the Raycast should point based on the player movement
        let raycast_vector_length_and_direction: Vector2 = self.input_direction * in_game_constant::TILE_SIZE / 2.0;
        // If we have Some(RayCast2D), then we can safetly operate over the pointer and set the Raycast casting direction and longitude
        self.blocking_raycast.unwrap().set_cast_to(raycast_vector_length_and_direction);
        self.blocking_raycast.unwrap().force_raycast_update();
        self.ledge_raycast.unwrap().set_cast_to(raycast_vector_length_and_direction);
        self.ledge_raycast.unwrap().force_raycast_update();

        if (self.ledge_raycast.unwrap().is_colliding() && self.input_direction == Vector2::new(0.0, 1.0)) || self.jumping_over_ledge {
            self.jump_over_ledge(owner, delta);
        } else if !self.blocking_raycast.unwrap().is_colliding() {
            self.move_character(owner, delta);
        } else {
            self.is_moving = false;
        }
    }

    /// Creates a `tile based` movement for the given Kinematic Body
    fn move_character(&mut self, owner: &KinematicBody2D, delta: f32) {
        // Increment the variable that tracks the position on the road between one tile and another
        self.percent_move_to_next_tile += in_game_constant::WALK_SPEED * delta as f64;
        // If the player already moved an entire tile...
        if self.percent_move_to_next_tile >= 1.0 {
            owner.set_global_position(self.initial_position + Vector2::new(in_game_constant::TILE_SIZE * self.input_direction.x, 
                in_game_constant::TILE_SIZE * self.input_direction.y));
            self.percent_move_to_next_tile = 0.0; // Set to zero to be ready for the next tile movement
            self.is_moving = false; // The player completed a whole step (moved one entire tile)
        // Else, sets the player position to a "somewhere-in-between" point
        } else {
            owner.set_global_position(self.initial_position + Vector2::new(in_game_constant::TILE_SIZE * self.input_direction.x * self.percent_move_to_next_tile as f32,
                in_game_constant::TILE_SIZE * self.input_direction.y * self.percent_move_to_next_tile as f32));
        }    
    }
}

impl CharacterJump<KinematicBody2D, Input> for PlayerCharacter {
    fn jump_over_ledge(&mut self, owner: &KinematicBody2D, delta: f32) {
        self.percent_move_to_next_tile += in_game_constant::JUMP_SPEED * delta as f64;
            // When jump, we want to cover a distance of 2 entire tiles
        if self.percent_move_to_next_tile >= 2.0 {
            // First, when player completes the jump, we should normalize the distance traveled by correcting the "jump simullator ecuation on the else case"
            let mut character_position: f32 = owner.position().y.ceil();
            while character_position % 16.0 != 0.0 { 
                character_position += 1.0;
            }
            // Now we can set the final point, where the player arrives after complete the jump
            owner.set_position(Vector2::new(owner.position().x, character_position));
            // Set back flags and trackers to default
            self.percent_move_to_next_tile = 0.0;
            self.is_moving = false;
            self.jumping_over_ledge = false;
            self.player_shadow.unwrap().set_visible(false);
            // Manages the landing effect
            self.landing_dust_effect(owner);

        } else {
            let jumping_input = in_game_constant::TILE_SIZE * self.input_direction.y * self.percent_move_to_next_tile as f32;
            let jump_simullator_ecuation = self.initial_position.y + (-0.96 - 0.53 * jumping_input + 0.05 * f32::powf(jumping_input, 2.0));
            self.jumping_over_ledge = true;
            owner.set_position(Vector2::new(owner.position().x,
            jump_simullator_ecuation.ceil()));
            self.player_shadow.unwrap().set_visible(true);
        }
    }

    fn landing_dust_effect(&mut self, owner: &KinematicBody2D) {
        let landing_dust_effect_node = unsafe { ResourceLoader::godot_singleton()
            .load("res://godot/Game/LandingDustEffect.tscn", "", false)
            .unwrap().assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .instance(0)
            .unwrap().assume_safe() };
        let landing_dust_effect = landing_dust_effect_node.cast::<AnimatedSprite>().unwrap();

        owner.add_child(landing_dust_effect, true);
        owner.move_child(landing_dust_effect, 0);
    }
}


#[methods]
impl PlayerCharacter {  

    /// The `PlayerCharacter` constructor
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            player_status: Default::default(),
            
            menu_status: MenuStatus::Closed,
            dialogue_box_status: DialogueBoxStatus::Inactive,

            input: Some(Input::godot_singleton()),

            blocking_raycast: None,
            ledge_raycast: None,
            player_shadow: None,
            landing_dust_effect_node: None,
            landing_dust_effect: None,

            // Tile movement system
            initial_position: Vector2::new(0.0, 0.0),
            input_direction: Vector2::new(0.0, 0.0),
            is_moving: false,
            percent_move_to_next_tile: 0.0,
            jumping_over_ledge: false,
        }
    }

    
    fn _ready(&mut self, owner: &KinematicBody2D) {
        // Adds the PlayerCharacter Node to the group that takes care about data persistence
        owner.add_to_group("save_game_data", false);
        
        // Retrieves the player absolute position from a JSON config file
        self.initial_position.x = utils::get_player_absolute_position().0;
        self.initial_position.y = utils::get_player_absolute_position().1;

        // Sets the retrieved position
        owner.set_global_position(Vector2::new(self.initial_position.x, self.initial_position.y));

        // Connect the Player Character with the Struct that takes care about process, manage and persist PlayerCharacter data
        self.connect_to_game_data(owner);

        // Sets the TRefs to the Raycast player nodes
        self.blocking_raycast = unsafe { owner.get_node_as::<RayCast2D>("BlockingRayCast") };
        self.ledge_raycast = unsafe { owner.get_node_as::<RayCast2D>("LedgeRayCast") };
        // Sets how long is the Vector that looks for collisions on the ledges Raycasts
        self.ledge_raycast.unwrap().set_cast_to(Vector2::new(0.0,  4.0));
        // Set the TRef to the player shadow
        self.player_shadow = unsafe { owner.get_node_as::<Sprite>("Shadow") };
        self.player_shadow.unwrap().set_visible(false); // The shadow it's only visible when the player it's jumping
    }

    
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        // Checks that the player it's able to move
        if self.player_status != PlayerStatus::Interacting {
            // Moving the player when an input is detected
            if self.is_moving == false {
                self.process_player_input(owner, self.input.unwrap())
            } else if self.input_direction != Vector2::default() {
                self.tilemove_or_collide(owner, delta);
            } else {
                self.is_moving = false;
            }
            // Calling the method that animates the sprite when the KinematicBody2D is moving
            self.animate_character(&owner);
        } else {
            // If player it's interacting, set the movement to zero...
            self.input_direction = Vector2::default();
            // Notifies the PlayerAnimation class that we are IDLE 'cause interaction
            self.animate_character(&owner); // <- Player interacting
        }
    }

    /// Method designed to act as an intermediary when some event blocks any action of the player.
    ///
    /// Ex:
    /// The player talking with some other character is an interaction. While it's happening, the player
    /// should not be moving or doing anything else that "reading the Dialogue Box" with the text that the interaction has.
    ///
    /// The info parameter just provides an String that contains info from the signal that will be used to match
    /// a certain behaviour with that provided String.
    
    fn handle_interaction(&mut self, _owner: &KinematicBody2D, signal_info: String) {
        // Get a full `slice` of the parameters in order to match it with a `classical` &str
        let signal_info = &signal_info[..];
        // Matching the signal extra data
        match signal_info {
            "on_dialogue" => {
                self.player_status = PlayerStatus::Interacting;
                self.is_moving = false;
                self.dialogue_box_status = DialogueBoxStatus::Active
            },
            "menu_active" => {
                self.player_status = PlayerStatus::Interacting;
                self.is_moving = false;
                self.menu_status = MenuStatus::Open
            },
            _ => {
                self.player_status = PlayerStatus::default();
                self.dialogue_box_status = DialogueBoxStatus::Inactive;
                self.menu_status = MenuStatus::Closed
            }
        }
    }

    /// The method for the "Interaction" behaviour of the `Player Character`.
    ///
    /// Retrieves the Node which is colliding with our player character. 
    ///
    /// If there's Some() collision, checks if the object are allowed to interact with the player.
    /// Sends a signal alerting that the player if the object has an "Interact" child.
    fn interact(&mut self, owner: &KinematicBody2D, coll_body: TRef<Node>) {
        //  Notifies the game that the player is interacting if true
        if self.is_valid_interaction(coll_body) {
            self.player_is_interacting(owner);
        }
    }

    /// Given a body that is colliding with the `Player Character`, checks if has an "Interaction" Node,
    /// that represents that the object holds data for the player, and the `PlayerStatus`, which has to currently be == `PlayerStatus::Interacting`
    ///
    /// If the required conditions are satisfied, returns true.
    /// 
    /// Remember that in Rust, `if` expressions without `else` evaluate to `()`
    fn is_valid_interaction(&self, coll_body: TRef<Node>) -> bool {
        if coll_body.has_node("Interact") && self.dialogue_box_status == DialogueBoxStatus::Inactive { 
            return true; 
        } else { return false; }
    }

    /// Send the "player interacting" custom signal, that alerts that the player is currently on `PlayerStatus::Interacting` state.
    fn player_is_interacting(&self, owner: &KinematicBody2D) {
        owner.emit_signal("player_interacting", &[]);
    }

    /// If the player character is moving, should be an animated representation.
    ///
    /// Emit the signal "animate" and send the current player motion data for the receivers
    
    fn animate_character(&self, owner: &KinematicBody2D) {
        owner.emit_signal("animate", &[self.input_direction.to_variant()]);
    }

    /// Connects the PlayerCharacter signal that transmits the current global position
    fn connect_to_game_data(&self, owner: &KinematicBody2D) {
        let receiver = unsafe { owner.get_node("/root/Game").unwrap().assume_safe() };
        owner.connect("player_position", receiver,
         "_save_player_position", VariantArray::new_shared(), 0).unwrap();
    }

    
    fn save_game_data(&self, owner: &KinematicBody2D) {
        owner.emit_signal("player_position", &[(self.initial_position.x, self.initial_position.y).to_variant()]);
    }
}

#[derive(GodotClass)]
#[class(base=AnimatedSprite)]
#[register_with(Self::register_signal)]
#[derive(Debug)]
pub struct PlayerAnimation {
    current_player_motion: PlayerStatus,
    current_player_direction: PlayerDirection,
    idle_player_direction: PlayerDirection
}

impl RegisterSignal<Self> for PlayerAnimation {
    fn register_signal(builder: &ClassBuilder<Self>) {
        // Indicates that the Player is moving
        builder.signal("player_direction").done();
    }
}

#[methods]
impl PlayerAnimation {
    fn new(_owner: &AnimatedSprite) -> Self {
        Self {
            current_player_motion: Default::default(),
            current_player_direction: Default::default(),
            idle_player_direction: Default::default()
        }
    }

    
    fn _ready(&mut self, base: &AnimatedSprite) {
        // Adds the PlayerCharacter Node to the group that takes care about data persistence
        base.add_to_group("save_game_data", false);

        self.idle_player_direction = utils::get_player_direction();

        match self.idle_player_direction {
            PlayerDirection::Downwards => { base.play("idle front", false); }
            PlayerDirection::Upwards => { base.play("idle back", false); }
            PlayerDirection::Left => { base.play("idle left", false); }
            PlayerDirection::Right => { base.play("idle right", false); }
        };

        // Connects with the Game class
        self.connect_to_game_data(base);
    }

    
    fn _on_player_animate(&mut self, base: &AnimatedSprite, _motion: Vector2) {
        
        let character_animated_sprite = unsafe { base.get_node_as::<AnimatedSprite>( ".") }.unwrap();

        match _motion {
            x if x.x > 0.0 => 
                { self.current_player_direction = PlayerDirection::Right; self.current_player_motion = PlayerStatus::Walking },

            x if x.x < 0.0 => 
                { self.current_player_direction = PlayerDirection::Left; self.current_player_motion = PlayerStatus::Walking }, 

            x if x.y < 0.0 => 
                { self.current_player_direction = PlayerDirection::Upwards; self.current_player_motion = PlayerStatus::Walking },
            
            x if x.y > 0.0 => 
                { self.current_player_direction = PlayerDirection::Downwards; self.current_player_motion = PlayerStatus::Walking },
            
            _ => 
                { self.current_player_motion = PlayerStatus::Idle }    
        }

        if self.current_player_motion == PlayerStatus::Idle {
            match self.idle_player_direction {
                PlayerDirection::Downwards => { character_animated_sprite.play("idle front", false); }
                PlayerDirection::Upwards => { character_animated_sprite.play("idle back", false); }
                PlayerDirection::Left => { character_animated_sprite.play("idle left", false); }
                PlayerDirection::Right => { character_animated_sprite.play("idle right", false); }
            }; 

        } else if self.current_player_direction == PlayerDirection::Right {
            character_animated_sprite.play("walk right", false);
            self.idle_player_direction = PlayerDirection::Right;

        } else if PlayerDirection::Left == self.current_player_direction {  
            character_animated_sprite.play("walk left", false);
            self.idle_player_direction = PlayerDirection::Left;

        } else if PlayerDirection::Downwards == self.current_player_direction {
            character_animated_sprite.play("walk downwards", false);
            self.idle_player_direction = PlayerDirection::Downwards;

        } else if PlayerDirection::Upwards == self.current_player_direction {
            character_animated_sprite.play("walk upwards", false);
            self.idle_player_direction = PlayerDirection::Upwards;
        }
    }

    /// Connects the PlayerCharacter signal with the Game class
    fn connect_to_game_data(&self, owner: &AnimatedSprite) {
        let receiver = unsafe { owner.get_node("/root/Game").unwrap().assume_safe() };
        owner.connect("player_direction", receiver,
            "_save_player_direction", VariantArray::new_shared(), 0).unwrap();
    }

    
    fn save_game_data(&self, base: &AnimatedSprite) {
        base.emit_signal("player_direction", &[self.idle_player_direction.to_variant()]);
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerStatus {
    Idle,
    Walking,
    // Running
    Interacting
}

impl Default for PlayerStatus {
    fn default() -> Self { PlayerStatus::Idle }
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub enum PlayerDirection {
    Upwards,
    Downwards,
    Left,
    Right,
}

impl Default for PlayerDirection {
    fn default() -> Self { PlayerDirection::Downwards }
}

impl Serialize for PlayerDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            PlayerDirection::Upwards => serializer.serialize_unit_variant("PlayerDirection", 0, "Upwards"),
            PlayerDirection::Downwards => serializer.serialize_unit_variant("PlayerDirection", 1, "Downwards"),
            PlayerDirection::Left => serializer.serialize_unit_variant("PlayerDirection", 2, "Left"),
            PlayerDirection::Right => serializer.serialize_unit_variant("PlayerDirection", 3, "Right"),
        }
    }
}