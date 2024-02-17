use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};

use crate::{player::Player, territories::Territory};

use super::{GamePhase, GameState, GameStateDuringInitialPlacement, TerritoryState, TerritoryStateDuringInitialPlacement};

impl GameStateDuringInitialPlacement {
    const STARTING_PLAYER: Player = Player::A;

    pub fn new() -> Self {
        Self {
            current_player: Self::STARTING_PLAYER,
            territories: vec![TerritoryStateDuringInitialPlacement { player: None, armies: 0 }; Territory::COUNT].try_into().unwrap()
        }
    }

    pub fn start(&self) -> GameState {
        let territories = self.territories.iter().map(|t| TerritoryState { player: t.player.unwrap(), armies: t.armies}).collect::<Vec<_>>();
        let current_player = Self::STARTING_PLAYER;
        let mut state = GameState {
            current_player,
            territories: territories.try_into().unwrap(),
            phase: GamePhase::Reinforce(0),
        };
        state.phase = GamePhase::Reinforce(state.number_of_reinforcements(state.current_player()));
        state
    }

    pub fn place_random(&self) -> GameStateDuringInitialPlacement {
        let mut territories = self.territories.clone();
        let mut active_player = self.current_player;
        let mut rng = rand::thread_rng();

        let mut armies = [0; Player::COUNT];
        let mut territories_per_player = [0; Player::COUNT];

        // Place all players
        loop {
            let number_of_unclaimed_territories = territories.iter().filter(|t| t.player.is_none()).count();
            if number_of_unclaimed_territories < 1 {
                break;
            }

            let mut random_territory = rng.gen_range(0..number_of_unclaimed_territories);

            for t in territories.iter_mut() {
                if t.player.is_none() {
                    if random_territory == 0 {
                        t.player = Some(active_player);
                        t.armies = 1;

                        armies[active_player as usize] += 1;
                        territories_per_player[active_player as usize] += 1;
                        break;
                    }

                    random_territory -= 1;
                }
            }
            active_player = active_player.next();
        }

        // Place remaining armies
        const ARMIES_COUNT: usize = 40;

        for player in Player::iter() {
            let mut remaining_armies = ARMIES_COUNT - armies[player as usize];
            let players_territories = territories_per_player[player as usize];

            while remaining_armies > 0 {
                let mut random_territory = rng.gen_range(0..players_territories);

                for t in territories.iter_mut() {
                    if t.player == Some(player) {
                        if random_territory == 0 {
                            t.armies += 1;
                            remaining_armies -= 1;
                            break;
                        }

                        random_territory -= 1;
                    }
                }

                assert!(random_territory == 0);
            }
        }

        GameStateDuringInitialPlacement {
            current_player: active_player,
            territories
        }
    }
}
