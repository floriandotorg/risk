use super::{GameState, Move};
use crate::player::Player;

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
}
