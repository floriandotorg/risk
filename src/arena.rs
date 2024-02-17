use std::sync::mpsc::channel;
use std::fmt::{self};

use num_cpus;
use threadpool::ThreadPool;

use crate::game::{Game, GameResult, PlayOptions};
use crate::player::Player;
use crate::bots::Bot;

pub struct ArenaResult {
    results: Vec<GameResult>,
}

impl ArenaResult {
    fn new(results: Vec<GameResult>) -> Self {
        Self { results }
    }

    pub fn wins(&self, player: Player) -> u32 {
        self.results.iter().filter(|r| r == &&GameResult::Win(player)).count() as u32
    }

    pub fn draws(&self) -> u32 {
        self.results.iter().filter(|r| r == &&GameResult::Draw).count() as u32
    }

    pub fn winner(&self) -> Option<Player> {
        if self.wins(Player::A) > self.wins(Player::B) {
            Some(Player::A)
        } else if self.wins(Player::B) > self.wins(Player::A) {
            Some(Player::B)
        } else {
            None
        }
    }
}

impl fmt::Debug for ArenaResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wins (A): {:?}, Wins (B): {:?}, Draws: {:?}", self.wins(Player::A), self.wins(Player::B), self.draws())
    }
}

pub fn play_games<BotA: Bot + Default, BotB: Bot + Default>(games: u32) -> Result<ArenaResult, &'static str> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();
    for _ in 0..games {
        let tx = tx.clone();
        pool.execute(move|| {
            let mut game = Game::new(BotA::default(), BotB::default());
            tx.send(game.play_until_end(&PlayOptions::default()).unwrap()).expect("channel will be there waiting for the pool");
        });
    }

    Ok(ArenaResult::new(rx.iter().take(games as usize).collect::<Vec<_>>()))
}
