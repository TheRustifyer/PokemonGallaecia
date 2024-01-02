use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize, Serializer};


/// Represents the cardinal directions that a character can face in a 2D world view.
#[derive(PartialEq, Copy, Clone, Debug, Default, Deserialize)]
pub enum CharacterDirection {
    /// The character is facing South in the 2D world. This is the default variant.
    #[default] Downwards = 0,
    /// The character is facing North in the 2D world.
    Upwards = 1,
    /// The character is facing West in the 2D world.
    Left = 2,
    /// The character is facing East in the 2D world.
    Right = 3,
}

impl From<i32> for CharacterDirection {
    fn from(value: i32) -> Self {
        match value {
            x if x == 0 => Self::Downwards,
            x if x == 1 => Self::Upwards,
            x if x == 2 => Self::Left,
            x if x == 3 => Self::Right,
            _ => panic!("Tried to create a `CharacterStatus` from an unknown discriminant")
        }
    }
}

impl Into<i32> for CharacterDirection {
    fn into(self) -> i32 {
        self as i32
    }
}

impl Display for CharacterDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let direction_str = match self {
            CharacterDirection::Upwards => "North",
            CharacterDirection::Downwards => "South",
            CharacterDirection::Left => "West",
            CharacterDirection::Right => "East",
        };
        write!(f, "{}", direction_str)
    }
}

impl Serialize for CharacterDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            CharacterDirection::Upwards => serializer.serialize_unit_variant("CharacterDirection", 0, "Upwards"),
            CharacterDirection::Downwards => serializer.serialize_unit_variant("CharacterDirection", 1, "Downwards"),
            CharacterDirection::Left => serializer.serialize_unit_variant("CharacterDirection", 2, "Left"),
            CharacterDirection::Right => serializer.serialize_unit_variant("CharacterDirection", 3, "Right"),
        }
    }
}
