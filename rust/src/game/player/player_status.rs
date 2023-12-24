use std::fmt::{Display, Formatter, Result};

/// Represents the possible states of the player-controlled character during gameplay.
#[derive(PartialEq, Clone, Debug, Default)]
#[allow(dead_code)] // ! TODO provisional while rewriting
pub enum PlayerStatus {
    /// The player character is stationary, not in motion.
    #[default] Idle = 0,
    /// The player character is walking towards another 2D point on the map with uniform motion.
    Walking = 1,
    /// The player character is running towards another 2D point on the map with higher velocity.
    Running = 2,
    /// The player character is engaged in an interaction with another element on the map that has speech content.
    Interacting = 3,
}

impl Display for PlayerStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let status_str = match self {
            PlayerStatus::Idle => "Idle",
            PlayerStatus::Walking => "Walking",
            PlayerStatus::Running => "Running",
            PlayerStatus::Interacting => "Interacting",
        };
        write!(f, "{}", status_str)
    }
}
