#[derive(Debug)]
struct CardData {
    _cards: String,
    _hand_type: u8,
    bid: u32,
    sort_key: String,
}

/// Part 1 solution
pub fn solution(contents: &Vec<String>) {
    let mut card_data: Vec<CardData> = Vec::new();
    for line in contents {
        let (cards, bid) = line.trim().split_once(' ').expect("Failed to split the things");
        let bid: u32 = bid.parse().expect("Failed to parse bid number.");
        let hand_type = classify_card(cards);
        card_data.push(CardData {
            _cards: cards.to_string(),
            _hand_type: hand_type,
            bid,
            sort_key: format!("{}{}", hand_type, card_sort_str(cards)),
        });
    }
    card_data.sort_by_key(|c| c.sort_key.clone());
    let mut total: u64 = 0;
    for (idx, data) in card_data.iter().enumerate() {
        total += ((idx as u64) + 1) * (data.bid as u64);
    }
    println!("{:?}", card_data);
    println!(">> {total}");
}

fn classify_card(cards: &str) -> u8 {
    let mut mapping: std::collections::HashMap<char, u8> = std::collections::HashMap::new();
    for foo in cards.chars() {
        let val = mapping.entry(foo).or_insert(0);
        *val += 1;
    }
    // Pull the values of the HashMap as a sorted vec of ints.
    let mut simplified: Vec<u8> = mapping
        .iter()
        .filter(|(&k, _)| k != 'J')
        .map(|(_, v)| *v)
        .collect();
    simplified.sort();

    if let Some(&jokers) = mapping.get(&'J') {
        if let Some(last_element) = simplified.last_mut() {
            *last_element += jokers;
        } else {
            // The vector was empty: this hand was all jokers!
            // Push our Five of a kind identifier, instead:
            simplified.push(5);
        }
    }

    // Join the sorted strings into one, then match them up to the classifications.
    // For example, if all cards match, a single value of 5 should be present, representing "Five of a kind".
    // Distinct values would show up as `1`, pairs as a `2`, etc.
    // All combinations below should always add up to 5, of course.
    // Each is assigned a simple int value to aid in ordering later.
    match simplified[..] {
        // Five of a kind: all cards are the same
        [5] => 7,
        // Four of a kind: 4 cards are the same, one is different
        [1, 4] => 6,
        // Full house: 3 are the same, 2 are a second type
        [2, 3] => 5,
        // Three of a kind: 3 are the same, the other 2 are both distinct
        [1, 1, 3] => 4,
        // Two pair: 2 are one type, 2 are another type, and the final is a distinct third type
        [1, 2, 2] => 3,
        // One pair: 2 are one type, the remaining 3 are all distinct types.
        [1, 1, 1, 2] => 2,
        // High card: all cards are distinct
        [1, 1, 1, 1, 1] => 1,
        // Should not hit the default case, but it's there.
        _ => 0,
    }
}

/// Card sets cannot be sorted lexigraphically, so we need to recontextualize them so they are.
///
/// Ordering:
///     A K Q T 9 8 7 6 5 4 3 2 J
///
/// Note the difference from part1, J is now at the end of this list
///
/// Translate these to ordered letters:
///     A B C D E F G H I J K L M
///
/// But reverse them for correct lexigraphical sort:
///     M L K J I H G F E D C B A
fn card_sort_str(cards: &str) -> String {
    let converted: String = cards.chars().map(|c| match c {
        'A' => 'M',
        'K' => 'L',
        'Q' => 'K',
        'T' => 'J',
        '9' => 'I',
        '8' => 'H',
        '7' => 'G',
        '6' => 'F',
        '5' => 'E',
        '4' => 'D',
        '3' => 'C',
        '2' => 'B',
        'J' => 'A',
        _ => c,
    }).collect::<Vec<char>>().iter().collect();
    converted
}
