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
pub fn get_file_contents(file_path: std::path::PathBuf) -> Vec<String> {
    // let args: Cli = Cli::parse();
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Map to string and collect, adapted from: https://stackoverflow.com/a/37547426/19462241
    let contents: Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();

    contents
}

/// String reverse utility. Borrowed from somewhere.
pub fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    let reversed_string: String = chars.into_iter().collect();
    reversed_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_string_works() {
        let result: String = reverse_string("abcdefg");
        assert_eq!(result, "gfedcba");
    }
}
