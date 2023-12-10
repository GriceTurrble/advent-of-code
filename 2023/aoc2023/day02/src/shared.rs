use std::cmp;
use std::collections::HashMap;

pub(crate) fn get_game_max_colors(details: &str) -> HashMap<&str, i32> {
    let mut max_colors: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for grab in details.split(';') {
        for color_detail in grab.trim().split(',') {
            let (num, col) = color_detail
                .trim()
                .split_once(' ')
                .expect("Failed to split this color detail");
            let num: i32 = num.trim().parse().expect("Failed to parse number");
            max_colors
                .entry(col)
                .and_modify(|val: &mut i32| *val = cmp::max(*val, num))
                .or_insert(num);
        }
    }
    max_colors
}
