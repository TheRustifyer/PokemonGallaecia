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

        pub fn check_credentials(username: &String, password: &String) -> (bool, bool) {

            let mut credentials_flag: (bool, bool) = (false, false);
            
            // Very trivial example to check the program flux
            // Obv this will be upgraded
            if username == "root" || username == "Root" {
                credentials_flag.0 = true;
            }
            if password == "root" || password == "Root" {
                credentials_flag.1 = true;
            }
            // Returns a tuple representing the checked status of each credential
            credentials_flag
        }

    }
}