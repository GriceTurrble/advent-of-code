use itertools::Itertools;
use std::{cmp, ops::Range};

#[derive(Debug)]
struct SeedMap {
    _name: String,
    source: Range<i64>,
    diff: i64,
}

/// Part 1 solution
pub fn solution(_contents: &Vec<&str>) -> i64 {
    let seeds: Vec<i64> = get_seed_numbers(&_contents[0]);
    let mut maps: Vec<Vec<SeedMap>> = Vec::new();
    for section in &_contents[1..] {
        maps.push(get_mapping(section).expect("Failed to get a mapping."));
    }
    let mut final_result: i64 = -1;
    for seed in seeds {
        let mut result: i64 = seed;
        for mapping in &maps {
            for seed_map in mapping {
                if seed_map.source.contains(&result) {
                    result += seed_map.diff;
                    break;
                }
            }
        }
        if final_result == -1 {
            final_result = result;
        } else {
            final_result = cmp::min(final_result, result);
        }
    }
    final_result
}


fn get_seed_numbers(seed_section: &str) -> Vec<i64> {
    let seeds: Vec<i64> = seed_section
        .split_once(':')
        .expect("Failed to split the seed nums")
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Failed to parse seed number"))
        .collect();
    seeds
}

fn get_mapping(section: &str) -> Result<Vec<SeedMap>, Box<dyn std::error::Error>> {
    let mut maps: Vec<SeedMap> = Vec::new();
    let (name, numbers) = section.trim().split_once("\n").expect("Failed to split");
    for numset in numbers.trim().split('\n') {
        let parsed_nums: Vec<i64> = numset
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse().expect("Failed to parse mapping number"))
            .collect();
        let (dest, source, length) = parsed_nums
            .into_iter()
            .collect_tuple()
            .expect("Failed to get a tuple of the proper size (mismatch of numbers?)");
        maps.push(SeedMap {
            _name: name.to_string(),
            source: source..(source + length),
            diff: dest - source,
        })
    }
    Ok(maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        data.trim().split("\n\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 35;
        assert_eq!(result, expected);
    }
}
