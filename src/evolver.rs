use std::ops::Range;

use rand::Rng;

use crate::bots::neural_bot::Float;

#[derive(PartialEq, Eq, Debug)]
pub enum EvaluationResult {
    A,
    B,
    Draw,
}

pub trait Evaluator<const LENGTH: usize> {
    fn initialize(&mut self) -> [Float; LENGTH];
    fn evaluate(&self, a: &[Float], b: &[Float]) -> EvaluationResult;
}

#[derive(Clone, Copy, Debug)]
struct GenomeStats<const LENGTH: usize> {
    genome: [Float; LENGTH],
    fitness: usize,
}

pub mod transformations {
    use rand::Rng;

    use crate::bots::neural_bot::Float;

    pub fn average<const LENGTH: usize>(a: &[Float; LENGTH], b: &[Float; LENGTH]) -> [Float; LENGTH] {
        let mut result = [0.0; LENGTH];
        for idx in 0..LENGTH {
            result[idx] = (a[idx] + b[idx]) / 2.0;
        }
        result
    }

    pub fn select<const LENGTH: usize>(a: &[Float; LENGTH], b: &[Float; LENGTH]) -> [Float; LENGTH] {
        let mut rng = rand::thread_rng();
        let mut result = [0.0; LENGTH];
        for idx in 0..LENGTH {
            result[idx] = match rng.gen_bool(0.5) {
                true => a[idx],
                false => b[idx],
            };
        }
        result
    }

    pub fn select_half<const LENGTH: usize>(a: &[Float; LENGTH], b: &[Float; LENGTH]) -> [Float; LENGTH] {
        let mut result = [0.0; LENGTH];
        let mid = LENGTH / 2;
        let (left, right) = result.split_at_mut(mid);
        left.copy_from_slice(&a[0..mid]);
        right.copy_from_slice(&b[mid..]);
        result
    }
}

pub trait Mutator {
    fn mutate(&self, genome: &mut [Float]);
}

pub struct ProbabilityMutator {
    pub range: Range<Float>,
    pub probability: f64,
}

impl Mutator for ProbabilityMutator {
    fn mutate(&self, genome: &mut [Float]) {
        let mut rng = rand::thread_rng();
        for value in genome {
            if rng.gen_bool(self.probability) {
                *value = rng.gen_range(self.range.clone());
            }
        }
    }
}

pub struct Evolver<E, M, const LENGTH: usize, const POPULATION: usize>
where E: Evaluator<LENGTH>, M: Mutator {
    population: [GenomeStats<LENGTH>; POPULATION],
    evaluator: E,
    mutator: Option<M>,
    transformation: Box<dyn Fn(&[Float; LENGTH], &[Float; LENGTH]) -> [Float; LENGTH]>
}

impl<E, M, const LENGTH: usize, const POPULATION: usize> Evolver<E, M, LENGTH, POPULATION>
where E: Evaluator<LENGTH>, M: Mutator {
    pub fn new(evaluator: E) -> Self {
        Evolver::with_transformation(evaluator, Box::new(transformations::average))
    }

    pub fn with_transformation(evaluator: E, transformation: Box<dyn Fn(&[Float; LENGTH], &[Float; LENGTH]) -> [Float; LENGTH]>) -> Self {
        let mut evaluator = evaluator;
        let mut population = Vec::with_capacity(POPULATION);
        for _ in 0..POPULATION {
            population.push(GenomeStats { genome: evaluator.initialize(), fitness: 0 });
        }
        let population: [GenomeStats<LENGTH>; POPULATION] = population.try_into().unwrap();
        Evolver { population, evaluator, mutator: None, transformation }
    }

    pub fn mutator(&self) -> &Option<M> {
        &self.mutator
    }

    pub fn set_mutator(&mut self, mutator: Option<M>) {
        self.mutator = mutator;
    }

    pub fn evolve_step(&mut self) -> [Float; LENGTH] {
        for state in &mut self.population {
            state.fitness = 0;
        }

        for idx_a in 0..(POPULATION - 1) {
            for idx_b in (idx_a + 1)..POPULATION {
                let idx_winner = match self.evaluator.evaluate(&self.population[idx_a].genome, &self.population[idx_b].genome) {
                    EvaluationResult::A => idx_a,
                    EvaluationResult::B => idx_b,
                    EvaluationResult::Draw => continue,
                };
                self.population[idx_winner].fitness += 1
            }
        }

        self.population.sort_by(|a, b| a.fitness.cmp(&b.fitness).reverse());
        let best_genome = self.population[0].genome;


        // Mutate everyone randomly
        if let Some(mutator) = &self.mutator {
            let threshold = POPULATION * 2 / 10;
            let mut rng = rand::thread_rng();
            for idx in 0..threshold {
                self.population[threshold + idx] = self.population[threshold];
                mutator.mutate(&mut self.population[threshold + idx].genome);
            }
            for idx in (threshold + threshold)..POPULATION {
                // Select two "winners" and apply some transformation
                let winner_a = rng.gen_range(0..threshold);
                let winner_b = winner_a + rng.gen_range(0..(threshold - 1));
                let winner_b = winner_b % threshold;
                let mut new_genome = self.transformation.as_ref()(&self.population[winner_a].genome, &self.population[winner_b].genome);
                mutator.mutate(&mut new_genome);
                self.population[idx].genome = new_genome;
            }
        }

        best_genome
    }
}


#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{bots::neural_bot::Float, evolver::ProbabilityMutator};

    use super::{EvaluationResult, Evaluator, Evolver};

    struct MaxEvaluator<const LENGTH: usize>;

    impl<const LENGTH: usize> MaxEvaluator<LENGTH> {
        fn evaluate_single(values: &[Float]) -> Float {
            values.iter().filter(|&v| *v >= 0.0 && *v <= 1.0).sum()
        }
    }

    impl<const LENGTH: usize> Evaluator<LENGTH> for MaxEvaluator<LENGTH> {
        fn initialize(&mut self) -> [Float; LENGTH] {
            let mut rng = rand::thread_rng();
            let mut result = [0.0; LENGTH];
            for value in &mut result {
                *value = rng.gen_range(0.0..=1.0);
            }
            result
        }

        fn evaluate(&self, a: &[Float], b: &[Float]) -> EvaluationResult {
            let sum_a = Self::evaluate_single(a);
            let sum_b = Self::evaluate_single(b);
            let a_is_bigger = if sum_a == sum_b {
                let mut rng = rand::thread_rng();
                rng.gen()
            } else {
                sum_a > sum_b
            };
            match a_is_bigger {
                true => EvaluationResult::A,
                false => EvaluationResult::B,
            }
        }
    }

    #[test]
    fn test_evaluator() {
        let evaluator = MaxEvaluator::<2> {};

        let a = [0.0, 0.0];
        let b = [0.0, 1.0];
        assert_eq!(evaluator.evaluate(&a, &b), EvaluationResult::B);

        let a = [10.0, 0.2];
        let b = [0.5, 0.5];
        assert_eq!(evaluator.evaluate(&a, &b), EvaluationResult::B);
    }



    struct OnceEvaluator<const LENGTH: usize> {
        generated: bool
    }

    impl<const LENGTH: usize> Evaluator<LENGTH> for OnceEvaluator<LENGTH> {
        fn initialize(&mut self) -> [Float; LENGTH] {
            let genome_value = match self.generated {
                true => 0.0,
                false => {
                    self.generated = true;
                    1.0
                }
            };
            [genome_value; LENGTH]
        }

        fn evaluate(&self, a: &[Float], b: &[Float]) -> EvaluationResult {
            let eval = MaxEvaluator::<LENGTH> {};
            eval.evaluate(a, b)
        }
    }

    #[test]
    fn test_evolver() {
        let evaluator = OnceEvaluator::<2> { generated: false };
        let mut evolver: Evolver<_, ProbabilityMutator, 2, 10> = Evolver::new(evaluator);
        let fittest = evolver.evolve_step();
        assert_eq!(fittest, [1.0, 1.0]);
    }
}
