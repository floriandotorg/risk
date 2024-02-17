use rand::Rng;
use itertools::Itertools;

use crate::game_state::{GamePhase, GameState, Move};

use super::Bot;

pub struct RuleBasedBot;

impl Bot for RuleBasedBot {
    fn make_move(&self, game_state: GameState) -> Move {
        match game_state.phase() {
            GamePhase::Reinforce(_) => {
                let my_territories = game_state.territories_of_player(game_state.current_player());
                let mut scores = vec![0u32; my_territories.len()];
                for i in 0..my_territories.len() {
                    let territory = &my_territories[i];
                    scores[i] += (1.0 / territory.state().armies() as f64 * 10.0) as u32;
                    for neighbor in territory.territory().neighbors() {
                        let neighbor_territory = game_state.territory(neighbor);
                        if neighbor_territory.player() == game_state.current_player() {
                            scores[i] += 1;
                        }
                    }
                    if territory.state().armies() == u8::MAX {
                        scores[i] = 0;
                    }
                }

                Move::Reinforce { territory: my_territories[scores.iter().position_max().unwrap()].territory(), armies: 1 }
            }
            _ => {
                let moves = game_state.legal_moves();
                moves[rand::thread_rng().gen_range(0..moves.len())]
            }
        }
    }
}

impl Default for RuleBasedBot {
    fn default() -> Self {
        Self
    }
}
