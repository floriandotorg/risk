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

pub enum Move {
  Pass,
  Move(u8, u8),
  Attack(u8, u8)
}

pub mod initial_placement;
pub mod draw_map;
pub mod display;
