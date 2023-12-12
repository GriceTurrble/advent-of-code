#![doc = include_str!("../README.md")]
use std::cmp;
use std::time::Instant;
use itertools::Itertools;

use funcs::{find_empty_rows, find_galaxies, transpose_grid};
use lube::{get_file_contents, get_input_file_path};

mod funcs;

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents = get_file_contents(inp_file_path);
    let contents: Vec<&str> = contents.as_str().trim().split("\n").collect();

    println!("-------------------- PART 1 --------------------");
    let part1start = Instant::now();
    let result = solve(&contents, 2);
    println!(">> {result}");
    println!("   [{} μs]", part1start.elapsed().as_micros());

    println!("-------------------- PART 2 --------------------");
    let part2start = Instant::now();
    let result = solve(&contents, 1_000_000);
    println!(">> {result}");
    println!("   [{} μs]", part2start.elapsed().as_micros());
}

fn solve(contents: &Vec<&str>, expansion: u64) -> u64 {
    let grid: Vec<String> = contents.iter().map(|s| s.to_string()).collect();
    let empty_rows = find_empty_rows(&grid);
    let transposed = transpose_grid(grid);
    let empty_cols = find_empty_rows(&transposed);
    let galaxies = find_galaxies(transposed);

    let sum_distances: u64 = galaxies
        .iter()
        .combinations(2)
        .map(|combo| {
            let found_cols = empty_cols
                .iter()
                .filter(|&&c| {
                    let (x1, x2) = (combo[0].0, combo[1].0);
                    let lower = cmp::min(x1, x2);
                    let upper = cmp::max(x1, x2);
                    (lower..upper).contains(&c)
                })
                .count() as u64;
            let found_rows = empty_rows
                .iter()
                .filter(|&&c| {
                    let (y1, y2) = (combo[0].1, combo[1].1);
                    let lower = cmp::min(y1, y2);
                    let upper = cmp::max(y1, y2);
                    (lower..upper).contains(&c)
                })
                .count() as u64;

            let absx = (combo[0].0 as i64 - combo[1].0 as i64).abs() as u64;
            let absx = absx + ((expansion - 1) * found_rows);

            let absy = (combo[0].1 as i64 - combo[1].1 as i64).abs() as u64;
            let absy = absy + ((expansion - 1) * found_cols);

            absx + absy
        })
        .sum();

    sum_distances
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        data.trim().split("\n").collect()
    }

    fn dotest(contents: &Vec<&str>, expansion: u64, expected: u64) {
        let result = solve(&contents, expansion);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let data = get_sample_data();
        dotest(&data, 2, 374);
    }

    #[test]
    fn test_part2() {
        let data = get_sample_data();
        dotest(&data, 10, 1030);
        dotest(&data, 100, 8410);
    }
}
