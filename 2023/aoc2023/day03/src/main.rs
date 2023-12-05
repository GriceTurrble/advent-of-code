#![doc = include_str!("../README.md")]
#![allow(dead_code, unused_mut, unused_variables)] // Remove this at the end!
use lube::{get_file_contents, get_input_file_path};
use regex::Regex;
use std::cmp;
use std::ops::Range;
use substring::Substring;

fn is_a_part_num(rows: &[String], colrange: &Range<usize>) -> bool {
    let re: Regex = Regex::new(r"[^\d\.]").expect("Failed to parse partnum regex");
    for substr in rows.iter().map(|r| {
        String::from(r.substring(
            cmp::max((colrange.start as i32) - 1, 0) as usize,
            cmp::min(colrange.end + 1, r.len() - 1),
        ))
    }) {
        if let Some(mat) = re.find(&substr) {
            return true;
        }
    }
    false
}

/// Part 1 solution
fn part_one(_contents: &Vec<String>) {
    let mut total: i32 = 0;
    let re: Regex = Regex::new(r"\d+").expect("Failed to parse numbers pattern");
    for (line_num, line) in _contents.iter().enumerate() {
        let content_rows_range: Range<usize> = Range {
            start: cmp::max((line_num as i32) - 1, 0) as usize,
            end: cmp::min((line_num as i32) + 2, (_contents.len() as i32) - 1) as usize,
        };
        let content_rows: &[String] = &_contents[content_rows_range];
        let matches: Vec<(i32, Range<usize>)> = re
            .find_iter(line)
            .map(|m| {
                (
                    m.as_str().parse().expect("Could not parse number"),
                    m.range(),
                )
            })
            .collect();
        for mat in matches {
            let (num, range) = mat;
            if is_a_part_num(&content_rows, &range) {
                total += num;
            }
        }
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

    println!("-------------------- PART 1 --------------------");
    part_one(&contents);
    println!("-------------------- PART 2 --------------------");
    part_two(&contents);
    println!("--------------------  DONE  --------------------");
}
