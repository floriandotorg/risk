extern crate image;

mod territories;
mod player;
mod game_state;
mod bots;
mod game;
mod arena;

fn main() {
    let mut game = game::Game::new(bots::random_bot::RandomBot {}, bots::random_bot::RandomBot {});
    println!("{:?}", game.play_until_end().unwrap());
}
