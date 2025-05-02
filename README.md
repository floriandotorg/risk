# Risk Game AI

A Rust implementation of the classic Risk board game with a focus on AI opponent development and evolutionary algorithms.

## Overview

This project implements the Risk board game with various AI bots that use different strategies to play the game. The main features include:

- Full implementation of Risk game mechanics
- Multiple AI bot implementations:
  - Random Bot (makes random moves)
  - Rule-Based Bot (follows basic heuristic rules)
  - Neural Bot (uses neural networks trained through evolution)
- Evolutionary algorithm framework for training neural network bots
- Game visualization and map rendering

## Game Mechanics

The game follows standard Risk rules:

1. **Initial Placement**: Territories are randomly assigned to players
2. **Game Phases**:
   - **Reinforce**: Place armies on territories you control
   - **Attack**: Attack enemy territories
   - **Fortify**: Move armies between adjacent territories

## Bot Types

### Random Bot

Makes completely random moves. Used as a baseline for bot performance evaluation.

### Rule-Based Bot

Uses predefined heuristics to make strategic decisions, such as prioritizing continent control and border reinforcement.

### Neural Bot

Uses a neural network to evaluate game states and determine optimal moves. The network is trained using an evolutionary algorithm that pits bots against each other and selects the most successful ones.

## Evolutionary Framework

The project includes a sophisticated evolutionary framework that:

- Generates random neural network parameters
- Evaluates bot performance through tournaments
- Selects best-performing bots
- Applies mutations to generate new bot variants
- Repeats the process to evolve increasingly stronger bots

## Usage

Build and run the project:

```bash
cargo build --release
cargo run --release
```

### Training a Neural Bot

By default, running the program will evolve neural bots over several generations, evaluating them against both Random and Rule-Based bots.

```rust
// Example from main.rs
let mut evolver: Evolver<_, ProbabilityMutator, { NeuralBot::LENGTH }, 40> = 
    Evolver::with_transformation(Eval::<{ NeuralBot::LENGTH }> {}, 
    Box::new(transformations::select));

for g in 1..5 {
    // Evolution steps...
    best_genome = evolver.evolve_step();
    
    // Test against Random Bot
    let results = arena::play_games::<RandomBot, NeuralBot, _, _, _>(100, 
        &game::evaluate_win, 
        || RandomBot {}, 
        || NeuralBot::from_weights_and_biases(&genome));
}
```

### Playing Games

You can modify `main.rs` to play games between different bot types:

```rust
let mut game = game::Game::new(RuleBasedBot {}, NeuralBot::default());
let result = game.play_until_end(&game::evaluate_win, 
    &PlayOptions::default().save_map_images("output_folder").verbose()).unwrap();
```

## Visualization

The game can render the map state to PNG images, showing territory ownership and army counts. Set the appropriate options when playing a game:

```rust
let options = PlayOptions::default()
    .save_map_images("game_images")
    .verbose();
```

## Project Structure

- `src/`
  - `main.rs` - Program entry point and evolution logic
  - `game.rs` - Core game mechanics and flow
  - `territories.rs` - Territory definitions and relationships
  - `player.rs` - Player representation
  - `evolver.rs` - Evolutionary algorithm implementation
  - `arena.rs` - Bot tournament functionality
  - `game_state/` - Game state representation
    - `moves.rs` - Move generation and validation
    - `initial_placement.rs` - Initial game setup
    - `draw_map.rs` - Map visualization
  - `bots/` - Bot implementations
    - `random_bot.rs` - Random move bot
    - `rule_based_bot.rs` - Heuristic-based bot
    - `neural_bot.rs` - Neural network bot

## Requirements

- Rust 2021 Edition
- Dependencies as specified in Cargo.toml

## License

This project is open source. 