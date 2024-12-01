from __future__ import annotations

from collections import defaultdict
from pathlib import Path
import bisect
import typing

if typing.TYPE_CHECKING:
    from io import TextIOWrapper

FILE = Path(__file__).parent / "inputs.txt"


def part1(inputs: TextIOWrapper) -> int:
    total = 0
    lefts: list[int] = []
    rights: list[int] = []
    for line in inputs:
        left, right = line.strip().split()
        bisect.insort(lefts, int(left))
        bisect.insort(rights, int(right))

    for lnum, rnum in zip(lefts, rights):
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


def main():
    with open(FILE) as f:
        print(f"PART 1: {part1(f)}")

    with open(FILE) as f:
        print(f"PART 2: {part2(f)}")


if __name__ == "__main__":
    main()
