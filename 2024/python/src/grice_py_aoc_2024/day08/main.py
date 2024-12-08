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
    """Calculates antinodes for the given two points.

    In Part 1, we have 0 to 2 antinodes, on either side of `c1` and `c2`.
    In Part 2, we have 0+ antinodes, repeating at new positions in a line that includes
    both `c1` and `c2`.
    """
    len_col, len_row = len(grid), len(grid[0])
    new_antinodes: set[CoordType] = set()

    if part2:
        # In part 2 only, the current nodes are both included in the set of antinodes
        new_antinodes = {c1, c2}

    # We need to calculate antinodes in two directions:
    # From C1 -> C2 -> ?
    # From C2 -> C1 -> ?
    directions = [(c1, c2), (c2, c1)]
    for first, second in directions:
        # Identify node A and B for the "current" iteration on this line.
        # (relevant for Part 2, when these values will change)
        curr_a, curr_b = first, second

        # A loop is not needed in Part 1, but we are making this flexible.
        # Later, if we detect we're in Part 1, we break on the first iteration, anyway.
        while True:
            new_node = (
                (2 * curr_b[0]) - curr_a[0],
                (2 * curr_b[1]) - curr_a[1],
            )
            # Test that the node is within the bounds of the grid
            if -1 < new_node[0] < len_col and -1 < new_node[1] < len_row:
                new_antinodes.add(new_node)
                if not part2:
                    break
                # Shift our pointers so we now consider B and the new node, C,
                # in the next iteration to generate a new antinode.
                curr_a, curr_b = curr_b, new_node
            else:
                break

    return new_antinodes


def num_antinodes(
    grid: list[str],
    part2: bool = False,
) -> int:
    """General method for getting the number of antinodes."""

    # Our final answer, in both parts, is just the length of this dict
    # (the number of unique nodes present). During planning, I supposed that
    # Part 2 may want to know how many antinodes i.e. resonate with multiple antenna.
    # So, knowing a list of antenna signals this antinode relates to would be helpful.
    antinodes: dict[CoordType, list[str]] = defaultdict(list)

    # In this dict, we key off the antenna signal, then include a set of antenna
    # already seen. As we encounter an antenna in the search, we can recall the
    # set of antenna already seen with the same signal, then generate antinodes
    # for each pair of those signals (the new antenna, compared to every previous one).
    # This could also be achieved by simply collecting all antenna coords first,
    # then working onut the solution with itertools permutations with n=2, for instance.
    nodes: dict[str, set[CoordType]] = defaultdict(set)

    for x, line in enumerate(grid):
        for y, char in enumerate(line):
            if char == ".":
                # Not an antenna, skip it.
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
                    # Add this signal to the list of signals attributed to each antinode
                    antinodes[new_antinode].append(char)

            # Add this new signal coordinate to the list of antenna seen for this signal
            nodes[char].add(this_coord)

    # Our final result is the number of unique keys in the antinodes dict.
    return len(antinodes)


def part1(grid: list[str]) -> int:
    """Solve for Part 1."""
    return num_antinodes(grid=grid)


def part2(grid: list[str]) -> int:
    """Solve for Part 2."""
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
