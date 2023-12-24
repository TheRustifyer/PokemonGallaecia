use std::fmt::{Display, Formatter, Result};

/// Represents the possible states of some character during gameplay.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
#[allow(dead_code)] // ! TODO provisional while rewriting
pub enum CharacterStatus {
    /// The character is stationary, not in motion.
    #[default] Idle = 0,
    /// The character is walking towards another 2D point on the map with uniform motion.
    Walking = 1,
    /// The character is running towards another 2D point on the map with higher velocity.
    Running = 2,
    /// The character is engaged in an interaction with another element on the map that has speech content.
    Interacting = 3,
}

impl From<i32> for CharacterStatus {
    fn from(value: i32) -> Self {
        match value {
            x if x == 0 => Self::Idle,
            x if x == 1 => Self::Walking,
            x if x == 2 => Self::Running,
            x if x == 3 => Self::Interacting,
            _ => panic!("Tried to create a `CharacterStatus` from an unknown discriminant")
        }
    }
}

impl Display for CharacterStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let status_str = match self {
            CharacterStatus::Idle => "Idle",
            CharacterStatus::Walking => "Walking",
            CharacterStatus::Running => "Running",
            CharacterStatus::Interacting => "Interacting",
        };
        write!(f, "{}", status_str)
    }
}
