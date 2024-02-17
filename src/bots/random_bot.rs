use rand::Rng;
use crate::game_state::{GameState, Move};

use super::Bot;

pub struct RandomBot;

impl Bot for RandomBot {
    fn make_move(&self, game_state: GameState) -> Move {
        let mut rng = rand::thread_rng();
        let moves = game_state.legal_moves();
        moves[rng.gen_range(0..moves.len())]
    }
}

impl Default for RandomBot {
    fn default() -> Self {
        Self
    }
}
