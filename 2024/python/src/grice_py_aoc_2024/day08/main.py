from __future__ import annotations

import time
from collections import defaultdict
from pathlib import Path
from typing import Protocol

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"

CoordType = tuple[int, int]
"""Generic coordinate, 2-tuple of ints."""


class CalcerType(Protocol):
    """Protocol for a caculator func that can be passed to `num_antinodes` function."""

    def __call__(
        self,
        grid: list[str],
        c1: CoordType,
        c2: CoordType,
    ) -> set[CoordType]: ...


def _calc_anti_nodes(
    grid: list[str],
    c1: CoordType,
    c2: CoordType,
    part2: bool = False,
) -> set[CoordType]:
    """Calculates antinodes for the given two points, in part 1."""
    len_col, len_row = len(grid), len(grid[0])
    permutations = [(c1, c2), (c2, c1)]
    new_antinodes: set[CoordType] = set()

    if part2:
        # Preset the "new" antinodes to include the current nodes
        new_antinodes = {c1, c2}

    for first, second in permutations:
        curr_a, curr_b = first, second
        while True:
            new_node = (
                (2 * curr_b[0]) - curr_a[0],
                (2 * curr_b[1]) - curr_a[1],
            )
            if -1 < new_node[0] < len_col and -1 < new_node[1] < len_row:
                new_antinodes.add(new_node)
                # TODO accurate swap?
                curr_a, curr_b = curr_b, new_node
                if not part2:
                    break
            else:
                break

    return new_antinodes


def num_antinodes(grid: list[str], part2: bool = False) -> int:
    """General method for getting the number of antinodes."""
    antinodes: dict[CoordType, list[str]] = defaultdict(list)
    nodes: dict[str, set[CoordType]] = defaultdict(set)

    for x, line in enumerate(grid):
        for y, char in enumerate(line):
            if char == ".":
                # skip non-antenna
                continue
            this_coord = (x, y)
            for seen_node in nodes[char]:
                new_antinodes = _calc_anti_nodes(
                    grid=grid,
                    c1=seen_node,
                    c2=this_coord,
                    part2=part2,
                )
                for new_antinode in new_antinodes:
                    antinodes[new_antinode].append(char)
            nodes[char].add(this_coord)

    return len(antinodes)


def part1(grid: list[str]) -> int:
    """Part 1 solver."""
    return num_antinodes(grid=grid)


def part2(grid: list[str]) -> int:
    """Part 2 solver."""
    return num_antinodes(grid=grid, part2=True)


def main():
    """Entrypoint."""
    grid = FILE.read_text().strip().split("\n")

    _start1 = time.perf_counter()
    result1 = part1(grid)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(grid)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
