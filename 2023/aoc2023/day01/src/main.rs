/*!
## Advent of Code 2023 Day 1

Link: <https://adventofcode.com/2023/day/1>
*/
use clap::Parser;
use regex::Regex;
use std::fs;

#[derive(Parser)]
struct Cli {
    /// The path to the input file
    input_file: std::path::PathBuf,
}

/// Utility for splitting the contents of a text file into a vector of strings, one per line in the file.
fn get_file_contents(file_path: std::path::PathBuf) -> Vec<String> {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Map to string and collect, adapted from: https://stackoverflow.com/a/37547426/19462241
    let contents: Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();

    contents
}

/// String reverse utility. Borrowed from somewhere.
fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    let reversed_string: String = chars.into_iter().collect();
    reversed_string
}

/// For part 1, extract the first and last digits in the string.
fn extract_numbers_one(inp: &str) -> [i32; 2] {
    let pattern_str = r"\d";
    let re: Regex = Regex::new(pattern_str).expect("Invalid regex pattern");
    let numbers: Vec<&str> = re.find_iter(inp).map(|mat| mat.as_str()).collect();
    let numbers: Vec<i32> = numbers
        .iter()
        .map(|&s| s.parse().expect("Cannot parse as number"))
        .collect();

    [
        *numbers.get(0).expect("Could not find index"),
        *numbers.get(numbers.len() - 1).expect("Could not find index"),
    ]
}

/// For part 2, extract the first and last digits in the string, including words.
///
/// Performs a string reversal and finds the reversed word for the "last" digit,
/// which handles cases where the "last" is something like "oneight", where the
/// initial regex may only find "one" instead of the correct "eight" digit.
fn extract_numbers_two(inp: &str) -> [i32; 2] {
    let reverse_inp = reverse_string(inp);
    let num_words: &str = "one|two|three|four|five|six|seven|eight|nine";
    let pat_forward = format!("({}|\\d)", num_words);
    let pat_reverse = format!("({}|\\d)", reverse_string(num_words));
    let re_forward = Regex::new(pat_forward.as_str()).expect("Invalid regex pattern");
    let re_reverse = Regex::new(pat_reverse.as_str()).expect("Invalid regex pattern");

    let first = re_forward.find(inp).expect("No match found").as_str();
    let first = match first {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => first.parse::<i32>().expect("Cannot parse as number"),
    };

    let last = re_reverse.find(reverse_inp.as_str()).expect("No match found").as_str();
    let last = match last {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => last.parse::<i32>().expect("Cannot parse as number"),
    };

    [first, last]
}

fn main() {
    // Take CLI args
    let args: Cli = Cli::parse();

    // Read contents of the file into a string
    let contents: Vec<String> = get_file_contents(args.input_file);

    // part_one(contents.clone());
    part_one(contents.clone());
    part_two(contents.clone());
}

/// Performs the full method for part 1.
fn part_one(contents: Vec<String>) {
    let mut total: i32 = 0;
    for content in &contents {
        let [first, last] = extract_numbers_one(content);
        let real: i32 = format!("{}{}", first.to_string(), last.to_string())
            .parse::<i32>()
            .expect("Cannot parse as number");
        total += real;
    }
    println!("Part one solution: {total}");
}

/// Performs the full method for part 2.
fn part_two(contents: Vec<String>) {
    let mut total: i32 = 0;
    for content in &contents {
        let [first, last] = extract_numbers_two(content);
        let real: i32 = format!("{}{}", first.to_string(), last.to_string())
            .parse::<i32>()
            .expect("Cannot parse as number");
        total += real;
    }
    println!("Part two solution: {total}");
}
