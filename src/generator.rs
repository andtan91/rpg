use rand::Rng;
use std::collections::HashSet;

use super::data::{CHARS, WORDS};

fn generate_random_indices(n: usize, modulo: usize) -> Vec<usize> {
    let mut indices = HashSet::new();

    let mut rng = rand::thread_rng();

    loop {
        let index = rng.gen_range(0..modulo);

        if !indices.contains(&index) {
            indices.insert(index);
        }

        if indices.len() == n as usize {
            break;
        }
    }

    indices.into_iter().collect()
}

/// Generates a word password.
pub fn generate_word_password(n_words: usize, sep: &str, add_digit: bool) -> String {
    let mut word_list: Vec<String> = generate_random_indices(n_words, WORDS.len())
        .iter()
        .map(|index| WORDS.get(*index).unwrap().to_string())
        .collect();

    if add_digit {
        let mut rng = rand::thread_rng();
        word_list.push((rng.gen_range(0..10)).to_string());
    }

    word_list.join(&sep.to_string())
}

/// Generates a character password.
pub fn generate_char_password(length: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let index: usize = rng.gen::<usize>() % CHARS.len();
            CHARS.get(index).unwrap().to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}
