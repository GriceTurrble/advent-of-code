use std::collections::HashMap;

use crate::shared::get_game_max_colors;

/// Part 1 solution
pub fn solution(contents: &Vec<&str>) -> i32 {
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
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 8;
        assert_eq!(result, expected);
    }
}
