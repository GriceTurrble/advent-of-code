from __future__ import annotations

from pathlib import Path

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 18
EXPECTED_PART_2 = 9


def test_part1():
    contents = TEST_FILE.read_text()
    result = part1(contents)
    assert result == EXPECTED_PART_1


def test_part2():
    contents = TEST_FILE.read_text()
    result = part2(contents)
    assert result == EXPECTED_PART_2
