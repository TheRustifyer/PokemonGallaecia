pub mod player_mod {
    
    #[derive(Debug)]
    pub struct Player {
        username: String,
        password: String,
        level: u8
    }

    impl Player {  
        // The public constructor
        pub fn create_new_player(username: 
            String, password: String, level: u8) -> Self {

            let player: Player = Player { 
                username: username, 
                password: password, 
                level: level 
            };

            player
        }

        pub fn check_credentials(username: Option<&String>, password: Option<&String>) -> (bool, bool) {

            let mut credentials_flag: (bool, bool) = (false, false);
            
            // Very trivial example to check the program flux
            // Classical if-else block
            // if username == "root" || username == "Root" {
            //     credentials_flag.0 = true;
            // }
            // if password == "root" || password == "Root" {
            //     credentials_flag.1 = true;
            // }

            // Upgraded flat String redentials to std::option:Option, so pattern matching
            //to make an ez way to scale multiples options when will be checked on a REST-backend
            match username {
                Some(usnm) if usnm == "root" || usnm == "Root" => credentials_flag.0 = true,
                Some(_) => (),
                None => panic!(),
            }

            match password {
                Some(pswd) if pswd == "root" => credentials_flag.1 = true,
                Some(_) => (),
                None => panic!() 
            }
            // Returns a tuple representing the checked status of each credential
            credentials_flag
        }

    }
}