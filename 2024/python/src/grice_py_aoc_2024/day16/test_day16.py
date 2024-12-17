from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE_EX1 = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_ex1.txt"
TEST_FILE_EX2 = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_ex2.txt"
EXPECTED_PART_1_EX1 = 7_036
EXPECTED_PART_1_EX2 = 11_048
EXPECTED_PART_2 = 456  # TODO replace!


@pytest.mark.skip(reason=f"{DIR.stem} P1 incomplete")
@pytest.mark.parametrize(
    "file, expected",
    [
        (TEST_FILE_EX1, EXPECTED_PART_1_EX1),
        (TEST_FILE_EX2, EXPECTED_PART_1_EX2),
    ],
)
def test_part1(file: Path, expected: int):
    contents = file.read_text()
    result = part1(contents=contents)
    assert result == expected


@pytest.mark.skip(reason=f"{DIR.stem} P2 incomplete")
def test_part2():
    contents = TEST_FILE_EX1.read_text()
    result = part2(contents=contents)
    assert result == EXPECTED_PART_2
