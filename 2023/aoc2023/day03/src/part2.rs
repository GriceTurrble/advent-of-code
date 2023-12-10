use std::cmp;
use std::collections::HashMap;
use std::ops::Range;

use regex::Regex;
use substring::Substring;

/// Part 2 solution
pub fn solution(_contents: &Vec<&str>) -> i32 {
    // our running total for the solution
    let mut total: i32 = 0;
    // hash map of points where the gears are located.
    let mut gear_points: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    // regex to find numbers within the string
    let re: Regex = Regex::new(r"\d+").expect("Failed to parse numbers pattern");
    for (line_num, line) in _contents.iter().enumerate() {
        let rowrange: Range<usize> = Range {
            start: cmp::max((line_num as i32) - 1, 0) as usize,
            end: cmp::min((line_num as i32) + 2, (_contents.len() as i32) - 1) as usize,
        };
        let matches: Vec<(i32, Range<usize>)> = re
            .find_iter(line)
            .map(|m| {
                (
                    m.as_str().parse().expect("Could not parse number"),
                    m.range(),
                )
            })
            .collect();
        for mat in matches {
            let (num, colrange) = mat;
            if let Some(point) = find_gear_point(_contents, rowrange.clone(), colrange.clone()) {
                gear_points
                    .entry(point)
                    .and_modify(|v| v.push(num))
                    .or_insert(vec![num]);
            }
        }
    }

    for (_, nums) in gear_points {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }
    total
}

fn find_gear_point(
    _contents: &Vec<&str>,
    rowrange: Range<usize>,
    colrange: Range<usize>,
) -> Option<(usize, usize)> {
    let re: Regex = Regex::new(r"\*").expect("Failed to parse partnum regex");
    for row in rowrange {
        let start_index: usize = cmp::max((colrange.start as i32) - 1, 0) as usize;
        let end_index: usize = cmp::min(colrange.end + 1, _contents[row].len() - 1);
        let substr: &str = _contents[row].substring(start_index, end_index);
        if let Some(mat) = re.find(&substr) {
            return Option::Some((row, mat.range().start + start_index));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 467835;
        assert_eq!(result, expected);
    }
}
