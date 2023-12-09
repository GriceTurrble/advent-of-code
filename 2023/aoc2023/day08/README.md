# Advent of Code 2023 Day 8

Link: <https://adventofcode.com/2023/day/8>

## Part 1

This was relatively simple in the setup: create a mapping from starter nodes to tuples of its destinations for L and R, then `.cycle()` through the directions and traverse nodes (getting the key of the new direction and accessing that new set) until we landed on `ZZZ`. Nothing too complex.

## Part 2

On trying to get this running in a brute force manner, I realized this would take too long to complete. Obviously there's a mathematical trick involved in most of these part-2s, so I went looking for answers.

At first folks were mentioning the Chinese Remainder Theorem, but that seemed too complicated to set up.

I noticed someone mention LCM (Least Common Multiple), and gave that a shot. I just had to run the original method for one cycle for each starting point, count the number of steps for each, and progressively obtain the LCM of those step numbers (starting from `1` as a basis). Sure enough, that was the answer!
