from __future__ import annotations

import time
from functools import cache
from itertools import product
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"

TRAILHEAD = "0"
END_OF_TRAIL = "9"


def num_trails(contents: list[str], x: int, y: int, unique: bool = False) -> int:
    @cache
    def _find_all_trail_ends(x: int, y: int) -> set[tuple[int, int]]:
        nonlocal contents
        curr = contents[x][y]
        if curr == END_OF_TRAIL:
            return {(x, y)}

        target = str(int(curr) + 1)
        trailheads = set()
        for dx, dy in [
            [-1, 0],
            [0, 1],
            [1, 0],
            [0, -1],
        ]:
            newx, newy = x + dx, y + dy
            if not 0 <= newx < len(contents) or not 0 <= newy < len(contents[0]):
                # new point is outside bounds of the contents, ignore
                continue
            if contents[newx][newy] == target:
                trailheads |= _find_all_trail_ends(x=newx, y=newy)
        return trailheads

    @cache
    def _find_num_unique_trails(x: int, y: int) -> int:
        # Oddly enough, this was the way I initially tried to solve Part 1
        # due to my misunderstanding things. I had to then go back and return a set
        # of those trail end coordinates, so as to have a unique set of trail ends,
        # which I could then count with len.
        # After finding that Part 2 was, indeed, this version,
        # I just re-built what I did at first and re-implemented with a flag.
        nonlocal contents
        curr = contents[x][y]
        if curr == END_OF_TRAIL:
            return 1

        target = str(int(curr) + 1)
        trails = 0
        for dx, dy in [
            [-1, 0],
            [0, 1],
            [1, 0],
            [0, -1],
        ]:
            newx, newy = x + dx, y + dy
            if not 0 <= newx < len(contents) or not 0 <= newy < len(contents[0]):
                # new point is outside bounds of the contents, ignore
                continue
            if contents[newx][newy] == target:
                trails += _find_num_unique_trails(x=newx, y=newy)
        return trails

    if unique:
        # Part 2
        return _find_num_unique_trails(x=x, y=y)

    # Part 1
    trailhead_set = _find_all_trail_ends(x=x, y=y)
    return len(trailhead_set)


def part1(contents: list[str]) -> int:
    total = 0
    for x, line in enumerate(contents):
        for y, char in enumerate(line):
            if char == TRAILHEAD:
                num = num_trails(contents=contents, x=x, y=y)
                total += num
    return total


def part2(contents: list[str]) -> int:
    total = 0
    for x, line in enumerate(contents):
        for y, char in enumerate(line):
            if char == TRAILHEAD:
                num = num_trails(contents=contents, x=x, y=y, unique=True)
                total += num
    return total


def main():
    contents = FILE.read_text().strip().split("\n")

    _start1 = time.perf_counter()
    result1 = part1(contents)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(contents)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
