use std::collections::HashMap;

/// Part 1 solution
pub fn solution(contents: &Vec<&str>) -> u32 {
    let top_directions = contents[0];
    // Build a map out of the inputs
    let maps: HashMap<&str, (&str, &str)> = contents[2..]
        .iter()
        .map(|line| {
            let (start, dirs) = line.split_once('=').unwrap();
            let (start, dirs) = (start.trim(), dirs.trim());

            let (left, right) = dirs[1..dirs.len() - 1].split_once(',').unwrap();
            let (left, right) = (left.trim(), right.trim());

            (start, (left, right))
        })
        .collect();

    // Use cycle to run forever
    let mut steps: u32 = 0;
    let mut current = "AAA";
    for direction in top_directions.chars().cycle() {
        steps += 1;
        current = match direction {
            'L' => maps[current].0,
            _ => maps[current].1,
        };
        if current == "ZZZ" {
            break
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data1() -> Vec<&'static str> {
        let data = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
        data.trim().split("\n").collect()
    }

    fn get_sample_data2() -> Vec<&'static str> {
        let data = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result1 = solution(&get_sample_data1());
        let expected1 = 2;
        assert_eq!(result1, expected1);

        let result2 = solution(&get_sample_data2());
        let expected2 = 6;
        assert_eq!(result2, expected2);
    }
}
