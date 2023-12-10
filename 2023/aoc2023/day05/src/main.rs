#![doc = include_str!("../README.md")]
use std::time::Instant;

use lube::{get_file_contents, get_input_file_path};

mod part1;
mod part2;
mod shared;


fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents = get_file_contents(inp_file_path);
    // Separate by two newlines to get the maps as whole strings
    let segments: Vec<&str> = contents.as_str().trim().split("\n\n").collect();

    println!("-------------------- PART 1 --------------------");
    let part1start = Instant::now();
    let result = part1::solution(&segments);
    println!(">> {result}");
    println!("   [{} μs]", part1start.elapsed().as_micros());

    println!("-------------------- PART 2 --------------------");
    let part2start = Instant::now();
    let result = part2::solution(&segments);
    println!(">> {result}");
    println!("   [{} μs]", part2start.elapsed().as_micros());
}
