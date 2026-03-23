use rayon::prelude::*;

use crate::WordEntry;
/// Evaluates a guess against a secret word and returns the Wordle pattern Id.
///
/// The pattern is represented as base-3 integer (0 to 242).
/// * 0 = Gray (Not in word)
/// * 1 = Yellow (Wrong position)
/// * 2 = Green (Correct position)
///
/// Handles duplicate letter edge-cases by counting available characters in the secret.
pub fn evaluate(guess: &str, secret: &str) -> u8 {
    let mut secret_counts = [0u8; 26];
    let mut results = [0u8; 5];

    let g_bytes = guess.as_bytes();
    let s_bytes = secret.as_bytes();

    // Identify Greens
    for i in 0..5 {
        if g_bytes[i] == s_bytes[i] {
            results[i] = 2;
        } else {
            secret_counts[(s_bytes[i] - b'a') as usize] += 1;
        }
    }

    // Identify Yellows
    for i in 0..5 {
        if results[i] == 0 {
            if secret_counts[(g_bytes[i] - b'a') as usize] > 0 {
                results[i] = 1;
                secret_counts[(g_bytes[i] - b'a') as usize] -= 1;
            }
        }
    }

    // Convert to base-3 number
    let pattern = results[0] + results[1] * 3 + results[2] * 9 + results[3] * 27 + results[4] * 81;
    pattern
}

/// Pre-compute the entire game space into a flat 1D array.
///
/// Distribute the workload across all CPU cores.
/// Returns a byte array where index = (guess_idx * num_secrets) + secret_idx.
pub fn generate_matrix(guesses: &[String], secrets: &[String]) -> Vec<u8> {
    let num_guesses = guesses.len();
    let num_secrets = secrets.len();

    let mut matrix = vec![0u8; num_guesses * num_secrets];

    println!(
        "Computing matrix for {} for guesses and {} secrets...",
        num_guesses, num_secrets
    );

    matrix
        .par_chunks_mut(num_secrets)
        .enumerate()
        .for_each(|(g_idx, row)| {
            let guess = &guesses[g_idx];
            for (a_idx, pattern) in row.iter_mut().enumerate() {
                let secret = &secrets[a_idx];
                *pattern = evaluate(guess, secret);
            }
        });

    matrix
}

/// Shrink the pool of possible answers based on the feedback pattern received.
pub fn filter_words(
    matrix: &[u8],
    received_pattern: u8,
    guess_idx: usize,
    mut answers_pool: Vec<usize>,
    num_answers: usize,
) -> Vec<usize> {
    answers_pool.retain(|&word_idx| matrix[guess_idx * num_answers + word_idx] == received_pattern);

    answers_pool
}

/// Re-balance the probability distribution of the remaining possible answers.
pub fn renormalize_probabilities(
    answers_pool: &[usize],
    words_with_probabilities: &mut [WordEntry],
) {
    let total_weight: f64 = answers_pool
        .iter()
        .map(|&idx| words_with_probabilities[idx].probability)
        .sum();

    if total_weight > 0.0 {
        for &idx in answers_pool {
            words_with_probabilities[idx].probability /= total_weight;
        }
    }
}

/// Calculates the Shannon Entropy (Expected Information) for a single guess.
pub fn calculate_entropy(
    matrix: &[u8],
    answers_pool: &[usize],
    words_with_probabilities: &[WordEntry],
    guess_idx: usize,
    num_answers: usize,
) -> f64 {
    let mut pattern_probabilites = [0.0f64; 243];

    for &answer_idx in answers_pool {
        let pattern = matrix[guess_idx * num_answers + answer_idx];

        pattern_probabilites[pattern as usize] += words_with_probabilities[answer_idx].probability;
    }

    let mut entropy: f64 = 0.0;
    for &p in &pattern_probabilites {
        if p > 0.0 {
            entropy += -p * p.log2();
        }
    }
    entropy
}

/// Scans the entire dictionary to find the guess that yields the highest entropy.
pub fn find_best_guess(
    num_guesses: usize,
    matrix: &[u8],
    answers_pool: &[usize],
    words_with_probabilities: &[WordEntry],
    num_answers: usize,
) -> usize {
    let best_tuple = (0..num_guesses)
        .into_par_iter()
        .map(|guess_idx| {
            let guess_entropy = calculate_entropy(
                matrix,
                answers_pool,
                words_with_probabilities,
                guess_idx,
                num_answers,
            );

            (guess_idx, guess_entropy)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    best_tuple.0
}
