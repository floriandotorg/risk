use strum::IntoEnumIterator;

use super::{move_results::GameStateResults, GamePhase, GameState, Move, MoveApplyErr, NamedTerritoryState, TerritoryState};
use crate::{player::Player, territories::{Continent, Territory}};

impl GameState {
    fn continent_bonus(continent: Continent) -> u8 {
        match continent {
            Continent::NorthAmerica => 5,
            Continent::SouthAmerica => 2,
            Continent::Europe => 5,
            Continent::Africa => 3,
            Continent::Asia => 7,
            Continent::Oceania => 2,
        }
    }

    pub fn number_of_reinforcements(&self, player: Player) -> u8 {
        let territories_of_player = self.territories.iter().filter(|territory| territory.player == player).count();
        let mut from_territories = match territories_of_player {
            // switch when ready https://github.com/rust-lang/rust/issues/37854
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => 3,
            14 | 15 | 16 => 4,
            _ => 5
        };
        for continent in self.continents_for_player(player) {
            from_territories += GameState::continent_bonus(continent);
        }
        from_territories
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        if self.is_finished() {
            return vec![];
        }

        let mut moves = Vec::new();

        let territories = self.territories_states_of_player(self.current_player);

        match self.phase {
            GamePhase::Reinforce(number_of_reinforcements) => {
                for armies in 1..=number_of_reinforcements {
                    for t in &territories {
                        if t.state.armies as u16 + armies as u16 <= u8::MAX as u16 {
                            moves.push(Move::Reinforce { territory: t.territory, armies });
                        }
                    }
                }
            }
            GamePhase::Attack | GamePhase::Fortify => {
                for territory in &territories {
                    if territory.state.armies < 2 {
                        continue;
                    }

                    for neighbor in territory.territory.neighbors() {
                        let neighbor_territory = self.territory_state(neighbor);

                        for armies in 1..territory.state.armies {
                            if neighbor_territory.player == self.current_player {
                                if neighbor_territory.armies as u16 + armies as u16 <= u8::MAX as u16 {
                                    moves.push(Move::Fortify { from: territory.territory, to: neighbor, armies })
                                }
                            } else if self.phase == GamePhase::Attack && armies <= 3 {
                                moves.push(Move::Attack { from: territory.territory, to: neighbor, attacking: armies });
                            }
                        }
                    }
                }
                moves.push(Move::Pass);
            }
        }

        moves
    }

    pub fn apply_move(&self, move_to_play: &Move) -> Result<GameStateResults, MoveApplyErr> {
        if self.is_finished() {
            return Err(MoveApplyErr::GameFinished);
        }

        match move_to_play {
            Move::Pass => {
                let (next_phase, next_player) = match self.phase {
                    GamePhase::Reinforce(armies) => return Err(MoveApplyErr::MoveNotInPhase(Move::Pass, GamePhase::Reinforce(armies))),
                    GamePhase::Attack | GamePhase::Fortify => {
                        let next_player = self.current_player.next();
                        let number_of_reinforcements = self.number_of_reinforcements(next_player);
                        (GamePhase::Reinforce(number_of_reinforcements), next_player)
                    },
                };
                Ok(GameStateResults::single(GameState {
                    current_player: next_player,
                    territories: self.territories,
                    phase: next_phase
                }))
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
                new_state.add_armies(*territory, *armies as i16)?;
                Ok(GameStateResults::single(new_state))
            },
            Move::Fortify { from, to, armies } => {
                if self.phase != GamePhase::Fortify && self.phase != GamePhase::Attack {
                    return Err(MoveApplyErr::MoveNotInPhase(*move_to_play, self.phase))
                }

                let mut new_state = self.clone();
                new_state.add_armies(*from, -(*armies as i16))?;
                new_state.add_armies(*to, *armies as i16)?;

                let next_player = self.current_player.next();
                let number_of_reinforcements = self.number_of_reinforcements(next_player);
                Ok(GameStateResults::single(GameState {
                    current_player: next_player,
                    territories: new_state.territories,
                    phase: GamePhase::Reinforce(number_of_reinforcements)
                }))
            },
            Move::Attack { from, to, attacking } => {
                if self.phase != GamePhase::Attack {
                    return Err(MoveApplyErr::MoveNotInPhase(*move_to_play, self.phase))
                }
                if *attacking == 0 {
                    return Err(MoveApplyErr::ZeroUnitsInAttack);
                }

                let attacking_territory = self.territory_state(*from);
                let defending_territory = self.territory_state(*to);
                let conquer = attacking_territory.armies > defending_territory.armies;
                let mut new_state = self.clone();
                if conquer {
                    new_state.conquer(*to, *attacking)?;
                    new_state.add_armies(*from, -(*attacking as i16))?;
                } else {
                    new_state.add_armies(*from, -(*attacking as i16))?;
                }

                Ok(GameStateResults::single(new_state))
            },
        }
    }

    fn apply_move_hack(&self) -> GameState {
        // todo!("applying this move is not yet implemented");
        self.clone()
    }

    pub fn territory_state(&self, territory: Territory) -> &TerritoryState {
        &self.territories[territory as usize]
    }

    fn add_armies(&mut self, territory: Territory, armies: i16) -> Result<(), MoveApplyErr> {
        let index = territory as usize;
        if self.territories[index].player != self.current_player {
            return Err(if armies < 0 { MoveApplyErr::FromTerritoryNotOwned } else { MoveApplyErr::ToTerritoryNotOwned });
        }
        if armies == 0 {
            return Ok(())
        }
        let new_armies = self.territories[index].armies as i16 + armies;
        self.territories[index].armies = match new_armies.try_into() {
            Ok(new_armies) => new_armies,
            Err(_) => return Err(MoveApplyErr::TooManyUnitsMoved),
        };
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

    pub fn territories_states_of_player(&self, player: Player) -> Vec<NamedTerritoryState> {
        self.territories_iter().filter_map(|territory| if territory.state.player == player { Some(territory) } else { None }).collect()
    }

    pub fn continents_for_player(&self, player: Player) -> Vec<Continent> {
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

    pub fn phase(&self) -> GamePhase {
        self.phase
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumCount;

    use crate::{game_state::{GamePhase, GameState, Move, TerritoryState}, player::Player, territories::Territory};

    impl GameState {
        fn territory_mut(&mut self, territory: Territory) -> &mut TerritoryState {
            &mut self.territories[territory as usize]
        }
    }

    const TARGET_TERRITORY: Territory = Territory::Alaska;
    const SOURCE_TERRITORY: Territory = Territory::NorthwestTerritory;

    fn dummy_state(phase: GamePhase) -> GameState {
        let mut state = GameState {
            current_player: Player::A,
            territories: [TerritoryState {player: Player::A, armies: 1}; Territory::COUNT],
            phase,
        };
        state.territory_mut(TARGET_TERRITORY).player = Player::B;
        state.territory_mut(SOURCE_TERRITORY).armies = 10;
        state
    }

    #[test]
    fn attack_pass_skips() {
        let start = dummy_state(GamePhase::Attack);
        let result = start.apply_move(&Move::Pass);
        let result = result.unwrap();
        assert_eq!(result.results().len(), 1);
        let result = &result.results()[0];
        assert_eq!(result.count(), 1);
        assert_eq!(result.state().current_player, Player::B);
        assert!(matches!(result.state().phase, GamePhase::Reinforce(..)));
    }

    #[test]
    fn attack_other_player() {
        let start = dummy_state(GamePhase::Attack);
        let result = start.apply_move(&Move::Attack { from: SOURCE_TERRITORY, to: TARGET_TERRITORY, attacking: 1 });
        let result = result.unwrap();
        for state_result in result.results() {
            assert!(state_result.count() > 0);
            let state = state_result.state();
            assert_eq!(state.current_player, Player::A);
            assert_eq!(state.phase, GamePhase::Attack);
        }
    }
}
