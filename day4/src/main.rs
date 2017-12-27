use std::collections::HashSet;

fn valid_passphrases(input: &str) -> i32 {
    let mut count = 0;
    'outer: for passphrase in input.split('\n') {
        // A trie would be more efficient.
        let mut used_words = HashSet::new();
        for word in passphrase.split_whitespace() {
            if used_words.contains(word) {
                continue 'outer;
            }
            used_words.insert(word);
        }
        count += 1;
    }
    count
}

fn valid_passphrases_2(input: &str) -> i32 {
    let mut count = 0;
    'outer: for passphrase in input.split('\n') {
        let mut used_words = HashSet::new();
        for word in passphrase.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            let mut word = String::new();
            for c in chars {
                word.push(c);
            }
            if used_words.contains(&word) {
                continue 'outer;
            }
            used_words.insert(word);
        }
        count += 1;
    }
    count
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    println!("input:\n{}\n", input);
    println!("result 1: {}", valid_passphrases(input));
    println!("result 1: {}", valid_passphrases_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_passphrases_test() {
        assert_eq!(
            valid_passphrases("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa"),
            2
        );
    }

    #[test]
    fn valid_passphrases_2_test() {
        assert_eq!(valid_passphrases_2("abcde fghij"), 1);
        assert_eq!(valid_passphrases_2("abcde xyz ecdab"), 0);
        assert_eq!(valid_passphrases_2("a ab abc abd abf abj"), 1);
        assert_eq!(
            valid_passphrases_2("iiii oiii ooii oooi oooo\noiii ioii iioi iiio"),
            1
        );
    }
}
