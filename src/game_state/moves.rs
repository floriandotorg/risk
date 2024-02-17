use strum::IntoEnumIterator;

use super::{GamePhase, GameState, Move, MoveApplyErr, NamedTerritoryState, TerritoryState};
use crate::{player::Player, territories::{Continent, Territory}};

impl GameState {
    pub fn number_of_reinforcements(territories: &[TerritoryState], player: Player) -> u8 {
        let territories_of_player = territories.iter().filter(|territory| territory.player == player).count();
        match territories_of_player {
            // switch when ready https://github.com/rust-lang/rust/issues/37854
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => 3,
            14 | 15 | 16 => 4,
            _ => 5
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        if self.is_finished() {
            return vec![];
        }

        let mut moves = Vec::new();

        let territories = self.territories_of_player(self.current_player);

        match self.phase {
            GamePhase::Reinforce(number_of_reinforcements) => {
                for armies in 1..=number_of_reinforcements {
                    for t in &territories {
                        moves.push(Move::Reinforce { territory: t.territory, armies });
                    }
                }
            }
            GamePhase::Attack | GamePhase::Fortify => {
                for territory in &territories {
                    if territory.state.armies < 2 {
                        continue;
                    }

                    for neighbor in territory.territory.neighbors() {
                        let neighbor_territory = self.territory(neighbor);

                        for armies in 1..territory.state.armies {
                            if neighbor_territory.player == self.current_player {
                                moves.push(Move::Fortify { from: territory.territory, to: neighbor, armies })
                            } else if self.phase == GamePhase::Attack {
                                for defending in 1..neighbor_territory.armies {
                                    moves.push(Move::Attack { from: territory.territory, to: neighbor, attacking: armies, defending })
                                }
                            }
                        }
                    }
                }
                moves.push(Move::Pass);
            }
        }

        moves
    }

    pub fn apply_move(&self, move_to_play: &Move) -> Result<GameState, MoveApplyErr> {
        if self.is_finished() {
            return Err(MoveApplyErr::GameFinished);
        }

        match move_to_play {
            Move::Pass => {
                let (next_phase, next_player) = match self.phase {
                    GamePhase::Reinforce(armies) => return Err(MoveApplyErr::MoveNotInPhase(Move::Pass, GamePhase::Reinforce(armies))),
                    GamePhase::Attack => (GamePhase::Fortify, self.current_player),
                    GamePhase::Fortify => {
                        let next_player = self.current_player.next();
                        let number_of_reinforcements = GameState::number_of_reinforcements(&self.territories, next_player);
                        (GamePhase::Reinforce(number_of_reinforcements), next_player)
                    },
                };
                Ok(GameState {
                    current_player: next_player,
                    territories: self.territories,
                    phase: next_phase
                })
            },
            Move::Reinforce { territory, armies } => {
                let number_of_reinforcements = match self.phase {
                    GamePhase::Reinforce(number_of_reinforcements) => number_of_reinforcements,
                    _ => return Err(MoveApplyErr::MoveNotInPhase(*move_to_play, self.phase)),
                };

                if *armies > number_of_reinforcements {
                    return Err(MoveApplyErr::TooManyReinforcements);
                }
                let remaining_reinforcements = number_of_reinforcements - *armies;
                let next_phase = match remaining_reinforcements {
                    0 => GamePhase::Attack,
                    _ => GamePhase::Reinforce(remaining_reinforcements)
                };

                let mut new_state = GameState {
                    current_player: self.current_player,
                    territories: self.territories,
                    phase: next_phase
                };
                new_state.add_armies(*territory, number_of_reinforcements as i16, true)?;
                Ok(new_state)
            },
            Move::Fortify { from, to, armies } => {
                if self.phase != GamePhase::Fortify && self.phase != GamePhase::Attack {
                    return Err(MoveApplyErr::MoveNotInPhase(*move_to_play, self.phase))
                }

                let mut new_state = self.clone();
                new_state.add_armies(*from, -(*armies as i16), true)?;
                new_state.add_armies(*to, *armies as i16, false)?;

                let next_player = self.current_player.next();
                let number_of_reinforcements = GameState::number_of_reinforcements(&self.territories, next_player);
                Ok(GameState {
                    current_player: next_player,
                    territories: new_state.territories,
                    phase: GamePhase::Reinforce(number_of_reinforcements)
                })
            },
            Move::Attack { from, to, attacking, defending } => {
                if self.phase != GamePhase::Attack {
                    return Err(MoveApplyErr::MoveNotInPhase(*move_to_play, self.phase))
                }

                let attacking_territory = self.territory(*from);
                let defending_territory = self.territory(*to);
                let conquer = attacking_territory.armies > defending_territory.armies;
                let mut new_state = self.clone();
                if conquer {
                    new_state.conquer(*to, *attacking)?;
                    new_state.add_armies(*from, -(*attacking as i16), true)?;
                } else {
                    new_state.add_armies(*from, -(*attacking as i16), true)?;
                }

                Ok(new_state)
            },
        }
    }

    fn territory(&self, territory: Territory) -> &TerritoryState {
        &self.territories[territory as usize]
    }

    fn add_armies(&mut self, territory: Territory, armies: i16, is_starting: bool) -> Result<(), MoveApplyErr> {
        let index = territory as usize;
        if self.territories[index].player != self.current_player {
            return Err(if is_starting { MoveApplyErr::FromTerritoryNotOwned } else { MoveApplyErr::ToTerritoryNotOwned });
        }
        let new_armies = self.territories[index].armies as i16 + armies;
        if new_armies <= 0 {
            return Err(MoveApplyErr::TooManyUnitsMoved);
        }
        self.territories[index].armies = new_armies as u8;
        Ok(())
    }

    fn conquer(&mut self, territory: Territory, armies: u8) -> Result<(), MoveApplyErr> {
        let index = territory as usize;
        if self.territories[index].player == self.current_player {
            return Err(MoveApplyErr::ToTerritoryOwned);
        }
        self.territories[index].player = self.current_player;
        self.territories[index].armies = armies;
        Ok(())
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
