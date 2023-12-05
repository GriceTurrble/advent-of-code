#![doc = include_str!("../README.md")]
use std::{ops::Range, cmp};
// Adds the `.collect_tuple()` feature
use itertools::Itertools;

use lube::{get_file_contents, get_input_file_path, split_strings};

#[derive(Debug)]
struct SeedMap {
    _name: String,
    source: Range<i64>,
    diff: i64,
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: String = get_file_contents(inp_file_path);
    // Separate by two newlines to get the maps as whole strings
    let segments: Vec<String> = split_strings(contents, "\n\n");

    println!("-------------------- PART 1 --------------------");
    part_one(&segments);
    println!("-------------------- PART 2 --------------------");
    part_two(&segments);
    println!("--------------------  DONE  --------------------");
}

fn get_seed_numbers(seed_section: &String) -> Vec<i64> {
    let seeds: Vec<i64> = seed_section
        .split_once(':')
        .expect("Failed to split the seed nums")
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Failed to parse seed number"))
        .collect();
    seeds
}

fn get_mapping(section: &String) -> Result<Vec<SeedMap>, Box<dyn std::error::Error>> {
    let mut maps: Vec<SeedMap> = Vec::new();
    let (name, numbers) = section.trim().split_once("\n").expect("Failed to split");
    for numset in numbers.trim().split('\n') {
        let parsed_nums: Vec<i64> = numset
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse().expect("Failed to parse mapping number"))
            .collect();
        let (dest, source, length) = parsed_nums
            .into_iter()
            .collect_tuple()
            .expect("Failed to get a tuple of the proper size (mismatch of numbers?)");
        maps.push(SeedMap {
            _name: name.to_string(),
            source: source..(source + length),
            diff: dest - source,
        })
    }
    Ok(maps)
}

/// Part 1 solution
fn part_one(_contents: &Vec<String>) {
    // Split the first set off into their seed values
    let seeds: Vec<i64> = get_seed_numbers(&_contents[0]);
    let mut maps: Vec<Vec<SeedMap>> = Vec::new();
    for section in &_contents[1.._contents.len()] {
        maps.push(get_mapping(section).expect("Failed to get a mapping."));
    }
    let mut final_result: i64 = -1;
    for seed in seeds {
        let mut result: i64 = seed;
        for mapping in &maps {
            for seed_map in mapping {
                if seed_map.source.contains(&result) {
                    result += seed_map.diff;
                    break;
                }
            }
        }
        if final_result == -1 {
            final_result = result;
        } else {
            final_result = cmp::min(final_result, result);
        }
    }
    println!(">> {final_result}");
}

/// Part 2 solution
fn part_two(_contents: &Vec<String>) {
    println!("...world!");
}
