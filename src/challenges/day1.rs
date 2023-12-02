use std::collections::HashMap;
#[derive(Debug)]
struct Word {
    string_representation: String,
}

impl Word {
    fn new_from_description(description: Option<String>) -> Self {
        Word {
            string_representation: description.unwrap_or_else(String::new),
        }
    }
}

struct WordContainer {
    words: Vec<Word>,
}

impl WordContainer {
    fn new() -> Self {
        WordContainer { words: Vec::new() }
    }

    fn add_word(&mut self, word: Word) {
        self.words.push(word);
    }

    fn get_first_word(&self) -> Option<&Word> {
        self.words.first()
    }

    fn get_last_word(&self) -> Option<&Word> {
        self.words.last()
    }
}

fn process_input_lines(input_line: &str, word_to_digit: &HashMap<String, u32>) -> u32 {
    let search_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut container = WordContainer::new();

    for (index, ch) in input_line.chars().enumerate() {
        for &word in &search_words {
            let slice = &input_line[index..];
            if slice.starts_with(word) {
                container.add_word(Word::new_from_description(Some(word.to_string())));
            }
        }

        if let Some(digit) = ch.to_digit(10) {
            if let Some(word) = word_to_digit.iter().find_map(|(key, &val)| {
                if val == digit {
                    Some(key.clone())
                } else {
                    None
                }
            }) {
                container.add_word(Word::new_from_description(Some(word)));
            }
        }
    }

    let first_word = container.get_first_word();
    let last_word = container.get_last_word();

    if let (Some(first), Some(last)) = (
        first_word.and_then(|word| word_to_digit.get(&word.string_representation)),
        last_word.and_then(|word| word_to_digit.get(&word.string_representation)),
    ) {
        return (*first * 10) + *last;
    }

    0
}

fn day2(input: Vec<&str>) -> u32 {
    let word_to_digit: HashMap<String, u32> = HashMap::from([
        (String::from("zero"), 0),
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]);

    input
        .iter()
        .map(|input_line| process_input_lines(input_line, &word_to_digit))
        .sum()
}

fn day1(input: Vec<&str>) -> u32 {
    fn extract_digit(line: &str) -> Option<u32> {
        let first = line.chars().find(|c| c.is_digit(10))?;
        let last = line.chars().rev().find(|c| c.is_digit(10))?;
        Some(first.to_digit(10)? * 10 + last.to_digit(10)?)
    }

    input.iter().filter_map(|line| extract_digit(line)).sum()
}

#[cfg(test)]
mod day1tests {
    use super::{day1, day2};
    use crate::utils::read_lines;

    fn get_input_file() -> String {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join("resources")
            .join("test")
            .join("day1.txt")
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn should_resolve_dummy_input_1() {
        let input: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let result = day1(input);

        assert_eq!(result, 142);
    }

    #[test]
    fn should_resolve_dummy_input_2() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let result = day2(input);

        assert_eq!(result, 281);
    }

    #[test]
    fn should_test_first_challenge() {
        let input_lines: Vec<String> = read_lines(get_input_file());

        let digit_sum = day1(input_lines.iter().map(String::as_str).collect());

        assert_eq!(digit_sum, 55130);
    }

    #[test]
    fn should_test_second_challenge() {
        let input: Vec<String> = read_lines(get_input_file());

        let result = day2(input.iter().map(|s| s.as_str()).collect());

        assert_eq!(result, 54985);
    }
}
