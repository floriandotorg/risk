use strum_macros::{EnumIter, EnumCount};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    pub fn color(self) -> (f64, f64, f64) {
        match self {
            Player::A => (0.0, 0.0, 1.0),
            Player::B => (1.0, 0.0, 0.0),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Player::A => write!(f, "Player A"),
            Player::B => write!(f, "Player B"),
        }
    }
}
