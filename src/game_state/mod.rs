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

#[derive(Debug, Copy, Clone)]
struct TerritoryState {
    player: Player,
    troops: u8,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GamePhase {
    Reinforce,
    Attack,
    Fortify,
}

#[derive(Copy, Clone)]
pub struct GameState {
    current_player: Player,
    territories: [TerritoryState; TERRITORIES.len()],
    phase: GamePhase,
}

#[derive(PartialEq)]
pub enum Move {
    Pass,
    Reinforce(u8, u8),
    Move(u8, u8, u8),
    Attack(u8, u8, u8, u8)
}

pub mod initial_placement;
pub mod moves;
pub mod draw_map;
pub mod display;
