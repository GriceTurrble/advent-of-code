from __future__ import annotations

from pathlib import Path

from .main import parse_input, part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 12


def test_part1():
    grid_size = (11, 7)
    robots = parse_input(TEST_FILE.read_text())
    result = part1(robots=robots, grid_size=grid_size)
    assert result == EXPECTED_PART_1


# There is no testable solution for part 2 today.
# You just have to run on your own input and see what comes out. :)
