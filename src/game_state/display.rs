use std::fmt::{self, Display, Formatter};

use super::{NamedTerritoryState, GameState, Move};

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for NamedTerritoryState { territory, state } in self.named_territories_iter() {
            writeln!(f, "{:24} - {:?} {}", territory, state.player, state.armies)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "Pass"),
            Move::Reinforce{ territory, armies } => write!(f, "Reinforce({} with {})", territory, armies),
            Move::Fortify{ from, to, armies } => write!(f, "Fortify({} from {} to {})", armies, from, to),
            Move::Attack{ from, to, attacking } => write!(f, "Attack(from {} with {} to {})", from, attacking, to),
        }
    }
}
