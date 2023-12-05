# Advent of Code 2023 Day 3

Link: <https://adventofcode.com/2023/day/3>

## Part 1

We need to identify numbers in the schematic, which is the first issue. For that we should probably identify them via regex and find a means to parse out thier positions (either the start pos or the bounding box). If we know a number's position, presumably we can form a box around that number and get all symbols in that box. If any symbols are present other than `.`, then the part is a part number and should be pulled.

So, we take substrings that contain and are adjacent to a number (row above and below, one space to left and right included) and test whether those substrings contain a symbol other than a digit or `.`. When one is found, we total up the numbers and find our part 1 solution.

## Part 2

Target numbers are only those near a `*` character, so we can skip most of the processing the looks for numbers in the first place. In fact, going from the numbers themselves could be problematic, since we would need to jump from a number to a `*` and then daisychain to another number.

Thinking about it as I write, actually that wouldn't be terrible, but we can't do it in one pass. We'd go from finding a number on a line to finding a nearby `*`, then record the position of that asterisk as a point in a HashMap. We could then map that point to a `Vec<i32>`, onto which we can push the number we just found.

After that pass, it's a simple matter of identifying points that have attached Vectors of exactly two numbers (applying a filter). With those combinations of numbers in hand, we obtain their products, sum the total, and there's our answer.
