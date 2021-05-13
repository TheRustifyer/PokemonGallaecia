pub mod game_consts {
    pub const UNIX_TIMESTAMP_OFFSET: i32 = 3600;
}

pub mod in_game_constant {
    use gdnative::prelude::*;

    pub const VELOCITY: f32 = 100.0;
    // pub const GRAVITY: f32 = 300.0;
    pub const JUMP_SPEED: f32 = 3000.0;
    pub const UP: Vector2 = Vector2::new(0.0, -1.0);
}

pub mod game_options {
    // pub const SCREEN_SIZE: Vector2 = 
    //     Vector2::new(0.0, 0.0);
}

pub mod labels {
    pub const APP_TITLE_LABEL: &str = 
        "Learn Programming With Godot";
    pub const APP_TITLE_LABEL_PATH: &str = 
        "VBoxContainer/Label";
}

pub mod line_edit {
    /* Line edits are text input fields */
    pub const USERNAME_LINE_EDIT_PATH: &str =
        "VBoxContainer/HBoxContainer/UsernameInput";
    pub const PASSWORD_LINE_EDIT_PATH: &str =
        "VBoxContainer/HBoxContainer/PasswordInput";
}

pub mod scenes {
    // pub const MAIN_SCENE: &str =
    //     "";
    // pub const PLAYER_AS_SCENE: &str =
    //     "res://godot/Player.tscn";

    pub const LEVEL_1: &str =
        "res://godot/Levels/Level_1.tscn";
}

