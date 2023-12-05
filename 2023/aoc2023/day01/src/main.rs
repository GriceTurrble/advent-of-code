#![doc = include_str!("../README.md")]
use lube::{get_file_contents, get_input_file_path, reverse_string, split_strings};
use regex::Regex;

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
        *numbers
            .get(numbers.len() - 1)
            .expect("Could not find index"),
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

    let last = re_reverse
        .find(reverse_inp.as_str())
        .expect("No match found")
        .as_str();
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

/// Performs the full method for part 1.
fn part_one(contents: &Vec<String>) {
    let mut total: i32 = 0;
    for content in contents {
        let [first, last] = extract_numbers_one(content);
        let real: i32 = format!("{}{}", first.to_string(), last.to_string())
            .parse::<i32>()
            .expect("Cannot parse as number");
        total += real;
    }
    println!(">> {total}");
}

/// Performs the full method for part 2.
fn part_two(contents: &Vec<String>) {
    let mut total: i32 = 0;
    for content in contents {
        let [first, last] = extract_numbers_two(content);
        let real: i32 = format!("{}{}", first.to_string(), last.to_string())
            .parse::<i32>()
            .expect("Cannot parse as number");
        total += real;
    }
    println!(">> {total}");
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: String = get_file_contents(inp_file_path);
    let contents: Vec<String> = split_strings(contents, "\n");

    println!("PART 1:");
    part_one(&contents);
    println!("PART 2:");
    part_two(&contents);
    println!("DONE");
}
