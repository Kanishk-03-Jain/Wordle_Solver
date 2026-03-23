use serde::Deserialize;
use std::fs::File;
use std::io;

mod engine;

#[derive(Debug, Deserialize)]
struct RawWord {
    word: String,
    count: u64,
}

#[derive(Debug)]
struct WordEntry {
    word: String,
    probability: f64,
}

/// Loads words and frequency counts, converting them into a probability distribution.
fn load_words_from_memory(csv_data: &str) -> Result<Vec<WordEntry>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let mut raw_data: Vec<RawWord> = Vec::new();
    let mut total_count: u64 = 0;

    // Read everything and get the total sum
    for result in rdr.deserialize() {
        let record: RawWord = result?;
        total_count += record.count;
        raw_data.push(record);
    }

    // Convert counts to probabilities: P(w) = count / total_count
    let total_f = total_count as f64;
    let entries = raw_data
        .into_iter()
        .map(|r| WordEntry {
            word: r.word,
            probability: (r.count as f64) / total_f,
        })
        .collect();
    Ok(entries)
}

/// Helper function to convert string input (e.g., "gybbg") into base-3
fn parse_pattern(input: &str) -> u8 {
    let mut pattern = 0;
    let mut multiplier = 1;

    for c in input.trim().chars() {
        let val = match c {
            'g' | 'G' | '2' => 2, // Green
            'y' | 'Y' | '1' => 1, // Yellow
            _ => 0,
        };
        pattern += val * multiplier;
        multiplier *= 3;
    }
    pattern
}

fn main() {
    println!("Initializing Information Theory Wordle Engine...");

    let csv_text = include_str!("final.csv");
    let words_result = load_words_from_memory(csv_text);

    let mut words_with_probabilities = match words_result {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Failed to lead CSV: {}", e);
            return;
        }
    };

    let all_words: Vec<String> = words_with_probabilities
        .iter()
        .map(|entry| entry.word.clone())
        .collect();

    let num_answers = all_words.len();
    let num_guesses = all_words.len();

    let matrix = engine::generate_matrix(&all_words, &all_words);

    println!(
        "Matrix computation complete! Size: {} MB",
        matrix.len() / 1_000_000
    );

    // Array of indices pointing to valid words
    let mut answers_pool: Vec<usize> = (0..num_answers).collect();

    println!("\n=== START ===");

    loop {
        if answers_pool.len() == 1 {
            println!(
                "The answer is: {} ",
                all_words[answers_pool[0]].to_uppercase()
            );
            break;
        } else if answers_pool.is_empty() {
            println!("Error: No possible words left! You may have entered a pattern incorrectly.");
            break;
        }

        println!(
            "Calculating maximum entropy across {} words...",
            num_guesses
        );
        let guess_idx = engine::find_best_guess(
            num_guesses,
            &matrix,
            &answers_pool,
            &words_with_probabilities,
            num_answers,
        );
        println!("Recommended guess: {}", all_words[guess_idx].to_uppercase());
        println!("Enter the color pattern received (e.g., gybbg):");
        let mut input_pattern = String::new();
        io::stdin()
            .read_line(&mut input_pattern)
            .expect("Failed to read input");

        let received_pattern = parse_pattern(&input_pattern);

        answers_pool = engine::filter_words(
            &matrix,
            received_pattern,
            guess_idx,
            answers_pool,
            num_answers,
        );
        engine::renormalize_probabilities(&answers_pool, &mut words_with_probabilities);

        println!("----------------");
        println!("Remaining possibilities shrunk to: {}", answers_pool.len());
    }
}
