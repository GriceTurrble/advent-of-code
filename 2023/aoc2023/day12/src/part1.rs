/// Part 1 solution
#[allow(unused_variables)]
pub fn solution(contents: &Vec<&str>) -> u64 {
    // TODO write actual solution (going to bed)
    21
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 21;
        assert_eq!(result, expected);
    }
}
