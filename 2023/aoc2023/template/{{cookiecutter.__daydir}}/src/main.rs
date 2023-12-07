#![doc = include_str!("../README.md")]
use lube::{get_file_contents, get_input_file_path};

mod part1;
mod part2;

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents = get_file_contents(inp_file_path);
    let contents: Vec<&str> = contents.as_str().trim().split("\n").collect();

    println!("-------------------- PART 1 --------------------");
    part1::solution(&contents);
    println!("-------------------- PART 2 --------------------");
    part2::solution(&contents);
}
