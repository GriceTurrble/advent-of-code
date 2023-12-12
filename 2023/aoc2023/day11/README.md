# Advent of Code 2023 Day 11

Link: <https://adventofcode.com/2023/day/11>

## Part 1

First and foremost we need a method for "expanding the universe", which is *probably* best done through just some mathematical equation after identifying those rows and columns, rather than actually adjusting the input grid. But where's the fun in that?

It's a simple matter of `.clone()`ing and `.push()`ing something onto an existing vec, so handling rows is simple. Columns are less simple, of course, but we don't need to worry about them much: just transpose the grid and then perform the same push operation on the new "rows".

After that, search for all galaxies, determining their positions in the grid; then use [`itertools.combination`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.combinations) to produce the various points we need to worry about. From there it's a simple matter of calculating their distances (essentially `abs(x1 - x2) + abs(y1 - y2)`), and then summing that value.

## Part 2

Sure enough, performing the whole insert and transpose ain't gonna cut it: we can't go shoving 1_000_000 rows in every time we find something. We'd quickly run out of memory!

Instead, we'll need to identify those rows and cols separately as being empty. I think then we can test if the range between, say, `x1..x2` includes one or more of those rows. If so, we add `999_999` (accounting for an off-by-one issue) to the distance calculation manually.

Interestingly, the test cases use `n` of `10` and `100` to demonstrate different values. This leads us to write the func taking an argument; one which could be used to solve the first part in the same way.

So, while the original did include a `part1.rs` and `part2.rs`, I opted to instead use a `funcs` module to move pieces out of the main module (trying to increase some clarity by reducing the lines of code per file). Then we move the solution from part 2 back to `main.rs`, rename to `fn solve()`, and add the `expansion: u64` variable (in place of `n`). Part 1 is now solved by the same method, and tests are run against all 3 examples.
