use std::fmt::{self, Display, Formatter};

use super::GameState;
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
