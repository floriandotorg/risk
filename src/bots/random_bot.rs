use rand::Rng;
use crate::game_state::{GameState, Move};

use super::Bot;

pub struct RandomBot;

impl Bot for RandomBot {
    fn make_move(&mut self, game_state: GameState) -> Move {
        let moves = game_state.legal_moves();
        moves[rand::thread_rng().gen_range(0..moves.len())]
    }
}

impl Default for RandomBot {
    fn default() -> Self {
        Self
    }
}
