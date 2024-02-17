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
pub struct TerritoryState {
    player: Player,
    armies: u8,
}

impl TerritoryState {
    pub fn player(&self) -> Player {
        self.player
    }

    pub fn armies(&self) -> u8 {
        self.armies
    }
}

pub struct NamedTerritoryState<'a> {
    territory: Territory,
    state: &'a TerritoryState,
}

impl NamedTerritoryState<'_> {
    pub fn territory(&self) -> Territory {
        self.territory
    }

    pub fn state(&self) -> &TerritoryState {
        self.state
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GamePhase {
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
    Fortify { from: Territory, to: Territory, armies: u8 },
    Attack { from: Territory, to: Territory, attacking: u8 }
}

#[derive(Debug)]
pub enum MoveApplyErr {
    MoveNotInPhase(Move, GamePhase),
    TooManyReinforcements,
    TooManyMoves,
    TooManyUnitsMoved,
    FromTerritoryNotOwned,
    ToTerritoryNotOwned,
    ToTerritoryOwned,
    GameFinished
}

pub mod initial_placement;
pub mod moves;
pub mod move_results;
pub mod draw_map;
pub mod display;
