from __future__ import annotations

from pathlib import Path

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
RESULT_PART_1 = 11
RESULT_PART_2 = 31


def test_part1():
    with open(TEST_FILE) as f:
        result = part1(f)
    assert result == RESULT_PART_1


def test_part2():
    with open(TEST_FILE) as f:
        result = part2(f)
    assert result == RESULT_PART_2
