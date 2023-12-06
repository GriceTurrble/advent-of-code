/// Part 2 solution
#[allow(unused_mut)]
pub fn solution(contents: &Vec<String>) {
    println!("-------------------- PART 1 --------------------");
    let (_, timing) = contents[0]
        .split_once(':')
        .expect("Failed to split the time string");
    let timing: u64 = timing
        .replace(" ", "")
        .parse()
        .expect("Failed to parse number");
    let (_, distance) = contents[1]
        .split_once(':')
        .expect("Failed to split the time string");
    let distance: u64 = distance
        .replace(" ", "")
        .parse()
        .expect("Failed to parse number");

    let mut wins: u32 = 0;
    for speed in 1..timing {
        let remaining = timing - speed;
        let remaining_dist = speed * remaining;
        if remaining_dist > distance {
            wins += 1;
        }
    }
    println!("Timing: {:?}", timing);
    println!("Distance: {:?}", distance);
    println!(">> {wins} ways to win");
}
