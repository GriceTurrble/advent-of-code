from __future__ import annotations

import time
from collections import deque
from pathlib import Path
from typing import TypeVar

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"

directions = deque(["up", "right", "down", "left"])

GUARD = "^"


def _find_guard(grid: list[str]) -> tuple[int, int]:
    for x, line in enumerate(grid):
        if GUARD in line:
            return (x, line.index(GUARD))
    raise ValueError("Did not find the guard!")


T = TypeVar("T")


def _new_direction(directions: deque[T]) -> T:
    """Returns the direction at index 0.

    Directions is mutated by this function: it is rotated,
    moving the first direction to the end of the sequence.
    """
    direction = directions[0]
    directions.rotate(-1)
    return direction


def part1(grid: list[str]) -> int:
    guard = _find_guard(grid=grid)
    len_col, len_row = len(grid), len(grid[0])
    directions = deque(
        [
            (-1, 0),  # up
            (0, 1),  # right
            (1, 0),  # down
            (0, -1),  # left
        ],
    )

    # Get the starting direction.
    direction = _new_direction(directions)

    # Gather set of all coordinates we've seen
    visited = set()
    visited.add(guard)

    while True:
        next_space = (guard[0] + direction[0], guard[1] + direction[1])
        nx, ny = next_space
        if not (0 <= nx < len_col) or not (0 <= ny < len_row):
            # The next space is outside the walls; the guard leaves the area
            break
        if grid[nx][ny] == "#":
            # Guard hits an obstacle.
            # Does not move, but changes direction for next time.
            direction = _new_direction(directions)
        else:
            # The guard moves into the new space.
            guard = (nx, ny)
            # Add this position to visited spaces
            visited.add(guard)

    return len(visited)


def part2(grid: list[str]):
    # TODO
    return "Not done yet!"


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
