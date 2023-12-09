/// Part 1 solution
pub fn solution(contents: &Vec<&str>) -> i32 {
    println!("{:?}", contents[0]);
    let mut total: i32 = 0;
    for line in contents {
        let nums: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
        total += get_next_num(nums);
    }
    total
}

fn get_next_num(nums: Vec<i32>) -> i32 {
    let num_zeros = nums.iter().filter(|&&x| x != 0).count();
    if num_zeros == 0 {
        0
    } else {
        // Build the next set of diffs for the input line and recurse
        let mut next_set: Vec<i32> = Vec::new();
        for i in 0..(nums.len()-1) {
            next_set.push(nums[i+1] - nums[i]);
        }
        nums[nums.len()-1] + get_next_num(next_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 114;
        assert_eq!(result, expected);
    }
}
