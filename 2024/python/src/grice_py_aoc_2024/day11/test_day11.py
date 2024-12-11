from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 55312
# The problem actually does not report this number.
# After solving it naturally, this result came back in tests.
# So, we roll with it!
EXPECTED_PART_2 = 65601038650482


def test_part1():
    contents = list(map(int, TEST_FILE.read_text().strip().split()))
    result = part1(contents)
    assert result == EXPECTED_PART_1


def test_part2():
    contents = list(map(int, TEST_FILE.read_text().strip().split()))
    result = part2(contents)
    assert result == EXPECTED_PART_2
