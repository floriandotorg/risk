use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};

use crate::{player::Player, territories::Territory};

use super::{GamePhase, GameState, GameStateDuringInitialPlacement, TerritoryState, TerritoryStateDuringInitialPlacement};

impl GameStateDuringInitialPlacement {
    const STARTING_PLAYER: Player = Player::A;

    pub fn new() -> Self {
        Self {
            current_player: Self::STARTING_PLAYER,
            territories: vec![TerritoryStateDuringInitialPlacement { player: None, troops: 0 }; Territory::COUNT].try_into().unwrap()
        }
    }

    pub fn start(&self) -> GameState {
        let territories = self.territories.iter().map(|t| TerritoryState { player: t.player.unwrap(), troops: t.troops}).collect::<Vec<_>>();
        let current_player = Self::STARTING_PLAYER;
        let territories_of_starting_player = territories.iter().filter(|territory| territory.player == current_player).count();
        let number_of_reinforcements = GameState::number_of_reinforcements(territories_of_starting_player);
        GameState {
            current_player,
            territories: territories.try_into().unwrap(),
            phase: GamePhase::Reinforce(number_of_reinforcements),
        }
    }

    pub fn place_random(&self) -> GameStateDuringInitialPlacement {
        let mut territories = self.territories.clone();
        let mut active_player = self.current_player;
        let mut rng = rand::thread_rng();

        let mut troops = [0; Player::COUNT];
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
                        t.troops = 1;

                        troops[active_player as usize] += 1;
                        territories_per_player[active_player as usize] += 1;
                        break;
                    }

                    random_territory -= 1;
                }
            }
            active_player = active_player.next();
        }

        // Place remaining troops
        const TROOP_COUNT: usize = 40;

        for player in Player::iter() {
            let mut remaining_troops = TROOP_COUNT - troops[player as usize];
            let players_territories = territories_per_player[player as usize];

            while remaining_troops > 0 {
                let mut random_territory = rng.gen_range(0..players_territories);

                for t in territories.iter_mut() {
                    if t.player == Some(player) {
                        if random_territory == 0 {
                            t.troops += 1;
                            remaining_troops -= 1;
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
