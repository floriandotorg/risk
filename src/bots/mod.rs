use crate::game_state::{GameState, Move};

pub trait Bot {
    fn make_move(&self, game_state: GameState) -> Move;
}

pub mod random_bot;
