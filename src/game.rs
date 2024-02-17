use std::fs::{self, create_dir_all};
use std::path::Path;

fn recreate_folder<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    if path.as_ref().exists() {
        fs::remove_dir_all(path.as_ref())?;
    }
    create_dir_all(path)
}

use crate::game_state::draw_map::DrawMapOptions;
use crate::game_state::{GameState, GameStateDuringInitialPlacement, Move, MoveApplyErr};
use crate::player::Player;
use crate::bots::Bot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win(Player),
    Draw,
}

pub struct Game<BotA: Bot, BotB: Bot> {
    round: u32,
    bot_a: BotA,
    bot_b: BotB,
    game_state: GameState,
}

#[derive(Clone)]
pub struct PlayOptions {
    filename: Option<String>,
    debug_output: bool,
}

impl Default for PlayOptions {
    fn default() -> Self {
        Self { filename: None, debug_output: false }
    }
}

impl PlayOptions {
    pub fn save_map_images(mut self, folder: &str) -> Self {
        self.filename = Some(folder.to_string());
        self
    }

    pub fn verbose(mut self) -> Self {
        self.debug_output = true;
        self
    }

}

impl<BotA: Bot, BotB: Bot> Game<BotA, BotB> {
    pub fn new(bot_a: BotA, bot_b: BotB) -> Self {
        Self { round: 0, bot_a, bot_b, game_state: GameStateDuringInitialPlacement::new().place_random().start() }
    }

    pub fn play_round(&mut self, options: PlayOptions) -> Result<(Option<GameResult>, Vec<Move>), MoveApplyErr> {
        if self.game_state.is_finished() {
            return Ok((Some(GameResult::Win(self.game_state.current_player())), vec![]))
        }

        if self.round >= 2000 {
            return Ok((Some(GameResult::Draw), vec![]))
        }

        if let Some(filename) = options.filename {
            self.game_state.draw_map(DrawMapOptions::default().filename(&filename)).expect("Could not save map to file");
        }

        if options.debug_output {
            println!("{}", self.game_state);
        }

        if options.debug_output {
            println!("Round {}", self.round + 1);
        }

        let mut moves_played = vec![];
        let player = self.game_state.current_player();
        let bot: &dyn Bot = if player == Player::A { &self.bot_a } else { &self.bot_b };
        while self.game_state.current_player() == player && !self.game_state.is_finished() {
            let move_to_play = bot.make_move(self.game_state.clone());
            if options.debug_output {
                println!("  {:?}", move_to_play);
            }
            self.game_state = *self.game_state.apply_move(&move_to_play).unwrap().results().first().unwrap().state();
            moves_played.push(move_to_play);
            if moves_played.len() > 100 {
                return Err(MoveApplyErr::TooManyMoves);
            }
        }

        self.round += 1;

        Ok((None, moves_played))
    }

    pub fn play_until_end(&mut self, options: &PlayOptions) -> Result<GameResult, MoveApplyErr> {
        if let Some(folder) = &options.filename {
            recreate_folder(folder).expect("Could not recreate folder");
        }

        let mut result: Option<GameResult> = None;
        while result.is_none() {
            let mut round_options = options.clone();
            if let Some(folder) = &options.filename {
                round_options.filename = Some(format!("./{}/{}.png", folder, self.round));
            }
            result = self.play_round(round_options)?.0;
        }
        Ok(result.unwrap())
    }

    pub fn round(&self) -> u32 {
        self.round
    }
}
