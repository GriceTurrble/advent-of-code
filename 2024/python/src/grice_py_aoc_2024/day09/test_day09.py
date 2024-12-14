from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 1928
EXPECTED_PART_2 = "REPLACE ME!"


@pytest.mark.skip(reason=f"{DIR.stem} P1 incomplete")
def test_part1():
    contents = TEST_FILE.read_text().strip()
    result = part1(contents)
    assert result == EXPECTED_PART_1


@pytest.mark.skip(reason=f"{DIR.stem} P2 incomplete")
def test_part2():
    contents = TEST_FILE.read_text().strip()
    result = part2(contents)
    assert result == EXPECTED_PART_2
