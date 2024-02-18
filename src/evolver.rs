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

fn average<const LENGTH: usize>(a: &[Float; LENGTH], b: &[Float; LENGTH]) -> [Float; LENGTH] {
    let mut result = [0.0; LENGTH];
    for idx in 0..LENGTH {
        result[idx] = (a[idx] + b[idx]) / 2.0;
    }
    result
}

fn select<const LENGTH: usize>(a: &[Float; LENGTH], b: &[Float; LENGTH]) -> [Float; LENGTH] {
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

pub struct Evolver<T, const LENGTH: usize, const POPULATION: usize>
where T: Evaluator<LENGTH> {
    population: [GenomeStats<LENGTH>; POPULATION],
    evaluator: T,
    mutation_chance: f64,
}

impl<T, const LENGTH: usize, const POPULATION: usize> Evolver<T, LENGTH, POPULATION>
where T: Evaluator<LENGTH> {
    pub fn new(evaluator: T) -> Self {
        let mut evaluator = evaluator;
        let mut population = Vec::with_capacity(POPULATION);
        for _ in 0..POPULATION {
            population.push(GenomeStats { genome: evaluator.initialize(), fitness: 0 });
        }
        let population: [GenomeStats<LENGTH>; POPULATION] = population.try_into().unwrap();
        Evolver { population, evaluator, mutation_chance: 1.0/10000.0 }
    }

    pub fn mutation_chance(&self) -> f64 {
        self.mutation_chance
    }

    pub fn set_mutation_chance(&mut self, mutation_chance: f64) {
        self.mutation_chance = mutation_chance;
    }

    pub fn evolve_step(&mut self) -> [Float; LENGTH] {
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

        let threshold = POPULATION * 10 / 2;
        let mut rng = rand::thread_rng();
        for idx in threshold..POPULATION {
            // Select two "winners" and apply some transformation
            let winner_a = rng.gen_range(0..threshold);
            let winner_b = winner_a + rng.gen_range(0..(threshold - 1));
            let winner_b = winner_b % threshold;
            let new_genome = average(&self.population[winner_a].genome, &self.population[winner_b].genome);
            self.population[idx].genome = new_genome;
        }

        // Mutate everyone randomly
        for genome in &mut self.population {
            for value in &mut genome.genome {
                if rng.gen_bool(self.mutation_chance) {
                    *value = rng.gen_range(0.0..1.0);
                }
            }
        }

        best_genome
    }
}


#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::bots::neural_bot::Float;

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
        let mut evolver: Evolver<_, 2, 10> = Evolver::new(evaluator);
        let fittest = evolver.evolve_step();
        assert_eq!(fittest, [1.0, 1.0]);
    }
}
