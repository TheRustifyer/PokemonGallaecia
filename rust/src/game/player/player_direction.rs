use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize, Serializer};


/// Represents the cardinal directions in which the player-controlled character can face in a 2D world view.
#[derive(PartialEq, Clone, Debug, Default, Deserialize)]
pub enum PlayerDirection {
    /// The player character is facing South in the 2D world. This is the default variant.
    #[default] Downwards = 0,
    /// The player character is facing North in the 2D world.
    Upwards = 1,
    /// The player character is facing West in the 2D world.
    Left = 2,
    /// The player character is facing East in the 2D world.
    Right = 3,
}

impl Display for &PlayerDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let direction_str = match self {
            PlayerDirection::Upwards => "North",
            PlayerDirection::Downwards => "South",
            PlayerDirection::Left => "West",
            PlayerDirection::Right => "East",
        };
        write!(f, "{}", direction_str)
    }
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
