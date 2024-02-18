use rand_distr::{Normal, Distribution};
use rand::thread_rng;
use rand::Rng;
use strum::EnumCount;
use itertools::Itertools;

use crate::bots::Bot;
use crate::game_state::{GamePhase, GameState, Move, NamedTerritoryState};
use crate::territories::{Territory, NEIGHBORS};

pub type Float = f32;

struct Layer {
    biases: Vec<Float>,
    weights: Vec<Vec<Float>>,
}

impl Layer {
    fn new(neuron_count: usize, input_count: usize) -> Self {
        let biases = (0..neuron_count).map(|_| 0.0).collect();
        let weights = vec![vec![0.0; input_count]; neuron_count];
        Layer { biases, weights }
    }
}

struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    fn generate_initialization_vector(layer_sizes: &[usize]) -> Vec<Float> {
        let mut rng = thread_rng();
        let mut initialization_vector = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            let n_in = layer_sizes[i];
            let n_out = layer_sizes[i + 1];
            let var = 2.0 / (n_in as Float + n_out as Float); // Xavier initialization variance
            let normal_dist = Normal::new(0.0, var.sqrt()).unwrap();

            initialization_vector.extend(vec![0.0; n_out]);

            for _ in 0..n_out {
                for _ in 0..n_in {
                    let weight = normal_dist.sample(&mut rng);
                    initialization_vector.push(weight);
                }
            }
        }

        initialization_vector
    }

    fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            let layer = Layer::new(layer_sizes[i + 1], layer_sizes[i]);
            layers.push(layer);
        }
        NeuralNetwork { layers }
    }

    fn forward_propagate(&self, inputs: Vec<Float>) -> Vec<Float> {
        let mut outputs = inputs;
        for layer in &self.layers {
            outputs = layer.forward_propagate(outputs);
        }
        outputs
    }

    fn export(&self) -> Vec<Float> {
        let mut data = Vec::new();
        for layer in &self.layers {
            for bias in &layer.biases {
                data.push(*bias);
            }
            for row in &layer.weights {
                for &weight in row {
                    data.push(weight);
                }
            }
        }
        data
    }

    fn load(&mut self, data: &[Float]) {
        let mut idx = 0;
        for layer in &mut self.layers {
            for bias in &mut layer.biases {
                *bias = data[idx];
                idx += 1;
            }
            for row in &mut layer.weights {
                for weight in row.iter_mut() {
                    *weight = data[idx];
                    idx += 1;
                }
            }
        }
    }
}

impl Layer {
    fn relu (x: Float) -> Float {
        if x > 0.0 { x } else { 0.0 }
    }

    fn forward_propagate(&self, inputs: Vec<Float>) -> Vec<Float> {
        self.biases.iter().enumerate().map(|(i, bias)| {
            Self::relu(self.weights[i].iter().zip(inputs.iter()).map(|(w, &input)| w * input).sum::<Float>() + bias)
        }).collect()
    }
}

pub struct NeuralBot {
    nn: NeuralNetwork,
}

impl NeuralBot {
    fn random_move(&self, game_state: GameState) -> Move {
        let moves = game_state.legal_moves();
        moves[rand::thread_rng().gen_range(0..moves.len())]
    }
}

impl Bot for NeuralBot {
    fn make_move(&self, game_state: GameState) -> Move {
        match game_state.phase() {
            GamePhase::Attack => {
                let inputs = game_state.territory_states().iter().map(|t| t.armies() as Float / 256.0 * (if t.player() == game_state.current_player() { 1.0 } else { -1.0 })).collect();

                let legal_moves = game_state.legal_moves();
                let named_territory_states: Vec<NamedTerritoryState> = game_state.named_territories_iter().collect();
                let outputs = self.nn.forward_propagate(inputs);
                let mut best_move = None;
                for &best in (0..outputs.len()).collect::<Vec<usize>>().iter().sorted_by(|&a, &b| outputs[*b].partial_cmp(&outputs[*a]).unwrap()) {
                    if outputs[best] < 0.3 {
                        best_move = Some(Move::Pass);
                        break;
                    }

                    let (a, b) = NEIGHBORS[best];
                    let state_a = named_territory_states[a as usize];
                    let state_b = named_territory_states[b as usize];
                    let from = if state_a.state().player() == game_state.current_player() { state_a } else { state_b };
                    let to = if state_a.state().player() == game_state.current_player() { state_b } else { state_a };
                    let possible_move = Move::Attack { from: from.territory(), to: to.territory(), attacking: from.state().armies() - 1 };
                    if legal_moves.contains(&possible_move) {
                        best_move = Some(possible_move);
                        break;
                    }
                }

                if best_move.is_none() {
                    return self.random_move(game_state);
                }

                best_move.unwrap()
            }
            _ => self.random_move(game_state)
        }
    }
}

impl Default for NeuralBot {
    fn default() -> Self {
        let architecture = vec![Territory::COUNT, 20, 20, NEIGHBORS.len()];
        let mut nn = NeuralNetwork::new(&architecture);
        nn.load(&NeuralNetwork::generate_initialization_vector(&architecture));
        Self { nn }
    }
}
