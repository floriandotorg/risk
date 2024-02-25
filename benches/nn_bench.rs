use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::{Array, Array1, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use rand::{self, Rng};

pub type Float = f32;

// struct Layer {
//     biases: Vec<Float>,
//     weights: Vec<Vec<Float>>,
// }

// impl Layer {
//     fn new(neuron_count: usize, input_count: usize) -> Self {
//         let mut rng = rand::thread_rng();
//         let biases = (0..neuron_count).map(|_| rng.gen_range(0.0..1.0)).collect();
//         let weights = vec![(0..input_count).map(|_| rng.gen_range(0.0..1.0)).collect(); neuron_count];
//         Layer { biases, weights }
//     }
//     fn relu (x: Float) -> Float {
//         if x > 0.0 { x } else { 0.0 }
//     }

//     fn forward_propagate(&self, inputs: Vec<Float>) -> Vec<Float> {
//         self.biases.iter().enumerate().map(|(i, bias)| {
//             Self::relu(self.weights[i].iter().zip(inputs.iter()).map(|(w, &input)| w * input).sum::<Float>() + bias)
//         }).collect()
//     }
// }

// struct NeuralNetwork {
//     layers: Vec<Layer>,
// }

// impl NeuralNetwork {
//     pub fn new(layer_sizes: &[usize]) -> Self {
//         let mut layers = Vec::new();
//         for i in 0..layer_sizes.len() - 1 {
//             let layer = Layer::new(layer_sizes[i + 1], layer_sizes[i]);
//             layers.push(layer);
//         }
//         NeuralNetwork { layers }
//     }

//     pub fn forward_propagate(&self, inputs: &Vec<Float>) -> Vec<Float> {
//         let mut outputs = inputs.clone();
//         for layer in &self.layers {
//             outputs = layer.forward_propagate(outputs);
//         }
//         outputs
//     }

//     pub fn outputs(&self) {

//     }
// }

// struct Layer {
//     biases: Vec<Float>,
//     weights: Vec<Float>,

//     pub outputs: Vec<Float>,
// }

// impl Layer {
//     fn new(neuron_count: usize, input_count: usize) -> Self {
//         let mut rng = rand::thread_rng();
//         let biases = (0..neuron_count).map(|_| rng.gen_range(0.0..1.0)).collect();
//         let weights = (0..input_count * neuron_count).map(|_| rng.gen_range(0.0..1.0)).collect();
//         let outputs = vec![0.0; neuron_count];
//         Layer { biases, weights, outputs }
//     }

//     fn forward_propagate(&mut self, inputs: &Vec<Float>) {
//         let num_cols = inputs.len();
//         assert!(self.weights.len() % num_cols == 0, "Number of inputs does not match number of weights");
//         let num_rows = self.weights.len() / num_cols;
//         for row in 0..num_rows {
//             let mut sum = 0.0;
//             for col in 0..num_cols {
//                 sum += self.weights[num_cols * row + col] * inputs[col];
//             }
//             self.outputs[row] = Float::max(0.0, sum + self.biases[row]);
//         }
//     }
// }

// struct NeuralNetwork {
//     layers: Vec<Layer>,
// }

// impl NeuralNetwork {
//     pub fn new(layer_sizes: &[usize]) -> Self {
//         let mut layers = Vec::new();
//         for i in 0..layer_sizes.len() - 1 {
//             layers.push(Layer::new(layer_sizes[i + 1], layer_sizes[i]));
//         }
//         NeuralNetwork { layers }
//     }

//     pub fn forward_propagate(&mut self, inputs: &Vec<Float>) {
//         self.layers[0].forward_propagate(inputs);
//         for idx in 1..self.layers.len() {
//             let (prev, next) = self.layers.split_at_mut(idx);
//             next[0].forward_propagate(&prev[prev.len() - 1].outputs);
//         }
//     }

//     pub fn outputs(&self) -> &Vec<Float> {
//         &self.layers[self.layers.len() - 1].outputs
//     }
// }

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
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            let layer = Layer::new(layer_sizes[i + 1], layer_sizes[i]);
            layers.push(layer);
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

fn forward_propagate(inputs: &Array1<Float>) {
    let mut nn = NeuralNetwork::new(&vec![42, 20, 20, 83]);
    for _ in 0..100_000 {
        nn.forward_propagate(&inputs);
        nn.outputs();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // let inputs: Vec<Float> = (0..42).map(|_| rand::thread_rng().gen_range(0.0..1.0)).collect();
    let inputs: Array1::<Float> = Array::random(42, Uniform::new(0.0, 1.0));
    c.bench_function("nn", |b| b.iter(|| forward_propagate(&inputs)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

