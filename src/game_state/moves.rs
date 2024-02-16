use strum::IntoEnumIterator;

use super::{GameState, Move, GamePhase};
use crate::{player::Player, territories::{Continent, Territory}};

impl GameState {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let territories = self.territories_of_player(self.current_player);
        let number_of_reinforcements = match territories.len() {
            // switch when ready https://github.com/rust-lang/rust/issues/37854
            0 | 1 | 2 | 3 |4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => 3,
            14 | 15 | 16 => 4,
            _ => 5
        };

        if self.phase == GamePhase::Reinforce {
            for i in 1..number_of_reinforcements {
                for t in territories.iter() {
                    moves.push(Move::Reinforce(*t as u8, i));
                }
            }
        }

        if self.phase == GamePhase::Attack || self.phase == GamePhase::Fortify {
            moves.push(Move::Pass);
        }

        moves
    }

    pub fn apply_move(&self, move_to_play: &Move) -> Result<GameState, &'static str> {
        if !self.legal_moves().contains(move_to_play) {
            return Err("Illegal move");
        }
        return Ok(self.clone());
    }

    fn territories_of_player(&self, player: Player) -> Vec<usize> {
        self.territories.iter().enumerate().filter_map(|(i, t)| if t.player == player { Some(i) } else { None }).collect()
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
