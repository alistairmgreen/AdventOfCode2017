use std::collections::HashSet;
fn main() {
    let passphrases = include_str!("puzzle_input.txt");
    let valid_count = passphrases.lines()
        .filter(|passphrase| valid_passphrase(&passphrase))
        .count();
    
    println!("There are {} valid passphrases in the puzzle input.", valid_count);
}

fn valid_passphrase(passphrase: &str) -> bool {
    let mut words: HashSet<&str> = HashSet::new();

    for word in passphrase.split_whitespace() {
        if words.contains(word) {
            return false;
        }

        words.insert(word);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passphrase_with_no_repeated_words_is_valid() {
        assert!(valid_passphrase("aa bb cc dd ee"));
        assert!(valid_passphrase("aa bb cc dd aaa"));        
    }

    #[test]
    fn passphrase_with_repeated_word_is_invalid() {
        assert!(!valid_passphrase("aa bb cc dd aa"));
    }
}