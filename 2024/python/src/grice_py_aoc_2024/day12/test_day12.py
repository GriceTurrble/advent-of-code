from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 1930
EXPECTED_PART_2 = 456  # TODO replace!


@pytest.mark.skip(reason=f"{DIR.stem} P1 incomplete")
def test_part1():
    grid = TEST_FILE.read_text().strip().split("\n")
    result = part1(grid=grid)
    assert result == EXPECTED_PART_1


@pytest.mark.skip(reason=f"{DIR.stem} P2 incomplete")
def test_part2():
    grid = TEST_FILE.read_text().strip().split("\n")
    result = part2(grid=grid)
    assert result == EXPECTED_PART_2
