#![doc = include_str!("../README.md")]
use std::time::Instant;

use lube::{get_file_contents, get_input_file_path};

mod part1;

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents = get_file_contents(inp_file_path);
    let contents: Vec<&str> = contents.as_str().trim().split("\n").collect();

    println!("-------------------- PART 1 --------------------");
    let part1start = Instant::now();
    let result = part1::solution(&contents);
    println!(">> {result}");
    println!("   [{} μs]", part1start.elapsed().as_micros());

    println!("--- Part 2, I cheated :( . Run python day10/solution.py ---")
}
