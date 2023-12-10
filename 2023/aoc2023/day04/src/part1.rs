use crate::shared::{get_game_details, get_num_shared, GameDetails};

/// Part 1 solution
pub fn solution(_contents: &Vec<&str>) -> u32 {
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
        let expected = 13;
        assert_eq!(result, expected);
    }
}
