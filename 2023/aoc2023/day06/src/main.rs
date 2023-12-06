#![doc = include_str!("../README.md")]
use lube::{get_file_contents, get_input_file_path, split_strings};

mod part1;
mod part2;

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: String = get_file_contents(inp_file_path);
    let contents: Vec<String> = split_strings(contents, "\n");

    part1::solution(&contents);
    part2::solution(&contents);
}
