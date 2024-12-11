from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 36
EXPECTED_PART_2 = 81


def test_part1():
    contents = TEST_FILE.read_text().strip().split("\n")
    result = part1(contents)
    assert result == EXPECTED_PART_1


def test_part2():
    contents = TEST_FILE.read_text().strip().split("\n")
    result = part2(contents)
    assert result == EXPECTED_PART_2
