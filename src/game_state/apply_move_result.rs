use rand::Rng;

use super::GameState;

pub struct GameStateWithCount {
    state: GameState,
    count: usize,
}

impl GameStateWithCount {
    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub struct ApplyMoveResult {
    states_with_count: Vec<GameStateWithCount>
}

impl ApplyMoveResult {
    pub fn single(state: GameState) -> Self {
        ApplyMoveResult { states_with_count: vec![GameStateWithCount { state, count: 1 }] }
    }

    pub fn new() -> Self {
        ApplyMoveResult { states_with_count: vec![] }
    }

    pub fn total_count(&self) -> usize {
        self.states_with_count.iter().map(|result| result.count).sum()
    }

    pub fn states_with_count(&self) -> &[GameStateWithCount] {
        &self.states_with_count
    }

    pub fn push(&mut self, state: GameState, count: usize) {
        self.states_with_count.push(GameStateWithCount { state, count })
    }

    pub fn random_state_by_probability(&self) -> GameState {
        let mut rnd: i16 = rand::thread_rng().gen_range(0..self.total_count()).try_into().unwrap();
        for state in self.states_with_count.iter() {
            rnd -= state.count() as i16;
            if rnd <= 0 {
                return *state.state();
            }
        }
        panic!("Should not happen");
    }
}
