from __future__ import annotations

from pathlib import Path

from .main import part1, part2

TEST_FILE = Path(__file__).parent / "test_inputs.txt"
RESULT_PART_1 = "Now it's done!"
RESULT_PART_2 = "Now it's done!"


def test_part1():
    with open(TEST_FILE) as f:
        result = part1(f)
    assert result == RESULT_PART_1


def test_part2():
    with open(TEST_FILE) as f:
        result = part2(f)
    assert result == RESULT_PART_2
