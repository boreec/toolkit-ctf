use rand::Rng;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

const DEFAULT_WORDS_LIST_FILE: &str =
    "data/eng-simple_wikipedia_2021_30K-words.txt";

fn load_frequency_list() -> Result<Vec<String>, io::Error> {
    let mut words: Vec<String> = Vec::with_capacity(30000);
    let file = File::open(DEFAULT_WORDS_LIST_FILE)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_content = line?;
        let line_words: Vec<&str> = line_content.split_whitespace().collect();
        if line_words.len() == 3 {
            words.push(line_words[1].to_string());
        }
    }
    Ok(words)
}

pub fn generate_passphrase(
    min_length: usize,
    words_list: &Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut passphrase = String::new();

    while passphrase.len() < min_length {
        let random_index = rng.gen_range(0..words_list.len());
        let random_word: String = words_list[random_index].clone();
        passphrase = passphrase + random_word.as_str();
    }
    Ok(passphrase.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_frequency_list() {
        let frequency_list = load_frequency_list();
        assert!(frequency_list.is_ok());
        assert!(frequency_list.unwrap().len() >= 30000);
    }

    #[test]
    fn test_generate_passphrase_with_min_length() {
        let frequency_list = load_frequency_list().unwrap();
        for min_length in 0..1000 {
            let passphrase = generate_passphrase(min_length, &frequency_list);
            assert!(passphrase.is_ok(), "{:?}", passphrase);
            assert!(passphrase.unwrap().len() >= min_length);
        }
    }
}
