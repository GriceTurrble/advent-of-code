/*!
## Advent of Code 2023 Day 3

Link: <https://adventofcode.com/2023/day/3>

## Thought process

We need to identify numbers in the schematic, which is the first issue.
For that we should probably identify them via regex and find a means to parse out thier positions
(either the start pos or the bounding box). If we know a number's position, presumably we can form
a box around that number and get all symbols in that box. If any symbols are present other than `.`,
then the part is a part number and should be pulled.

Numbers all range from 1 to 3 digits
*/
use lube::{get_file_contents, get_input_file_path};

/// Part 1 solution
fn part_one(contents: &Vec<String>) {
    println!("Hello...");
}

/// Part 2 solution
fn part_two(contents: &Vec<String>) {
    println!("...world!");
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: Vec<String> = get_file_contents(inp_file_path);

    println!("PART 1:");
    part_one(&contents);
    println!("PART 2:");
    part_two(&contents);
    println!("DONE");
}
