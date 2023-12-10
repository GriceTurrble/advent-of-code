use std::collections::HashMap;

use crate::shared::get_game_max_colors;

/// Part 2 solution
pub fn solution(contents: &Vec<&str>) -> i32 {
    let mut solution: i32 = 0;
    for content in contents {
        let (_, details) = content.split_once(':').expect("Failed to split this game");
        let max_colors: HashMap<&str, i32> = get_game_max_colors(details);
        let power: i32 = max_colors["red"] * max_colors["green"] * max_colors["blue"];
        solution += power;
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
        let expected = 2286;
        assert_eq!(result, expected);
    }
}
