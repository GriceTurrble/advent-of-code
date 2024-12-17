from __future__ import annotations

from pathlib import Path

import pytest

from .main import part1, part2

DIR = Path(__file__).parent
TEST_FILE_EX1 = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_ex1.txt"
TEST_FILE_EX2 = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_ex2.txt"
EXPECTED_PART_1_EX1 = 10092
EXPECTED_PART_1_EX2 = 2028
EXPECTED_PART_2 = 9021


@pytest.mark.parametrize(
    "file, expected",
    [
        (TEST_FILE_EX1, EXPECTED_PART_1_EX1),
        (TEST_FILE_EX2, EXPECTED_PART_1_EX2),
    ],
)
def test_part1(file: Path, expected: int):
    result = part1(contents=file.read_text())
    assert result == expected


def test_part2():
    # We have a test value for the "larger" sample only,
    # which is our `EX1` file.
    result = part2(contents=TEST_FILE_EX1.read_text())
    assert result == EXPECTED_PART_2
