/// Part 2 solution
pub fn solution(contents: &Vec<&str>) -> &'static str  {
    println!("Part 2 incomplete!");
    println!("{:?}", contents[0]);
    "XYZ"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = "XYZ";
        assert_eq!(result, expected);
    }
}
