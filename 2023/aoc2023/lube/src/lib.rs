#![doc = include_str!("../README.md")]
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    /// The path to the input file
    input_file: std::path::PathBuf,
}

/// Parses CLI args via `clap`.
pub fn get_input_file_path() -> std::path::PathBuf {
    let args: Cli = Cli::parse();

    args.input_file
}

/// Given a file path for input, reads text from that file and produces a Vector of Strings,
/// one for each line of the file.
pub fn get_file_contents(file_path: std::path::PathBuf) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

/// String reverse utility. Borrowed from somewhere.
pub fn reverse_str(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

pub fn kata_find_shortest_word(s: &str) -> u32 {
    s.split_whitespace().map(|w| w.len()).min().unwrap_or(0) as u32
}

pub fn kata_accum(s: &str) -> String {
    // Lesson learned:
    //   String + &str works
    //   String + String does not
    // So a sample of c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(idx).as_str() works for simple concatenationF

    // Below I changed the above concat to use `format!` macro, which works with either type and returns String.
    // Working with a Vec of Strings that way seems most efficient.

    // I think generally I'll use funcs that can accept string slices and manipulate them into String,
    // then make a practice of converting back to `as_str()` in the calling context.
    // Not nearly as pleasant as just returning exactly what I want,
    // but works better with ownership and borrowing and whatnot.
    s.chars()
        .enumerate()
        .map(|(idx, c)| {
            format!(
                "{}{}",
                c.to_string().to_uppercase(),
                c.to_string().to_lowercase().repeat(idx),
            )
        })
        .collect::<Vec<String>>()
        .join("-")
}

pub fn kata_count_sheep(sheep: &[bool]) -> u8 {
    // Original: convert the bools to `u8` and `sum()` them
    //   sheep.iter().map(|s| *s as u8).sum()
    // Preferred: filter by `true` values and `.count()` them as `u8` :+1:
    sheep.iter().filter(|&&x| x).count() as u8
}

pub fn kata_positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|x| x.is_positive()).sum()
}

/// Kata: https://www.codewars.com/kata/515e271a311df0350d00000f/train/rust
pub fn kata_square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x.pow(2)).sum()
}

/// Kata: https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust
pub fn kata_descending_order(x: u64) -> u64 {
    let mut result: Vec<char> = x
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    result.sort_by(|a, b| b.cmp(a));
    // Learned about this one. Easier to produce a string from some iterator this way than doing it manually (.iter().collect() etc.)
    String::from_iter(result).parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let result = reverse_str(&"abcdefg");
        assert_eq!(result, "gfedcba");
    }

    #[test]
    fn test_kata_find_shortest_word() {
        let stuff: Vec<(&str, u32)> = vec![
            ("bitcoin take over the world maybe who knows perhaps", 3),
            (
                "turns out random test cases are easier than writing out basic ones",
                3,
            ),
            ("lets talk about javascript the best language", 3),
            ("i want to travel the world writing code one day", 1),
            ("Lets all go on holiday somewhere very cold", 2),
            ("Let's travel abroad shall we", 2),
        ];
        for (teststr, expected) in stuff {
            assert_eq!(kata_find_shortest_word(teststr), expected);
        }
    }

    #[test]
    fn test_kata_accum() {
        assert_eq!(
            kata_accum("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            kata_accum("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            kata_accum("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            kata_accum("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            kata_accum("HbideVbxncC"),
            "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
        );
    }

    #[test]
    fn test_kata_count_sheep() {
        assert_eq!(kata_count_sheep(&[false]), 0);
        assert_eq!(kata_count_sheep(&[true]), 1);
        assert_eq!(kata_count_sheep(&[true, false]), 1);
    }

    #[test]
    fn test_kata_positive_sum() {
        assert_eq!(kata_positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(kata_positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(kata_positive_sum(&[-1,2,3,4,-5]), 9);
        assert_eq!(kata_positive_sum(&[]), 0);
    }

    #[test]
    fn test_kata_square_sum() {
        assert_eq!(kata_square_sum(vec![1, 2]), 5);
        assert_eq!(kata_square_sum(vec![-1, -2]), 5);
        assert_eq!(kata_square_sum(vec![5, 3, 4]), 50);
        assert_eq!(kata_square_sum(vec![]), 0);
    }

    #[test]
    fn test_kata_descending_order() {
        assert_eq!(kata_descending_order(0), 0);
        assert_eq!(kata_descending_order(1), 1);
        assert_eq!(kata_descending_order(15), 51);
        assert_eq!(kata_descending_order(1021), 2110);
        assert_eq!(kata_descending_order(123456789), 987654321);
        assert_eq!(kata_descending_order(145263), 654321);
        assert_eq!(kata_descending_order(1254859723), 9875543221);
    }
}
