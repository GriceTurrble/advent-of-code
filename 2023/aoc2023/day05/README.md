# Advent of Code 2023 Day 5

Link: <https://adventofcode.com/2023/day/5>

## Part 1

The description here seems confusing for some (I was reading PyDis chat during the night as this opened up, and felt that confusion from folks), so I'll start with my understanding of the breakdown.

A map entry from the sample reads like so:

```
seed-to-soil map:
50 98 2
52 50 48
```

The *second* number in each line indicates the **lower bound** of a **range**, and the *third* number is the **length** of that range. So, `98` lower bound (inclusive) with length `2` equates to a range of `98..(98+2)` aka `98..100`, which covers numbers `(98, 99)`.

Reading the second line, `50` length `48` is a range of `50..(50+48)`, aka `50..98`, covering numbers `(50, 51, ..., 97)` (inclusive lower bound, exclusive upper bound).

The *first* number identifies the lower bound of a destination range with the same length as the source range. So the first line is `50..52` (`(50, 51)` only), the second line `52..(52+48)` aka `52..100` (`(52, 53, ..., 99)`).

Finally, any number not accounted for in a defined source range equates to the *same* number in a destination. Thus, example seed `14` falls outside both of the source ranges, therefor its destination is `14`

Considering all the mappings together, we need to track the source range in each mapping and the *difference* between the source and destination. Example, `50 98 2`, for seeds in `(98, 99)`, we add `-48` to the seed number to end up in the correct destination.

This process repeats for each kind of unique mapping:

```mermaid
flowchart LR
    seed --> soil
    soil --> fertilizer
    fertilizer --> water
    water --> light
    light --> temperature
    temperature --> humidity
    humidity --> location
```

This mapping appears to match up perfectly in the input data, which might lead to just writing special-case code for each kind of mapping. However, it would probably be easier just to construct a `Vector` of `HashMap`s keyed by `range`s to `i64` values (big numbers), so that we can iterate through them to obtain our final values at the end (without needing to identify them).

And once we calculate those final values in some collection, we just find the minimum value to obtain our solve.

### Challenges

- Need to test a value's membership inside a set of ranges (map them?)
- If we don't find a matching value in the source ranges, then the new value is the same as the original.
- If we do find a matching range, calculate the new value using the stored diff.
- Building the mappings shouldn't be too difficult. We need to split up our content by double newlines to identify different sections. We can then parse each section individually to build and return a `struct` containing the `range` and `diff` (I said HashMap at first; I thing it just makes more sense to make a small struct instead of reusing the thing, since we don't really need to perform lookups in the map).

  ```rs
  struct SeedMap {
      source: std::ops::Range,
      diff: i64,
  }
  ```

  Just a Vec of these structs should be enough. Have a function that returns this Vec given the string contents of the input and stuff.

- I'll need to include a simpler func in the `lube` crate that returns the entire file contents, not the Vec of Strings as before.

  That or I can make a more specific utility that accepts a delimiter on which to split the contents, then refactor to use it in the other func (sounds like a winner to me).

## Part 2

Now we need to deal with these seed values coming in pairs, each pair determining a range (lower bound and length). Alright then.

I think I'll initially try brute-forcing this one. Keep the same methodology with the mappings, and just generate seed values out of these new ranges. Most of the machinery should stay the same.

*One hour later...*

Didn't quite work out in debug mode. Thankfully, running it with the release flag helps speed things up considerably:

```shell
cargo run -rp day05 -- day05/input.txt
```

This got it to the right answer relatively fast, though not *really* that fast (still took about a minute to process).

A better solution would have been something like what I see here: <https://www.youtube.com/watch?v=NmxHw_bHhGM>

But, works for me for now! I'm sure I'll figure out the more clever/better solutions some other time. :)
