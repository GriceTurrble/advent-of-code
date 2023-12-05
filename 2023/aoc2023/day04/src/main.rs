#![doc = include_str!("../README.md")]
use std::collections::{HashSet, HashMap};

use lube::{get_file_contents, get_input_file_path};

#[derive(Debug)]
struct GameDetails {
    card_id: u32,
    winning_nums: HashSet<u32>,
    game_nums: HashSet<u32>,
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

/// Part 1 solution
fn part_one(_contents: &Vec<String>) {
    let mut total: u32 = 0;
    for line in _contents {
        let mut adder: u32 = 0;
        let game_details: GameDetails = get_game_details(line);
        let num_shared: u32 = get_num_shared(&game_details);
        let base: u32 = 2;
        if num_shared > 0 {
            adder = base.pow(num_shared - 1);
        }
        total += adder;
    }
    println!(">> {total}");
}

fn get_game_details(line: &String) -> GameDetails {
    let (card_id, games) = line.split_once(':').expect("Failed to split the card IDs.");
    let card_id: u32 = (card_id
        .trim()
        .split_once(' ')
        .expect("Failed to get the card ID")
        .1)
        .trim()
        .parse()
        .expect("Failed to parse card ID");
    let (winning_nums, game_nums) = games
        .trim()
        .split_once('|')
        .expect("Failed to split the winning numbers.");
    let winning_nums: HashSet<u32> = winning_nums
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Failed to parse winning num"))
        .collect();
    let game_nums: HashSet<u32> = game_nums
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Failed to parse game num"))
        .collect();

    GameDetails {
        card_id,
        winning_nums,
        game_nums,
    }
}

fn get_num_shared(details: &GameDetails) -> u32 {
    let shared_nums: HashSet<_> = details.game_nums.intersection(&details.winning_nums).collect();
    let num_shared: usize = shared_nums.len();
    num_shared as u32
}

/// Part 2 solution
fn part_two(_contents: &Vec<String>) {
    let mut total: u32 = 0;
    let mut game_counts: HashMap<u32, u32> = HashMap::new();
    for line in _contents {
        let game_details: GameDetails = get_game_details(line);
        let num_shared = get_num_shared(&game_details);
        let num_runs = *game_counts.entry(game_details.card_id).or_insert(1);
        total += num_runs;
        for _ in 0..num_runs {
            for new_game in (game_details.card_id + 1)..=(game_details.card_id + num_shared) {
                *game_counts.entry(new_game).or_insert(1) += 1;
            }
        }
    }
    println!(">> {total}");
}
