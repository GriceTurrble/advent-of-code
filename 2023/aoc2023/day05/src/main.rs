#![doc = include_str!("../README.md")]
use std::{cmp, ops::Range};
// Adds the `.collect_tuple()` feature
use itertools::Itertools;

use lube::{get_file_contents, get_input_file_path};

#[derive(Debug)]
struct SeedMap {
    _name: String,
    source: Range<i64>,
    diff: i64,
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents = get_file_contents(inp_file_path);
    // Separate by two newlines to get the maps as whole strings
    let segments: Vec<&str> = contents.as_str().trim().split("\n\n").collect();

    println!("-------------------- PART 1 --------------------");
    part_one(&segments);
    println!("-------------------- PART 2 --------------------");
    part_two(&segments);
    println!("--------------------  DONE  --------------------");
}

fn get_seed_numbers_one(seed_section: &str) -> Vec<i64> {
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

fn get_mapping(section: &str) -> Result<Vec<SeedMap>, Box<dyn std::error::Error>> {
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
fn part_one(_contents: &Vec<&str>) {
    let seeds: Vec<i64> = get_seed_numbers_one(&_contents[0]);
    let mut maps: Vec<Vec<SeedMap>> = Vec::new();
    for section in &_contents[1..] {
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

fn get_seed_numbers_two(seed_section: &str) -> Vec<Range<i64>> {
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    let seeds: Vec<i64> = seed_section
        .split_once(':')
        .expect("Failed to split the seed nums")
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Failed to parse seed number"))
        .collect();
    // Main difference between this and part 1 is in parsing the seed values as ranges,
    // which will be far more efficient than returning a Vec of individual numbers.
    for seed_range in seeds.chunks(2) {
        seed_ranges.push(seed_range[0]..(seed_range[0] + seed_range[1]));
    }
    seed_ranges
}

/// Part 2 solution
fn part_two(_contents: &Vec<&str>) {
    let seed_ranges: Vec<Range<i64>> = get_seed_numbers_two(&_contents[0]);
    let mut maps: Vec<Vec<SeedMap>> = Vec::new();
    for section in &_contents[1..] {
        maps.push(get_mapping(section).expect("Failed to get a mapping."));
    }
    let mut final_result: i64 = -1;
    for seed_range in seed_ranges {
        for seed in seed_range {
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
    }
    println!(">> {final_result}");
}
