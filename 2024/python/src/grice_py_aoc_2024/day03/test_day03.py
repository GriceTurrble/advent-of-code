from __future__ import annotations

from pathlib import Path

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE_P1 = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_p1.txt"
TEST_FILE_P2 = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_p2.txt"
RESULT_PART_1 = 161
RESULT_PART_2 = 48


def test_part1():
    contents = TEST_FILE_P1.read_text()
    result = part1(contents)
    assert result == RESULT_PART_1


def test_part2():
    contents = TEST_FILE_P2.read_text()
    result = part2(contents)
    assert result == RESULT_PART_2
