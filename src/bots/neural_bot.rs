use rand_distr::{Normal, Distribution};
use rand::thread_rng;
use rand::Rng;
use strum::EnumCount;
use itertools::Itertools;
use ndarray::{Array, Array1, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

use crate::bots::Bot;
use crate::game_state::{GamePhase, GameState, Move, NamedTerritoryState};
use crate::territories::{Territory, NEIGHBORS};

pub type Float = f32;
struct Layer {
    biases: Array1<Float>,
    weights: Array2<Float>,

    pub outputs: Array1<Float>,
}

impl Layer {
    fn new(neuron_count: usize, input_count: usize) -> Self {
        let biases = Array::random(neuron_count, Uniform::new(0.0, 1.0));
        let weights = Array::random((neuron_count, input_count), Uniform::new(0.0, 1.0));
        let outputs = Array::zeros(neuron_count);
        Layer { biases, weights, outputs }
    }

    fn forward_propagate(&mut self, inputs: &Array1<Float>) {
        self.outputs = self.weights.dot(inputs) + &self.biases;
        self.outputs.map_inplace(|x| *x = Float::max(0.0, *x));
    }
}

struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    fn generate_initialization_vector(layer_sizes: &[usize]) -> Array1<Float> {
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

        Array::from_vec(initialization_vector)
    }

    fn export(&self) -> Vec<Float> {
        let mut data = Vec::new();
        for layer in &self.layers {
            for bias in &layer.biases {
                data.push(*bias);
            }
            for weight in &layer.weights {
                data.push(*weight);
            }
        }
        data
    }

    fn load(&mut self, data: &Array1<Float>) {
        let mut idx = 0;
        for layer in &mut self.layers {
            for bias in &mut layer.biases {
                *bias = data[idx];
                idx += 1;
            }
            for weight in &mut layer.weights {
                *weight = data[idx];
                idx += 1;
            }
        }
    }

    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i + 1], layer_sizes[i]));
        }
        NeuralNetwork { layers }
    }

    pub fn forward_propagate(&mut self, inputs: &Array1<Float>) {
        self.layers[0].forward_propagate(inputs);
        for idx in 1..self.layers.len() {
            let (prev, next) = self.layers.split_at_mut(idx);
            next[0].forward_propagate(&prev[prev.len() - 1].outputs);
        }
    }

    pub fn outputs(&self) -> &Array1<Float> {
        &self.layers[self.layers.len() - 1].outputs
    }
}

pub struct NeuralBot {
    nn: NeuralNetwork,
}

impl NeuralBot {
    const ARCHITECTURE: [usize; 4] = [Territory::COUNT, 20, 20, NEIGHBORS.len()];
    pub const LENGTH: usize = Territory::COUNT * 20 + 20 * 20 + 20 * NEIGHBORS.len() + 20 + 20 + NEIGHBORS.len();

    pub fn from_weights_and_biases(data: &Array1<Float>) -> Self {
        let mut nn = NeuralNetwork::new(&Self::ARCHITECTURE);
        nn.load(data);
        Self { nn }
    }

    pub fn get_random_weights_and_biases() -> Array1<Float> {
        NeuralNetwork::generate_initialization_vector(&Self::ARCHITECTURE)
    }

    fn random_move(&self, game_state: GameState) -> Move {
        let moves = game_state.legal_moves();
        if moves.is_empty() {
            println!("{}", game_state);
            println!("NB: No legal moves");
        }
        moves[rand::thread_rng().gen_range(0..moves.len())]
    }
}

impl Bot for NeuralBot {
    fn make_move(&mut self, game_state: GameState) -> Move {
        match game_state.phase() {
            GamePhase::Attack => {
                let inputs = Array::from_iter(game_state.territory_states().iter().map(|t| t.armies() as Float / 256.0 * (if t.player() == game_state.current_player() { 1.0 } else { -1.0 })));

                let legal_moves = game_state.legal_moves();
                let named_territory_states: Vec<NamedTerritoryState> = game_state.named_territories_iter().collect();
                self.nn.forward_propagate(&inputs);
                let mut best_move = None;
                let outputs = &self.nn.outputs();
                for &best in (0..outputs.len()).collect::<Vec<usize>>().iter().sorted_by(|&a, &b| outputs[*b].partial_cmp(&outputs[*a]).unwrap()) {
                    if outputs[best] < 0.1 {
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
        let mut nn = NeuralNetwork::new(&Self::ARCHITECTURE);
        nn.load(&Self::get_random_weights_and_biases());
        Self { nn }
    }
}
