/*!
## Advent of Code 2023 Day 2

Link: <https://adventofcode.com/2023/day/2>
*/
use lube::{get_file_contents, get_input_file_path, ...};

/// Part 1 solution
fn part_one(contents: Vec<String>) {
    println!("Hello...");
}

/// Part 2 solution
fn part_two(contents: Vec<String>) {
    println!("...world!");
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: Vec<String> = get_file_contents(inp_file_path);

    println!("PART 1:");
    part_one(contents.clone());
    println!("PART 2:");
    part_two(contents.clone());
    println!("DONE");
}
