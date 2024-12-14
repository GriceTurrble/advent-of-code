from __future__ import annotations

import time
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"


def _calc_fence_perimeter(grid: list[str], x: int, y: int, char: str) -> int:
    len_x, len_y = len(grid), len(grid[0])
    fences = 0
    for dx, dy in [
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 0),
    ]:
        newx, newy = x + dx, y + dy
        if not 0 <= newx < len_x or not 0 <= newy < len_y:
            # Position is outside the grid: this side naturally has a fence.
            fences += 1
        elif char != grid[newx][newy]:
            # The adjacent region is not the same as our current one,
            # so a dividing fence is needed.
            fences += 1
        # Natural 'else' is an adjacent region with the same type of flower,
        # so no fence is needed there.
    return fences


MAX_FENCE = 4


def part1(grid: list[str]) -> int:
    total = 0
    for x, line in enumerate(grid):
        for y, char in enumerate(line):
            fences_here = _calc_fence_perimeter(grid=grid, x=x, y=y, char=char)
            if fences_here == MAX_FENCE:
                # This is a self-contained region: no further calculation needed.
                # Area is 1, with 4 fences
                total += fences_here
                continue
    return total


def part2(grid: list[str]) -> int:
    total = 0
    # TODO part 2
    return total


def main():
    grid = FILE.read_text().strip().split("\n")

    _start1 = time.perf_counter()
    result1 = part1(grid=grid)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(grid=grid)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
