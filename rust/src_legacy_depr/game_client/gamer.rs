use gdnative::prelude::*;

/// Base class for that holds the user's account related data of the real person
///
/// This one allows to create new Gamer instances mapping the `client actions`: 
/// * account level -> Global level of the Gamer Player
/// * see your points/coins
/// * unlocked features
/// * choose avatars 
/// * track languages progress -> Player Character level
/// * character designs...
#[derive(Debug)]
pub struct Gamer {
    username: Option<String>,
    password: Option<String>,
    level: Option<i8>, // This should be a hash map that tracks language : level
}

impl Gamer {
    
    /// Method that login into the client a registered gamer
    pub fn gamer_login(
        username: String, 
        password: String, 
        level: i8) -> Self {

        let gamer: Gamer = Gamer { 
            username: Some(username), 
            password: Some(password), 
            level: Some(level),
        };
        gamer
    }

    pub fn check_credentials(username: Option<&String>, password: Option<&String>) -> (bool, bool) {

        let mut credentials_flag: (bool, bool) = (false, false);

        // Upgraded flat String credentials to std::option:Option, in order to use pattern matching
        //to make an ez way to scale future options when will be checked on a REST-backend
        match username {
            Some(usnm) if usnm == "root" || usnm == "Root" => credentials_flag.0 = true,
            Some(usnm) if usnm == "" => godot_print!("Provide an username"), // While insert an informative label as a child isn't implemented
            Some(_) => (),
            None => panic!(),
        }

        match password {
            Some(pswd) if pswd == "root" || pswd == "Root" => credentials_flag.1 = true,
            Some(pswd) if pswd == "" => godot_print!("Provide a password"),  // While insert an informative label as a child isn't implemented
            Some(_) => (),
            None => panic!() 
        }
        // Returns a tuple representing the checked status of each credential
        credentials_flag
    }

    /// Little method to convert the credentials (retrieved as a tuple of GodotStrings) into a tuple of Strings
    pub fn credentials_to_rust_string(cred_tup: (GodotString, GodotString)) -> (String, String) {
        let credentials = cred_tup;
        (credentials.0.to_string(), credentials.1.to_string())
    }
}
