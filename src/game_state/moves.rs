use super::{GameState, Move};

impl GameState {
    pub fn legal_moves(&self) -> Vec<Move> {
        return Vec::new();
    }

    pub fn apply_move(&self, _move: &Move) -> GameState {
        return self.clone();
    }
}
