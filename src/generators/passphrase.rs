use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn load_frequency_list() -> Result<Vec<String>, io::Error> {
    let mut words: Vec<String> = Vec::with_capacity(30000);
    let file_path = "data/eng-simple_wikipedia_2011_30K-words.txt";
    let file = File::open(file_path)?;
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

pub fn generate_passphrase(lower_bound: u128) -> String {
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_passphrase_with_lower_bound() {
        for i in 0..1000 {
            let passphrase = generate_passphrase(i);
            assert!(passphrase.len() >= i as usize);
        }
    }
}
