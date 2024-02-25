// #![allow(dead_code)]
// #![allow(unused_imports)]

extern crate image;

mod territories;
mod player;
mod game_state;
mod bots;
mod game;
mod arena;
mod evolver;

use game_state::GameStateDuringInitialPlacement;
use bots::Bot;
use game::PlayOptions;
use game_state::{draw_map::DrawMapOptions, GamePhase, GameState};
use evolver::{EvaluationResult, Evaluator, Evolver, ProbabilityMutator};
use player::Player;
use ndarray::{Array1, Array, ArrayView1};

use bots::neural_bot::{NeuralBot, Float};
use bots::random_bot::RandomBot;
use bots::rule_based_bot::RuleBasedBot;

use crate::evolver::transformations;
use crate::game::{Game, GameResult};

struct Eval<const LENGTH: usize>;

impl<const LENGTH: usize> Evaluator<LENGTH> for Eval<LENGTH> {
    fn initialize(&mut self) -> [Float; LENGTH] {
        NeuralBot::get_random_weights_and_biases().to_vec().try_into().unwrap()
    }

    fn evaluate(&self, a: &[Float], b: &[Float]) -> EvaluationResult {
        let result = arena::play_games::<NeuralBot, NeuralBot, _, _, _>(20, &eval_territory, || NeuralBot::from_weights_and_biases(&Array1::from(a.to_owned())), || NeuralBot::from_weights_and_biases(&Array1::from(b.to_owned()))).unwrap();
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

fn eval_territory(game_state: GameState) -> GameResult {
    if game_state.is_finished() {
        GameResult::Win(game_state.current_player())
    } else {
        let counts = game_state.territory_states().iter().fold((0, 0), |(a, b), t| {
            if t.player() == Player::A {
                (a + 1, b)
            } else {
                (a, b + 1)
            }
        });

        if counts.0 > counts.1 {
            GameResult::Win(Player::A)
        } else if counts.1 > counts.0 {
            GameResult::Win(Player::B)
        } else {
            GameResult::Draw
        }
    }
}

fn main() {
    let mut evolver: Evolver<_, ProbabilityMutator, { NeuralBot::LENGTH }, 40> = Evolver::with_transformation(Eval::<{ NeuralBot::LENGTH }> {}, Box::new(transformations::select));
    let mut best_genome;

    for g in 1..5 {
        println!("Generation: {}", g);

        evolver.set_mutator(Some(ProbabilityMutator { probability: 1.0/(g as f64 * 100.0) + 0.001, range: -1.0..1.0 }));

        best_genome = evolver.evolve_step();

        let genome = ArrayView1::from(&best_genome).to_owned();

        let results = arena::play_games::<RandomBot, NeuralBot, _, _, _>(100, &game::evaluate_win, || RandomBot {}, || NeuralBot::from_weights_and_biases(&genome));
        println!("Against Random Bot {:?}", results);

        let results = arena::play_games::<RuleBasedBot, NeuralBot, _, _, _>(100, &game::evaluate_win, || RuleBasedBot {}, || NeuralBot::from_weights_and_biases(&genome));
        println!("Against Rule Based Bot {:?}", results);
    }

    // let mut game = game::Game::new(RandomBot {}, NeuralBot::from_weights_and_biases(&best_genome));
    // println!("{:?}", game.play_until_end(&game::evaluate_win, &PlayOptions::default().save_map_images("test")).unwrap());

    // println!("{:?}", best_genome);

    // let mut game = game::Game::new(RuleBasedBot {}, NeuralBot::from_weights_and_biases(&best_genome));
    // println!("{:?}", game.play_until_end(&game::evaluate_win, &PlayOptions::default().save_map_images("test").verbose()).unwrap());

    // let mut game = game::Game::new(RuleBasedBot {}, NeuralBot::default());
    // println!("{:?}", game.play_until_end(&game::evaluate_win, &PlayOptions::default().save_map_images("test").verbose()).unwrap());

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
