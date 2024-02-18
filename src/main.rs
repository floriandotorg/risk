use game_state::GameStateDuringInitialPlacement;

extern crate image;

mod territories;
mod player;
mod game_state;
mod bots;
mod game;
mod arena;
mod evolver;

use bots::Bot;
use game::PlayOptions;
use game_state::{draw_map::DrawMapOptions, GamePhase};
use evolver::{Evaluator, EvaluationResult, Evolver};
use player::Player;

use bots::neural_bot::{NeuralBot, Float};
use bots::random_bot::RandomBot;
use bots::rule_based_bot::RuleBasedBot;

struct Eval<const LENGTH: usize>;

impl<const LENGTH: usize> Evaluator<LENGTH> for Eval<LENGTH> {
    fn initialize(&self) -> [Float; LENGTH] {
        NeuralBot::get_random_weights_and_biases().try_into().unwrap()
    }

    fn evaluate(&self, a: &[Float], b: &[Float]) -> EvaluationResult {
        let result = arena::play_games::<NeuralBot, NeuralBot, _, _>(20, || NeuralBot::from_weights_and_biases(a), || NeuralBot::from_weights_and_biases(b)).unwrap();
        match result.winner() {
            Some(winner) => {
                if winner == Player::A { EvaluationResult::A } else { EvaluationResult::B }
            }
            None => {
                return EvaluationResult::Draw;
            }
        }
    }
}

fn main() {
    let mut evolver: Evolver<_, { NeuralBot::LENGTH }, 10> = Evolver::new(Eval::<{ NeuralBot::LENGTH }> {});
    let mut best_genome;

    for g in 1..10 {
        println!("Generation: {}", g);

        best_genome = evolver.evolve_step();

        let results = arena::play_games::<RandomBot, NeuralBot, _, _>(100, || RandomBot {}, || NeuralBot::from_weights_and_biases(&best_genome));
        println!("Against Random Bot {:?}", results);

        let results = arena::play_games::<RuleBasedBot, NeuralBot, _, _>(100, || RuleBasedBot {}, || NeuralBot::from_weights_and_biases(&best_genome));
        println!("Against Rule Based Bot {:?}", results);
    }

    // let mut game = game::Game::new(RuleBasedBot {}, NeuralBot::from_weights_and_biases(&best_genome));
    // println!("{:?}", game.play_until_end(&PlayOptions::default().save_map_images("test").verbose()).unwrap());

    // let mut game = game::Game::new(RuleBasedBot {}, NeuralBot::default());
    // println!("{:?}", game.play_until_end(&PlayOptions::default().save_map_images("test").verbose()).unwrap());

    // let results = arena::play_games_with_default_bot_init::<bots::random_bot::RandomBot, bots::rule_based_bot::RuleBasedBot>(100);
    // println!("{:?}", results);

    // let mut state = GameStateDuringInitialPlacement::new().place_random().start();
    // let bot = NeuralBot::default();
    // // println!("{:?}",  bot.make_move(state));
    // while let GamePhase::Reinforce(_) = state.phase() {
    //     state = state.apply_move(&bot.make_move(state)).unwrap().random_state_by_probability();
    // }
    // state.draw_map(DrawMapOptions::default()).expect("Could not save map to file");
    // println!("{:?}",  bot.make_move(state));
}
