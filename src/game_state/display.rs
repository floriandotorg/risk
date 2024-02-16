use std::fmt::{self, Display, Formatter};

use super::{GameState, Move};
use crate::territories::TERRITORIES;

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current player: {:?}", self.current_player)?;
        for (territory, state) in TERRITORIES.iter().zip(self.territories.iter()) {
            writeln!(f, "{:24} - {:?} {}", territory, state.player, state.troops)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "Pass"),
            Move::Reinforce(t, n) => write!(f, "Reinforce({} with {})", TERRITORIES[*t as usize], n),
            Move::Move(from, to, amount) => write!(f, "Move {} from {} to {}", amount, TERRITORIES[*from as usize], TERRITORIES[*to as usize]),
            Move::Attack(from, to, attacking, defending) => write!(f, "Attack {} with {} from {} defending with {}",  TERRITORIES[*to as usize], attacking, TERRITORIES[*from as usize], defending),
        }
    }
}
