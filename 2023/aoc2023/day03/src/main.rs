#![doc = include_str!("../README.md")]
use lube::{get_file_contents, get_input_file_path};
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
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
        if let Some(_mat) = re.find(&substr) {
            return true;
        }
    }
    false
}

fn find_gear_point(
    _contents: &Vec<String>,
    rowrange: Range<usize>,
    colrange: Range<usize>,
) -> Option<(usize, usize)> {
    let re: Regex = Regex::new(r"\*").expect("Failed to parse partnum regex");
    for row in rowrange {
        let start_index: usize = cmp::max((colrange.start as i32) - 1, 0) as usize;
        let end_index: usize = cmp::min(colrange.end + 1, _contents[row].len() - 1);
        let substr: &str = _contents[row].substring(start_index, end_index);
        if let Some(mat) = re.find(&substr) {
            return Option::Some((row, mat.range().start + start_index));
        }
    }
    None
}

/// Part 1 solution
fn part_one(_contents: &Vec<String>) {
    let mut total: i32 = 0;
    let re: Regex = Regex::new(r"\d+").expect("Failed to parse numbers pattern");
    for (line_num, line) in _contents.iter().enumerate() {
        let rowrange: Range<usize> = Range {
            start: cmp::max((line_num as i32) - 1, 0) as usize,
            end: cmp::min((line_num as i32) + 2, (_contents.len() as i32) - 1) as usize,
        };
        let content_rows: &[String] = &_contents[rowrange];
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
            let (num, colrange) = mat;
            if is_a_part_num(&content_rows, &colrange) {
                total += num;
            }
        }
    }
    println!(">> {total}");
}

/// Part 2 solution
fn part_two(_contents: &Vec<String>) {
    // our running total for the solution
    let mut total: i32 = 0;
    // hash map of points where the gears are located.
    let mut gear_points: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    // regex to find numbers within the string
    let re: Regex = Regex::new(r"\d+").expect("Failed to parse numbers pattern");
    for (line_num, line) in _contents.iter().enumerate() {
        let rowrange: Range<usize> = Range {
            start: cmp::max((line_num as i32) - 1, 0) as usize,
            end: cmp::min((line_num as i32) + 2, (_contents.len() as i32) - 1) as usize,
        };
        // let content_rows: &[String] = &_contents[rowrange];
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
            let (num, colrange) = mat;
            if let Some(point) = find_gear_point(_contents, rowrange.clone(), colrange.clone()) {
                gear_points
                    .entry(point)
                    .and_modify(|v| v.push(num))
                    .or_insert(vec![num]);
            }
        }
    }

    for (_, nums) in gear_points {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }
    // println!("{:?}", gear_points);
    println!(">> {total}");
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
