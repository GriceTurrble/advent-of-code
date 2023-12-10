use regex::Regex;

/// Performs the full method for part 1.
pub fn solution(contents: &Vec<&str>) -> i32 {
    let mut total: i32 = 0;
    for content in contents {
        let [first, last] = extract_numbers(content);
        let real: i32 = format!("{}{}", first.to_string(), last.to_string())
            .parse::<i32>()
            .expect("Cannot parse as number");
        total += real;
    }
    total
}

/// Extract the first and last digits in the string.
fn extract_numbers(inp: &str) -> [i32; 2] {
    let pattern_str = r"\d";
    let re: Regex = Regex::new(pattern_str).expect("Invalid regex pattern");
    let numbers: Vec<&str> = re.find_iter(inp).map(|mat| mat.as_str()).collect();
    let numbers: Vec<i32> = numbers
        .iter()
        .map(|&s| s.parse().expect("Cannot parse as number"))
        .collect();

    [
        *numbers.get(0).expect("Could not find index"),
        *numbers
            .get(numbers.len() - 1)
            .expect("Could not find index"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 142;
        assert_eq!(result, expected);
    }
}
