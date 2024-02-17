use super::GameState;

pub struct GameStateResult {
    state: GameState,
    count: usize,
}

impl GameStateResult {
    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub struct GameStateResults {
    result: Vec<GameStateResult>
}

impl GameStateResults {
    pub fn single(state: GameState) -> Self {
        GameStateResults { result: vec![GameStateResult { state, count: 1 }] }
    }

    pub fn new() -> Self {
        GameStateResults { result: vec![] }
    }

    pub fn all_counts(&self) -> usize {
        self.result.iter().map(|result| result.count).sum()
    }

    pub fn results(&self) -> &[GameStateResult] {
        &self.result
    }

    pub fn push(&mut self, state: GameState, count: usize) {
        self.result.push(GameStateResult { state, count })
    }
}
