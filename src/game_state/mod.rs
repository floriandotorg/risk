use crate::player::Player;
use crate::territories::TERRITORIES;

#[derive(Debug, Clone)]
struct TerritoryStateDuringInitialPlacement {
    player: Option<Player>,
    troops: u8,
}

pub struct GameStateDuringInitialPlacement {
    current_player: Player,
    territories: [TerritoryStateDuringInitialPlacement; TERRITORIES.len()]
}

#[derive(Debug)]
struct TerritoryState {
    player: Player,
    troops: u8,
}

pub struct GameState {
  current_player: Player,
  territories: [TerritoryState; TERRITORIES.len()]
}

pub mod game_state;
pub mod draw_map;
