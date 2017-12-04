use std::collections::{HashMap, HashSet};
fn main() {
    let passphrases = include_str!("puzzle_input.txt");
    let passphrases_without_duplicates: Vec<Vec<&str>> = passphrases
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .filter(|passphrase| !contains_duplicate_words(passphrase))
        .collect();

    println!(
        "There are {} passphrases with no repeated words.",
        passphrases_without_duplicates.len()
    );

    let valid_count = passphrases_without_duplicates
        .iter()
        .filter(|&passphrase| !contains_anagrams(passphrase))
        .count();

    println!("There are {} passphrases with no anagrams.", valid_count);
}

fn contains_duplicate_words(passphrase: &[&str]) -> bool {
    let mut words: HashSet<&str> = HashSet::new();

    for word in passphrase {
        if words.contains(word) {
            return true;
        }

        words.insert(word);
    }

    false
}

fn count_letters(word: &str) -> HashMap<char, u32> {
    let mut letters: HashMap<char, u32> = HashMap::new();

    for letter in word.chars() {
        *letters.entry(letter).or_insert(0) += 1;
    }

    letters
}

fn is_anagram(word1: &str, word2: &str) -> bool {
    count_letters(word1) == count_letters(word2)
}

fn contains_anagrams(passphrase: &[&str]) -> bool {
    for word in passphrase {
        if passphrase.iter().any(|word2| word != word2 && is_anagram(word, word2)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_words_negative_case() {
        assert!(!contains_duplicate_words(&["aa", "bb", "cc", "dd", "ee"]));
        assert!(!contains_duplicate_words(&["aa", "bb", "cc", "dd", "aaa"]));
    }

    #[test]
    fn contains_duplicate_words_positive_case() {
        assert!(contains_duplicate_words(&["aa", "bb", "cc", "dd", "aa"]));
    }

    #[test]
    fn count_letters_gives_correct_results() {
        let letter_counts = count_letters("aabccc");
        assert_eq!(letter_counts[&'a'], 2);
        assert_eq!(letter_counts[&'b'], 1);
        assert_eq!(letter_counts[&'c'], 3);
    }

    #[test]
    fn not_an_anagram() {
        assert!(!is_anagram("abc", "def"));
        assert!(!is_anagram("abc", "abcc"));
    }

    #[test]
    fn is_anagram_detects_anagrams() {
        assert!(is_anagram("abc", "bca"));
    }

    #[test]
    fn contains_anagrams_positive_case() {
        assert!(contains_anagrams(&["abcde", "xyz", "ecdab"]));
        assert!(contains_anagrams(&["oiii", "ioii", "iioi", "iiio"]));
    }

    #[test]
    fn contains_anagrams_negative_case() {
        assert!(!contains_anagrams(&["abcde", "fghij"]));
        assert!(!contains_anagrams(&["a", "ab", "abc", "abd", "abf", "abj"]));
        assert!(!contains_anagrams(&["iiii", "oiii", "ooii", "oooi", "oooo"]));
    }
}
