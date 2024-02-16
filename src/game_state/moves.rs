use strum::IntoEnumIterator;

use super::{GameState, Move};
use crate::{player::Player, territories::{Continent, Territory}};

impl GameState {
    pub fn legal_moves(&self) -> Vec<Move> {
        return Vec::new();
    }

    pub fn apply_move(&self, _move: &Move) -> GameState {
        return self.clone();
    }

    pub fn territories_per_player(&self, player: Player) -> usize {
        self.territories.iter().filter(|t| t.player == player).count()
    }

    pub fn continents(&self, player: Player) -> Vec<Continent> {
        let mut result = Continent::iter().collect::<Vec<_>>();
        for (idx, territory_state) in self.territories.iter().enumerate() {
            let territory = Territory::try_from(idx as u8).unwrap();
            if territory_state.player != player {
                let continent = territory.continent();
                result.retain(|&c| c != continent);
            }
        }
        result
    }
}
