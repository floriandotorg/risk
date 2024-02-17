use strum::EnumCount;

use crate::player::Player;
use crate::territories::Territory;

#[derive(Debug, Clone)]
struct TerritoryStateDuringInitialPlacement {
    player: Option<Player>,
    armies: u8,
}

pub struct GameStateDuringInitialPlacement {
    current_player: Player,
    territories: [TerritoryStateDuringInitialPlacement; Territory::COUNT]
}

#[derive(Debug, Copy, Clone)]
struct TerritoryState {
    player: Player,
    armies: u8,
}

struct NamedTerritoryState<'a> {
    territory: Territory,
    state: &'a TerritoryState,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GamePhase {
    Reinforce(u8),
    Attack,
    Fortify,
}

#[derive(Copy, Clone)]
pub struct GameState {
    current_player: Player,
    territories: [TerritoryState; Territory::COUNT],
    phase: GamePhase,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Move {
    Pass,
    Reinforce { territory: Territory, armies: u8 },
    Move { from: Territory, to: Territory, armies: u8 },
    Attack { from: Territory, to: Territory, attacking: u8, defending: u8 }
}

pub mod initial_placement;
pub mod moves;
pub mod draw_map;
pub mod display;
