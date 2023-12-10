use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct GameDetails {
    pub card_id: u32,
    pub winning_nums: HashSet<u32>,
    pub game_nums: HashSet<u32>,
}

pub(crate) fn get_game_details(line: &str) -> GameDetails {
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

pub(crate) fn get_num_shared(details: &GameDetails) -> u32 {
    let shared_nums: HashSet<_> = details
        .game_nums
        .intersection(&details.winning_nums)
        .collect();
    let num_shared: usize = shared_nums.len();
    num_shared as u32
}
