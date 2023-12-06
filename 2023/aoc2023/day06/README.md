# Advent of Code 2023 Day 6

Link: <https://adventofcode.com/2023/day/6>

## Part 1

This one is pretty straightforward, at least the way I'm doing it (brute force FTW!).

### The Elegant Way

How I might approach this elegantly is figuring out both the minimum speed that beats a record and the maximum. Anything else is unnecessary information: we can just take `(max_speed + 1) - min_speed` and get the right result. The sample shows this, as max speed of `5` wins vs min speed of `2`: `5+1-2 == 4` which is the right result.

So I might have gone at this with two separate loops, starting from the center point, looking for those min and max speeds.

### My Inelegant Way

Similar to how the example was laid out, I constructed a range from `1..time`, where `time` is the time of the race as an upper bound (we don't need to consider `0` or `time`, as those both result in distances of `0`). From there, it's a simple matter of counting `wins` and then `cum_total *= wins` at the end of that race.

## Part 2

Very similar to the first step, except we just had to smash the strings together to construct one larger number for each:

```rust
let (_, timing) = time_str
    .split_once(':')
    .expect("Failed to split the time string");
let timing: u64 = timing
    .replace(" ", "")
    .parse()
    .expect("Failed to parse number");
```

Initially I threw these into `u32` values, but that quickly overflowed for the distance I had: `246_144_110_121_111` (246 trillion). So, `u64` it is.

And for lack of anything indicating that I would blow up the machine if I tried, I just calculated number of wins by running over the complete range:

```rust
let mut wins: u32 = 0;
for speed in 1..timing {
    let remaining = timing - speed;
    let remaining_dist = speed * remaining;
    if remaining_dist > distance {
        wins += 1;
    }
}
```

Only takes a couple milliseconds to complete, which came as a slight surprise.

## My Rust experience so far

Given that I'm doing things in crude ways and not really being punished for it, I'm just hacking away with the simplest possible solutions I can muster while letting Rust's compilation do the heavy lifting. This may end up with my creating really bad code in the future, only to be saved by powerful tools that make it look trivial on human time scales. :)

Other than that, so far so good! I'll keep trucking along and building more stuff in Rust from here on out. Hopefully I can learn some better algos along the way so I won't look so clumsy.

### Rust API hacking

Side note, I took time yesterday to start hacking away with Rust in the Warp framework to create a sample API service inside one of our existing Python microservice monorepos (yes, that's a thing). Can't share more details than that, but I'm seeing parallels here with writing, say, a Flask application, in that there are so many batteries lacking from the implementation that I need to fill in myself.

Aside from that, got a Docker build chain working that builds the app inside a stage and carts it over to our other one. That part's working out great! We already have stuff running in Tilt using Helm, so I smashed those bits together and got some good responses.

Will need to spend more time hardening the thing, making usable entity patterns in separate modules, incorporating structlogging within our chosen cloud platform, as well as interacting with cloud services. The journey continues. 🦀
