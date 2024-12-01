from __future__ import annotations

import time
import typing
from collections import defaultdict
from pathlib import Path

if typing.TYPE_CHECKING:
    from io import TextIOWrapper

FILE = Path(__file__).parent / "inputs.txt"


def part1(inputs: TextIOWrapper) -> int:
    total = 0
    lefts: list[int] = []
    rights: list[int] = []
    for line in inputs:
        left, right = line.strip().split()
        lefts.append(int(left))
        rights.append(int(right))
    lefts.sort()
    rights.sort()

    for lnum, rnum in zip(lefts, rights, strict=False):
        total += abs(rnum - lnum)
    return total


def part2(inputs: TextIOWrapper) -> int:
    lefts: defaultdict[int, int] = defaultdict(int)
    rights: defaultdict[int, int] = defaultdict(int)

    for line in inputs:
        left, right = line.strip().split()
        lefts[int(left)] += 1
        rights[int(right)] += 1

    total = 0
    for num, count in lefts.items():
        total += count * (num * rights[num])
    return total


def main() -> None:
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
