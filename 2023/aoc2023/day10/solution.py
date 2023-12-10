"""2023 Day 10 solution, adapted from a community member on Python Discord
(sorry, jenna1234!)

I know I didn't solve this on my own, but I recognized part 2 today
(involving checking if points on a map lie inside a polygon) falls well
outside my expertise.

I did note some useful suggestions such as doubling the size of the map
in order to expose gaps within the pipes, then floodfilling the empty
spaces. I just cannot fathom how to achieve that with an algo I can
write myself.

So, rather than try to wrap my head around this at 3AM on a Sunday (also
the day before I leave for a work trip for 4 days), I looked for
solutions offered by others and ran with it.

I intend to come back here later and try to translate this method into a
working solution in Rust for the sake of completion, though. My goal
this year is learning Rust, after all: I can still do that just by
translating a working program from another language. :)
"""

from itertools import count
from pathlib import Path

file = Path(__file__).parent / "input.txt"
lines = [list(x) for x in file.read_text().strip().split("\n")]

y = next(y for y, line in enumerate(lines) if "S" in line)
x = lines[y].index("S")
lines[y][x] = "7"  # ???

pipe = set()
unchecked = {(x, y)}
for part1 in count():
    pipe.update(unchecked)
    unchecked = {
        coord
        for x, y in unchecked
        for coord in {
            "-": ((x - 1, y), (x + 1, y)),
            "|": ((x, y - 1), (x, y + 1)),
            "F": ((x + 1, y), (x, y + 1)),
            "J": ((x - 1, y), (x, y - 1)),
            "L": ((x + 1, y), (x, y - 1)),
            "7": ((x - 1, y), (x, y + 1)),
        }[lines[y][x]]
        if coord not in pipe
    }
    if not unchecked:
        break
print("Part 1", part1)

part2 = 0
for y, line in enumerate(lines):
    is_inside = -1
    for x, c in enumerate(line):
        if (x, y) not in pipe:
            part2 += is_inside == 1
        elif c == "|":
            is_inside *= -1
        elif c in "FJ":
            is_inside *= 1j
        elif c in "L7":
            is_inside *= -1j
print("Part 2", part2)
