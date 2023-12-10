# Advent of Code 2023 Day 10

Link: <https://adventofcode.com/2023/day/10>

## Part 1

I figured out fairly quickly that the "furthest point" from the start was essentially the midpoint along the path, so I set about counting steps around the path until the loop was completed. To do this meant identifying the start point, then traversing connecting points along the way.

Since all points that are not ground `.` are pipes with two endpoints (including S, though discovering its true identity was a task all its own), I figured I would always have 3 points of interest: the `previous`, the `current`, and the `next`. By inspecting any `current` point and identifying the two connecting points, I would be guaranteed that one of those connections was the `previous`, and the other would need to be the `next`. So it was a simple matter of counting steps until the `next` step equated to the same position as the `start`.

A couple off-by-one errors later, I got a solution working.

## Part 2

See [solution.py](solution.py).

As noted, I'll return here soon (probably in January) and try to rewrite this method into Rust.
