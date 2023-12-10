use std::collections::HashMap;

use crate::shared::{get_game_details, get_num_shared, GameDetails};

/// Part 2 solution
pub fn solution(_contents: &Vec<&str>) -> u32 {
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
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 30;
        assert_eq!(result, expected);
    }
}
