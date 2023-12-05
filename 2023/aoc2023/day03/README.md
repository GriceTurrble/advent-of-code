# Advent of Code 2023 Day 3

Link: <https://adventofcode.com/2023/day/3>

## Part 1

We need to identify numbers in the schematic, which is the first issue.
For that we should probably identify them via regex and find a means to parse out thier positions
(either the start pos or the bounding box).
If we know a number's position, presumably we can form a box around that number and get all symbols in that box.
If any symbols are present other than `.`, then the part is a part number and should be pulled.

So, we take substrings that contain and are adjacent to a number
(row above and below, one space to left and right included)
and test whether those substrings contain a symbol other than a digit or `.`.
When one is found, we total up the numbers and find our part 1 solution.

## Part 2
