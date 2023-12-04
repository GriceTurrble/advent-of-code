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
#![allow(dead_code, unused_mut, unused_variables)] // Remove this at the end!
use lube::{get_file_contents, get_input_file_path};
use regex::Regex;
use std::ops::Range;

fn has_nearby_symbol(range: Range<usize>, _contents: &Vec<String>) -> bool {
    true
}

/// Part 1 solution
fn part_one(_contents: &Vec<String>) {
    let mut total: i32 = 0;
    let re: Regex = Regex::new(r"\d+").expect("Failed to parse regex pattern");
    for (line_num, line) in _contents.iter().enumerate() {
        let matches: Vec<(i32, Range<usize>)> = re
            .find_iter(line)
            .map(|m| {
                (
                    m.as_str().parse().expect("Could not parse number"),
                    m.range(),
                )
            })
            .collect();
        println!("{:?} {:?}", line_num, matches);
        println!("{:?}", matches[0].1);
        return;
    }
    println!(">> {total}");
}

/// Part 2 solution
fn part_two(_contents: &Vec<String>) {
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
