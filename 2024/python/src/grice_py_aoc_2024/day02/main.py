from __future__ import annotations

from pathlib import Path
import typing

if typing.TYPE_CHECKING:
    from io import TextIOWrapper

FILE = Path(__file__).parent / "inputs.txt"


def part1(inputs: TextIOWrapper):
    raise NotImplementedError


def part2(inputs: TextIOWrapper):
    raise NotImplementedError


def main():
    with open(FILE) as f:
        print(f"PART 1: {part1(f)}")

    with open(FILE) as f:
        print(f"PART 2: {part2(f)}")


if __name__ == "__main__":
    main()
