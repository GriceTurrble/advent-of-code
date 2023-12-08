/// Part 1 solution
pub fn solution(contents: &Vec<&str>) -> u32 {
    let directions = contents[0];
    let maps = &contents[2..];
    println!("{:?}", maps);
    let mut steps: u32 = 0;
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
