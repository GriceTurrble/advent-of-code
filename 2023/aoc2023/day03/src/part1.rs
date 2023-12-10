use std::cmp;
use std::ops::Range;

use regex::Regex;
use substring::Substring;

/// Part 1 solution
pub fn solution(_contents: &Vec<&str>) -> i32 {
    let mut total: i32 = 0;
    let re: Regex = Regex::new(r"\d+").expect("Failed to parse numbers pattern");
    for (line_num, line) in _contents.iter().enumerate() {
        let rowrange: Range<usize> = Range {
            start: cmp::max((line_num as i32) - 1, 0) as usize,
            end: cmp::min((line_num as i32) + 2, (_contents.len() as i32) - 1) as usize,
        };
        let content_rows: &[&str] = &_contents[rowrange];
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
            if is_a_part_num(&content_rows, &colrange) {
                total += num;
            }
        }
    }
    total
}

fn is_a_part_num(rows: &[&str], colrange: &Range<usize>) -> bool {
    let re: Regex = Regex::new(r"[^\d\.]").expect("Failed to parse partnum regex");
    for substr in rows.iter().map(|r| {
        String::from(r.substring(
            cmp::max((colrange.start as i32) - 1, 0) as usize,
            cmp::min(colrange.end + 1, r.len() - 1),
        ))
    }) {
        if let Some(_mat) = re.find(&substr) {
            return true;
        }
    }
    false
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
        let expected = 4361;
        assert_eq!(result, expected);
    }
}
