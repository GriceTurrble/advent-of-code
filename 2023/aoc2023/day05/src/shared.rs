use std::ops::Range;

use itertools::Itertools;

#[derive(Debug)]
pub(crate) struct SeedMap {
    pub _name: String,
    pub source: Range<i64>,
    pub diff: i64,
}

pub(crate) fn get_mapping(section: &str) -> Result<Vec<SeedMap>, Box<dyn std::error::Error>> {
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
