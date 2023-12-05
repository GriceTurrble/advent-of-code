#![doc = include_str!("../README.md")]
use lube::{get_file_contents, get_input_file_path};

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: Vec<String> = get_file_contents(inp_file_path);

    println!("-------------------- PART 1 --------------------");
    part_one(&contents);
    println!("-------------------- PART 2 --------------------");
    part_two(&contents);
    println!("--------------------  DONE  --------------------");
}

/// Part 1 solution
fn part_one(_contents: &Vec<String>) {
    println!("Hello...");
}

/// Part 2 solution
fn part_two(_contents: &Vec<String>) {
    println!("...world!");
}
