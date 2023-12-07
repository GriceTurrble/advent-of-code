#![doc = include_str!("../README.md")]
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    /// The path to the input file
    input_file: std::path::PathBuf,
}

/// Parses CLI args via `clap`.
pub fn get_input_file_path() -> std::path::PathBuf {
    let args: Cli = Cli::parse();

    args.input_file
}

/// Given a file path for input, reads text from that file and produces a Vector of Strings,
/// one for each line of the file.
pub fn get_file_contents(file_path: std::path::PathBuf) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

/// String reverse utility. Borrowed from somewhere.
pub fn reverse_str(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

pub fn find_shortest_word(s: &str) -> u32 {
    s.split_whitespace().map(|w| w.len()).min().unwrap_or(0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let result = reverse_str(&"abcdefg");
        assert_eq!(result, "gfedcba");
    }

    #[test]
    fn test_find_shortest_word() {
        let stuff: Vec<(&str, u32)> = vec![
            ("bitcoin take over the world maybe who knows perhaps", 3),
            (
                "turns out random test cases are easier than writing out basic ones",
                3,
            ),
            ("lets talk about javascript the best language", 3),
            ("i want to travel the world writing code one day", 1),
            ("Lets all go on holiday somewhere very cold", 2),
            ("Let's travel abroad shall we", 2),
        ];
        for (teststr, expected) in stuff {
            assert_eq!(find_shortest_word(teststr), expected);
        }
    }
}
