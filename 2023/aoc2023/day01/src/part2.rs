use regex::Regex;

use lube::reverse_str;

/// Performs the full method for part 2.
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

/// Extract the first and last digits in the string, including words.
///
/// Performs a string reversal and finds the reversed word for the "last" digit,
/// which handles cases where the "last" is something like "oneight", where the
/// initial regex may only find "one" instead of the correct "eight" digit.
fn extract_numbers(inp: &str) -> [i32; 2] {
    let reverse_inp = reverse_str(inp);
    let reverse_inp = reverse_inp.as_str();
    let num_words = "one|two|three|four|five|six|seven|eight|nine";

    let pat_forward = format!("({num_words}|\\d)");
    let pat_forward = pat_forward.as_str();
    let re_forward = Regex::new(pat_forward).expect("Invalid regex pattern");

    let pat_reverse = format!("({}|\\d)", reverse_str(&num_words));
    let pat_reverse = pat_reverse.as_str();
    let re_reverse = Regex::new(pat_reverse).expect("Invalid regex pattern");

    let first = re_forward.find(inp).expect("No match found").as_str();
    let first = match first {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => first.parse::<i32>().expect("Cannot parse as number"),
    };

    let last = re_reverse
        .find(reverse_inp)
        .expect("No match found")
        .as_str();
    let last = match last {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => last.parse::<i32>().expect("Cannot parse as number"),
    };

    [first, last]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 281;
        assert_eq!(result, expected);
    }
}
