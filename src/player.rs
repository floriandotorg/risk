use strum_macros::{EnumIter, EnumCount};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub const COUNT: usize = 2;

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
