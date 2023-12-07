/// Part 2 solution
pub fn solution(contents: &Vec<&str>) {
    let mut cum_total: u32 = 1;
    let times: Vec<u32> = contents[0]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| s.parse().expect("Failed to parse numbers for times"))
        .collect();
    println!("Times: {:?}", times);

    let distances: Vec<u32> = contents[1]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| s.parse().expect("Failed to parse numbers for times"))
        .collect();
    println!("Distances: {:?}", distances);

    for i in 0..times.len() {
        let mut wins: u32 = 0;
        for run in 1..times[i] {
            // time minus length of the current run is the amount of time we push the button,
            // which is our speed. It's also how much we subtract from the time
            // to multiple that remaining time
            let speed = times[i] - run;
            let remaining = times[i] - speed;
            let distance = speed * remaining;
            if distance > distances[i] {
                wins += 1;
            }
        }
        cum_total *= wins;
    }
    println!(">> {cum_total} product of ways to win");
}
