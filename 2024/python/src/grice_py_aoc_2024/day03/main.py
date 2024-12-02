from __future__ import annotations

import time
import typing
from pathlib import Path

if typing.TYPE_CHECKING:
    from io import TextIOWrapper

FILE = Path(__file__).parent / "inputs.txt"


def part1(inputs: TextIOWrapper):
    return "Not done yet!"


def part2(inputs: TextIOWrapper):
    return "Not done yet!"


def main():
    _start1 = time.perf_counter()
    with open(FILE) as f:
        result1 = part1(f)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    with open(FILE) as f:
        result2 = part2(f)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
