use strum::IntoEnumIterator;

use super::{GamePhase, GameState, Move, MoveApplyErr, NamedTerritoryState, TerritoryState};
use crate::{player::Player, territories::{Continent, Territory}};

impl GameState {
    pub fn number_of_reinforcements(territories: usize) -> u8 {
        match territories {
            // switch when ready https://github.com/rust-lang/rust/issues/37854
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => 3,
            14 | 15 | 16 => 4,
            _ => 5
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let territories = self.territories_of_player(self.current_player);

        match self.phase {
            GamePhase::Reinforce(number_of_reinforcements) => {
                for armies in 1..number_of_reinforcements {
                    for t in &territories {
                        moves.push(Move::Reinforce { territory: t.territory, armies });
                    }
                }
            }
            GamePhase::Attack => {
                for territory in &territories {
                    if territory.state.armies < 2 {
                        continue;
                    }

                    for neighbor in territory.territory.neighbors() {
                        let neighbor_territory = self.territory(neighbor);
                        if neighbor_territory.player == self.current_player {
                            continue;
                        }

                        for attacking in 1..territory.state.armies {
                            for defending in 1..neighbor_territory.armies {
                                moves.push(Move::Attack { from: territory.territory, to: neighbor, attacking, defending })
                            }
                        }
                    }
                }
                moves.push(Move::Pass);
            }
            GamePhase::Fortify => {
                for territory in &territories {
                    if territory.state.armies < 2 {
                        continue;
                    }

                    for neighbor in territory.territory.neighbors() {
                        let neighbor_territory = self.territory(neighbor);
                        if neighbor_territory.player != self.current_player {
                            continue;
                        }

                        for armies in 1..territory.state.armies {
                            moves.push(Move::Fortify { from: territory.territory, to: neighbor, armies })
                        }
                    }
                }
                moves.push(Move::Pass);
            }
        }

        moves
    }

    pub fn apply_move(&self, move_to_play: &Move) -> Result<GameState, MoveApplyErr> {
        if !self.legal_moves().contains(move_to_play) {
            return Err(MoveApplyErr::IllegalMove);
        }
        Ok(self.apply_move_hack())
    }

    fn apply_move_hack(&self) -> GameState {
        // todo!("applying this move is not yet implemented");
        self.clone()
    }

    fn territory(&self, territory: Territory) -> &TerritoryState {
        &self.territories[territory as usize]
    }

    pub fn territories_iter<'a>(&'a self) -> impl Iterator<Item = NamedTerritoryState> {
        self.territories.iter().enumerate().map(|(i, t)| NamedTerritoryState { territory: Territory::try_from(i as u8).unwrap(), state: t })
    }

    fn territories_of_player(&self, player: Player) -> Vec<NamedTerritoryState> {
        self.territories_iter().filter_map(|territory| if territory.state.player == player { Some(territory) } else { None }).collect()
    }

    pub fn continents(&self, player: Player) -> Vec<Continent> {
        let mut result = Continent::iter().collect::<Vec<_>>();
        for NamedTerritoryState { territory, state } in self.territories_iter() {
            if state.player != player {
                let continent = territory.continent();
                result.retain(|&c| c != continent);
            }
        }
        result
    }

    pub fn is_finished(&self) -> bool {
        let player = self.territories.first().unwrap().player;
        self.territories.iter().all(|t| t.player == player)
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }
}
