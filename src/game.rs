use crate::game_state::{GameState, GameStateDuringInitialPlacement, Move};
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

impl<BotA: Bot, BotB: Bot> Game<BotA, BotB> {
    pub fn new(bot_a: BotA, bot_b: BotB) -> Self {
        Self { round: 0, bot_a, bot_b, game_state: GameStateDuringInitialPlacement::new().place_random().start() }
    }

    pub fn play_round(&mut self) -> Result<(Option<GameResult>, Vec<Move>), &'static str> {
        if self.game_state.is_finished() {
            return Ok((Some(GameResult::Win(self.game_state.current_player())), vec![]))
        }

        if self.round > 1000 {
            return Ok((Some(GameResult::Draw), vec![]))
        }

        let mut moves_played = vec![];
        let player = self.game_state.current_player();
        let bot: &dyn Bot = if player == Player::A { &self.bot_a } else { &self.bot_b };
        while self.game_state.current_player() == player && !self.game_state.is_finished() {
            let move_to_play = bot.make_move(self.game_state.clone());
            self.game_state = self.game_state.apply_move(&move_to_play)?;
            moves_played.push(move_to_play);
            if moves_played.len() > 100 {
                return Err("Too many moves played");
            }
        }

        self.round += 1;

        Ok((None, moves_played))
    }

    pub fn play_until_end(&mut self) -> Result<GameResult, &'static str> {
        let mut result: Option<GameResult> = None;
        while result.is_none() {
            result = self.play_round()?.0;
        }
        Ok(result.unwrap())
    }

    pub fn round(&self) -> u32 {
        self.round
    }
}
