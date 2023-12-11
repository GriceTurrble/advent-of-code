/*!
  # Codewars Kata Solutions collection (mine, anyway)

  A collection of [Codewars](https://www.codewars.com) kata solutions I'm keeping handy
  as a reference for things in the future.

  Some solutions here are my own. Others are refactored after my solution to show
  a best practice.

  Each should link to the kata description the solution is based from.

  ## Doesn't really belong here

  These aren't exactly relevant to the `lube` crate itself,
  but I found it useful to produce them, make them available to access via the crate,
  etc.; doing all the "needful" to not only showcase these solutions,
  but also display the machinery necessary to have multi-file libs in the first place.

  ## Usage

  These aren't intended to be used directly: they don't have much utility outside
  the Katas themselves. However, there are tests configured, which can be run
  by testing the entire lib:

  ```shell
  cargo test -p lube
  ```
*/

/// Kata: <https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9>
pub fn kata_find_shortest_word(s: &str) -> u32 {
    s.split_whitespace().map(|w| w.len()).min().unwrap_or(0) as u32
}

/// Kata: <https://www.codewars.com/kata/5667e8f4e3f572a8f2000039>
pub fn kata_mumbling(s: &str) -> String {
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

/// Kata: <https://www.codewars.com/kata/54edbc7200b811e956000556/>
pub fn kata_count_sheep(sheep: &[bool]) -> u8 {
    // Original: convert the bools to `u8` and `sum()` them
    //   sheep.iter().map(|s| *s as u8).sum()
    // Preferred: filter by `true` values and `.count()` them as `u8` :+1:
    sheep.iter().filter(|&&x| x).count() as u8
}

/// Kata: <https://www.codewars.com/kata/5715eaedb436cf5606000381>
pub fn kata_positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|x| x.is_positive()).sum()
}

/// Kata: <https://www.codewars.com/kata/515e271a311df0350d00000f/>
pub fn kata_square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x.pow(2)).sum()
}

/// Kata: <https://www.codewars.com/kata/5467e4d82edf8bbf40000155/>
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
            kata_mumbling("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            kata_mumbling("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            kata_mumbling("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            kata_mumbling("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            kata_mumbling("HbideVbxncC"),
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
