use gdnative::prelude::*;

pub fn print_login_credentials(credentials_tup: (&String, &String)) {
    godot_print!("Username: {:?}", credentials_tup.0);
    godot_print!("Password: {:?}", credentials_tup.1);
}