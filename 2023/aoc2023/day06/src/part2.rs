/// Part 2 solution
pub fn solution(contents: &Vec<&str>) -> u32 {
    let (_, timing) = contents[0]
        .split_once(':')
        .expect("Failed to split the time string");
    let timing: u64 = timing
        .replace(" ", "")
        .parse()
        .expect("Failed to parse number");

    let (_, distance) = contents[1]
        .split_once(':')
        .expect("Failed to split the time string");
    let distance: u64 = distance
        .replace(" ", "")
        .parse()
        .expect("Failed to parse number");

    let mut wins: u32 = 0;
    for speed in 1..timing {
        let remaining = timing - speed;
        let remaining_dist = speed * remaining;
        if remaining_dist > distance {
            wins += 1;
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "Time:      7  15   30\nDistance:  9  40  200";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 71503;
        assert_eq!(result, expected);
    }
}
