# Wordle Solver: Information Theory Engine

A high-performance, parallelized Wordle solver written in Rust that utilizes Shannon Entropy to find the optimal guesses.

This project is designed to solve Wordle by computing the expected information gain (entropy) for every possible guess, balancing word frequency probabilities to mirror how humans naturally prioritize more common words.

## Features

- **Information Theory (Shannon Entropy) Approach:** Calculates the optimal guess by maximizing the expected information gain.
- **Parallel Computing:** Heavily leverages the `rayon` crate to parallelize the matrix generation and entropy calculations across all available CPU cores.
- **Frequency-Aware:** Evaluates word probabilities based on their real-world usage frequencies, improving practical guessing efficiency.
- **Pre-computed Game Space:** Pre-computes the entire game matrix (every possible guess vs. every possible answer) for lightning-fast querying.

## How It Works

The solver works on the foundational concepts of Information Theory:
1. **Initial Probability Distribution:** It loads a list of valid words and their occurrence frequencies (`final.csv`). The probability of a word being the answer is proportional to its relative frequency.
2. **Matrix Generation:** The engine pre-evaluates every combination of a guess and a secret word, generating a base-3 pattern for the colors returned (Gray, Yellow, Green).
3. **Entropy Calculation:** For a given guess, it calculates the probability of each of the 243 possible color patterns arising from the remaining pool of possible answers. It computes the Shannon Entropy: $H = -\sum p(x) \log_2 p(x)$.
4. **Best Guess:** The solver parallelly evaluates the entropy for all remaining valid guesses and recommends the one with the maximum entropy.
5. **Filtering and Renormalization:** After the user plays the recommended guess and provides the resulting color pattern, the engine filters out impossible answers and renormalizes the probabilities of the remaining words.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
# Clone the repository (if applicable)
git clone <repository-url>
cd wordle_solver

# Build the project for release (highly recommended for performance)
cargo build --release
```

## Usage

1. Run the application:
   ```bash
   cargo run --release
   ```

2. The application will initialize, compute the game matrix, and present the first recommended guess.

3. Enter the recommended guess into the Wordle game.

4. Provide the color pattern returned by the game back to the solver's terminal prompt:
   - Use `g`, `G`, or `2` for **Green** (correct letter, correct spot)
   - Use `y`, `Y`, or `1` for **Yellow** (correct letter, wrong spot)
   - Use any other key (e.g., `b`, `B`, `0`) for **Gray/Black** (letter not in word)

   *Example Input: `gybbg`* (Green, Yellow, Black, Black, Green)

5. The solver will filter the remaining word string and provide the next best guess. Repeat the process until the solver narrows it down precisely!

## Project Structure

- `src/main.rs`: Entry point of the application. Handles loading CSV data, formatting state logic, user interactions, and the iterative game loop.
- `src/engine.rs`: Core mathematical backend. Implements the pattern evaluation, entropy calculations, Rayon parallelization, probability renormalization, and game space matrix generation.
- `src/final.csv`: Contains the list of words along with their occurrence count metadata.

## Dependencies

- `csv`: Fast CSV parsing for loading the word lists.
- `rayon`: Data-parallelism library for multi-threading operations.
- `serde` & `serde_json`: Used for deserialization of data formats.

## Author

- **Kanishk Jain**
