use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
pub enum EvaluationResult {
    A,
    B,
}

pub trait Evaluator<const LENGTH: usize> {
    fn initialize(&self) -> [f64; LENGTH];
    fn evaluate(&self, a: &[f64], b: &[f64]) -> EvaluationResult;
}

#[derive(Clone, Copy, Debug)]
struct GenomeStats<const LENGTH: usize> {
    genome: [f64; LENGTH],
    fitness: usize,
}

fn average<const LENGTH: usize>(a: &[f64; LENGTH], b: &[f64; LENGTH]) -> [f64; LENGTH] {
    let mut result = [0f64; LENGTH];
    for idx in 0..LENGTH {
        result[idx] = (a[idx] + b[idx]) / 2f64;
    }
    result
}

struct Evolver<T, const LENGTH: usize, const POPULATION: usize>
where T: Evaluator<LENGTH> {
    population: [GenomeStats<LENGTH>; POPULATION],
    evaluator: T,
}

impl<T, const LENGTH: usize, const POPULATION: usize> Evolver<T, LENGTH, POPULATION>
where T: Evaluator<LENGTH> {
    pub fn new(evaluator: T) -> Self {
        let mut population = Vec::with_capacity(POPULATION);
        for genome in &mut population {
            *genome = GenomeStats { genome: evaluator.initialize(), fitness: 0 };
        }
        let population: [GenomeStats<LENGTH>; POPULATION] = population.try_into().unwrap();
        Evolver { population, evaluator }
    }

    pub fn evolve_step(&mut self) -> [f64; LENGTH] {
        for idx_a in 0..(POPULATION - 1) {
            for idx_b in (idx_a + 1)..POPULATION {
                let idx_winner = match self.evaluator.evaluate(&self.population[idx_a].genome, &self.population[idx_b].genome) {
                    EvaluationResult::A => idx_a,
                    EvaluationResult::B => idx_b,
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
                if rng.gen_range(0..10000) == 0 {
                    *value = rng.gen_range(0f64..1f64);
                }
            }
        }

        best_genome
    }
}


#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::{Evaluator, EvaluationResult};

    struct MaxEvaluator<const LENGTH: usize>;

    impl<const LENGTH: usize> MaxEvaluator<LENGTH> {
        fn evaluate_single(values: &[f64]) -> f64 {
            values.iter().filter(|&v| *v >= 0f64 && *v <= 1f64).sum()
        }
    }

    impl<const LENGTH: usize> Evaluator<LENGTH> for MaxEvaluator<LENGTH> {
        fn initialize(&self) -> [f64; LENGTH] {
            let mut rng = rand::thread_rng();
            let mut result = [0f64; LENGTH];
            for value in &mut result {
                *value = rng.gen_range(0f64..=1f64);
            }
            result
        }

        fn evaluate(&self, a: &[f64], b: &[f64]) -> EvaluationResult {
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

        let a = [0f64, 0f64];
        let b = [0f64, 1f64];
        assert_eq!(evaluator.evaluate(&a, &b), EvaluationResult::B);

        let a = [10f64, 0.2f64];
        let b = [0.5f64, 0.5f64];
        assert_eq!(evaluator.evaluate(&a, &b), EvaluationResult::B);
    }
}
