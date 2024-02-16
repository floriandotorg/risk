extern crate image;

mod territories;
mod player;
mod game_state;

use game_state::GameStateDuringInitialPlacement;

fn main() {
    let state = GameStateDuringInitialPlacement::new().place_random().start();
    println!("{:?}", state.legal_moves());
    state.save_map().expect("Failed to draw map");
}
