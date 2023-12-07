#[derive(Debug)]
struct CardData {
    cards: String,
    bid: u32,
    hand_type: u8,
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
            cards: cards.to_string(),
            bid,
            hand_type,
            sort_key: format!("{}{}", hand_type, card_sort_str(cards))
        })
    }
    card_data.sort_by_key(|c| c.sort_key.clone());
    let mut total: u64 = 0;
    for (idx, data) in card_data.iter().enumerate() {
        total += ((idx as u64) + 1) * (data.bid as u64);
    }
    println!(">> {total}");
    // println!("{:?} :: {:?} :: {:?}", cards, bid, hand_type);
}

fn classify_card(cards: &str) -> u8 {
    let mut mapping: std::collections::HashMap<char, u8> = std::collections::HashMap::new();
    for foo in cards.chars() {
        let val = mapping.entry(foo).or_insert(0);
        *val += 1;
    }
    // Pull the values of the HashMap as a sorted vec of strings.
    let mut simplified: Vec<String> = mapping.iter().map(|(_,v)| v.to_string()).collect();
    simplified.sort();
    // Join the sorted strings into one, then match them up to the classifications.
    // For example, if all cards match, a single value of 5 should be present, representing "Five of a kind".
    // Distinct values would show up as `1`, pairs as a `2`, etc.
    // All combinations below should always add up to 5, of course.
    // Each is assigned a simple int value to aid in ordering later.
    match simplified.join("").as_str() {
        // Five of a kind: all cards are the same
        "5" => 7,
        // Four of a kind: 4 cards are the same, one is different
        "14" => 6,
        // Full house: 3 are the same, 2 are a second type
        "23" => 5,
        // Three of a kind: 3 are the same, the other 2 are both distinct
        "113" => 4,
        // Two pair: 2 are one type, 2 are another type, and the final is a distinct third type
        "122" => 3,
        // One pair: 2 are one type, the remaining 3 are all distinct types.
        "1112" => 2,
        // High card: all cards are distinct
        "11111" => 1,
        // Should not hit the default case, but it's there.
        _ => 0,
    }
}

/// Card sets cannot be sorted lexigraphically, so we need to recontextualize them so they are.
///
/// Ordering:
///     A K Q J T 9 8 7 6 5 4 3 2
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
        'J' => 'J',
        'T' => 'I',
        '9' => 'H',
        '8' => 'G',
        '7' => 'F',
        '6' => 'E',
        '5' => 'D',
        '4' => 'C',
        '3' => 'B',
        '2' => 'A',
        _ => c,
    }).collect::<Vec<char>>().iter().collect();
    converted
}
