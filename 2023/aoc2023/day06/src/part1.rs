/// Part 2 solution
pub fn solution(contents: &Vec<&str>) -> u32 {
    let mut cum_total: u32 = 1;
    let times: Vec<u32> = contents[0]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| s.parse().expect("Failed to parse numbers for times"))
        .collect();

    let distances: Vec<u32> = contents[1]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| s.parse().expect("Failed to parse numbers for times"))
        .collect();

    for i in 0..times.len() {
        let mut wins: u32 = 0;
        for run in 1..times[i] {
            // time minus length of the current run is the amount of time we push the button,
            // which is our speed. It's also how much we subtract from the time
            // to multiple that remaining time
            let speed = times[i] - run;
            let remaining = times[i] - speed;
            let distance = speed * remaining;
            if distance > distances[i] {
                wins += 1;
            }
        }
        cum_total *= wins;
    }
    cum_total
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
        let expected = 288;
        assert_eq!(result, expected);
    }
}
