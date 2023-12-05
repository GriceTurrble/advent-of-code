# Advent of Code 2023 Day 4

Link: <https://adventofcode.com/2023/day/4>

## Part 1

Relatively simple implementation, constructing a HashSet of the winning numbers and the game numbers, then get the intersection between them. The length of that intersection informs the calculation: if 0 matches, add 0 to the total; if 1+, add `2^(n-1)` to the total, where `n` is number of matches.

## Part 2

A bit tougher on this one, while also a little simpler. I think I need to construct a HashMap keyed by card IDs and containing an int with number of games to run. We can take number of winning numbers from one game and add new "games" to the keys of points ahead of the current position equal to the number of copies of the current game, up to the number of games played in the current set. Might be a simpler method out there, but I'll run with it for now.

The running total will just be the count of the number of games run, so as we move forward we total up number of games executed.
