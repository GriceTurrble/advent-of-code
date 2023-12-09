use std::collections::HashMap;
use num::integer::lcm;

/// Part 2 solution
pub fn solution(contents: &Vec<&str>) -> u64 {
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

    let starting_nodes: Vec<&str> = maps.keys().filter(|k| k.ends_with('A')).cloned().collect();
    let mut steps_lcm: u64 = 1;

    for current_node in starting_nodes {
        let mut current = current_node;
        let mut steps_curr: u64 = 0;
        for direction in top_directions.chars().cycle() {
            steps_curr += 1;
            current = match direction {
                'L' => maps[current].0,
                _ => maps[current].1,
            };
            if current.ends_with("Z") {
                break
            }
        }
        steps_lcm = lcm(steps_lcm, steps_curr);
    }
    steps_lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 6;
        assert_eq!(result, expected);
    }
}
