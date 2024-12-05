from __future__ import annotations

from pathlib import Path

from .main import part1, part2

TEST_FILE_P1 = Path(__file__).parent / "test_inputs_p1.txt"
TEST_FILE_P2 = Path(__file__).parent / "test_inputs_p2.txt"
EXPECTED_PART_1 = 18
EXPECTED_PART_2 = 9


def test_part1():
    contents = TEST_FILE_P1.read_text()
    result = part1(contents)
    assert result == EXPECTED_PART_1


def test_part2():
    contents = TEST_FILE_P2.read_text()
    result = part2(contents)
    assert result == EXPECTED_PART_2
