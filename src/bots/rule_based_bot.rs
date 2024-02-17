use rand::Rng;
use itertools::Itertools;

use crate::game_state::{GamePhase, GameState, Move, NamedTerritoryState};

use super::Bot;

pub struct RuleBasedBot;

impl RuleBasedBot {
    fn random_move(&self, game_state: GameState) -> Move {
        let moves = game_state.legal_moves();
        moves[rand::thread_rng().gen_range(0..moves.len())]
    }
}

impl Bot for RuleBasedBot {
    fn make_move(&self, game_state: GameState) -> Move {
        match game_state.phase() {
            GamePhase::Reinforce(_) => {
                let my_territories = game_state.territories_states_of_player(game_state.current_player());
                let reinforceable_territories: Vec<_> = my_territories.iter().filter(|t| t.state().armies() < u8::MAX).collect();
                let mut scores = vec![0u32; reinforceable_territories.len()];
                for i in 0..reinforceable_territories.len() {
                    let territory = &reinforceable_territories[i];
                    scores[i] += (1.0 / territory.state().armies() as f64 * 10.0) as u32;
                    for neighbor in territory.territory().neighbors() {
                        let neighbor_territory = game_state.territory_state(neighbor);
                        if neighbor_territory.player() != game_state.current_player() {
                            scores[i] += 1 + ((neighbor_territory.armies() as f64 / territory.state().armies() as f64)) as u32;
                        }
                    }
                }

                if scores.len() < 2 {
                    return self.random_move(game_state);
                }

                Move::Reinforce { territory: reinforceable_territories[scores.iter().position_max().unwrap()].territory(), armies: 1 }
            }
            GamePhase::Attack => {
                let my_territories = game_state.territories_states_of_player(game_state.current_player());
                let mut possible_attacks = Vec::<(NamedTerritoryState, NamedTerritoryState)>::new();
                for territory in my_territories {
                    for neighbor in territory.territory().neighbors() {
                        let neighbor_territory_state = game_state.territory_state(neighbor);
                        if neighbor_territory_state.player() != game_state.current_player() && territory.state().armies() > 2 {
                            possible_attacks.push((territory, NamedTerritoryState::new(neighbor, neighbor_territory_state)));
                        }
                    }
                }
                if possible_attacks.is_empty() {
                    return Move::Pass;
                }
                let mut scores = vec![0u32; possible_attacks.len()];
                for (idx, (our_territory, enemy_territory)) in possible_attacks.iter().enumerate() {
                    scores[idx] += (our_territory.state().armies() as f64 / enemy_territory.state().armies() as f64) as u32

                }
                let best_score_idx = scores.iter().position_max().unwrap();
                if scores[best_score_idx] < 2 {
                    return Move::Pass;
                }
                let best_attack = possible_attacks[best_score_idx];
                Move::Attack { from: best_attack.0.territory(), to: best_attack.1.territory(), attacking: best_attack.0.state().armies() - 1 }
            }
            _ => {
                self.random_move(game_state)
            }
        }
    }
}

impl Default for RuleBasedBot {
    fn default() -> Self {
        Self
    }
}
