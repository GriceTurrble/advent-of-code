from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}.txt"
EXPECTED_PART_1 = 55312
EXPECTED_PART_2 = "REPLACE ME!"


@pytest.mark.skip(reason=f"{DIR.stem} P1 incomplete")
def test_part1():
    contents = list(map(int, TEST_FILE.read_text().strip().split()))
    result = part1(contents)
    assert result == EXPECTED_PART_1


@pytest.mark.xfail(reason=f"{DIR.stem} P1 incomplete")
def test_part2():
    contents = list(map(int, TEST_FILE.read_text().strip().split()))
    result = part2(contents)
    assert result == EXPECTED_PART_2
