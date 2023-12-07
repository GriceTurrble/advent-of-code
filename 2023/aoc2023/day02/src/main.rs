#![doc = include_str!("../README.md")]
use lube::{get_file_contents, get_input_file_path};
use std::cmp;
use std::collections::HashMap;

fn get_game_max_colors(details: &str) -> HashMap<&str, i32> {
    let mut max_colors: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for grab in details.split(';') {
        for color_detail in grab.trim().split(',') {
            let (num, col) = color_detail
                .trim()
                .split_once(' ')
                .expect("Failed to split this color detail");
            let num: i32 = num.trim().parse().expect("Failed to parse number");
            max_colors
                .entry(col)
                .and_modify(|val: &mut i32| *val = cmp::max(*val, num))
                .or_insert(num);
        }
    }
    max_colors
}

/// Part 1 solution
fn part_one(contents: &Vec<&str>) {
    let mut solution: i32 = 0;
    let possible_rgb: HashMap<&str, i32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for content in contents {
        let (game, details) = content.split_once(':').expect("Failed to split this game");
        let (_, game_id) = game
            .split_once(' ')
            .expect("Failed to split the Game ID parts");
        let game_id: i32 = game_id
            .trim()
            .parse()
            .expect("Failed to parse the Game ID.");
        let max_colors: HashMap<&str, i32> = get_game_max_colors(details);

        if possible_rgb["red"] >= max_colors["red"]
            && possible_rgb["green"] >= max_colors["green"]
            && possible_rgb["blue"] >= max_colors["blue"]
        {
            solution += game_id;
        }
    }
    println!(">> {solution}");
}

/// Part 2 solution
fn part_two(contents: &Vec<&str>) {
    let mut solution: i32 = 0;
    for content in contents {
        let (_, details) = content.split_once(':').expect("Failed to split this game");
        let max_colors: HashMap<&str, i32> = get_game_max_colors(details);
        let power: i32 = max_colors["red"] * max_colors["green"] * max_colors["blue"];
        solution += power;
    }
    println!(">> {solution}");
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents = get_file_contents(inp_file_path);
    let contents = contents.as_str();
    let contents: Vec<&str> = contents.trim().split("\n").collect();

    println!("PART 1:");
    part_one(&contents);
    println!("PART 2:");
    part_two(&contents);
    println!("DONE");
}
