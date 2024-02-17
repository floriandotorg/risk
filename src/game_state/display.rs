use std::fmt::{self, Display, Formatter};

use super::{NamedTerritoryState, GameState, Move};

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current player: {:?}", self.current_player)?;
        for NamedTerritoryState { territory, state } in self.territories_iter() {
            writeln!(f, "{:24} - {:?} {}", territory, state.player, state.troops)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "Pass"),
            Move::Reinforce{territory, armies} => write!(f, "Reinforce({} with {})", territory, armies),
            Move::Move{ from, to, armies} => write!(f, "Move {} from {} to {}", armies, from, to),
            Move::Attack{ from, to, attacking, defending} => write!(f, "Attack {} with {} from {} defending with {}",  from, attacking, to, defending),
        }
    }
}
