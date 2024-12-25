from __future__ import annotations

import time
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"

LockKeyType = tuple[int, int, int, int, int]


def transpose_keys(grid: list[str]) -> list[str]:
    """Transposes a matrix, so rows become columns and vice versa."""
    return list(map("".join, zip(*grid, strict=True)))


def convert_to_nums(grid: list[str]) -> LockKeyType:
    transposed = transpose_keys(grid=grid)
    return tuple([x.count("#") - 1 for x in transposed])  # type: ignore


def num_not_overlapping(one: LockKeyType, others: list[LockKeyType]) -> int:
    total = 0
    maximum = 5
    for other in others:
        if any([x for x in map(sum, zip(one, other, strict=True)) if x > maximum]):
            continue
        total += 1
    return total


def part1(contents: str) -> int:
    total = 0
    sets = contents.split("\n\n")
    locks: list[LockKeyType] = []
    keys: list[LockKeyType] = []
    for lock_or_key in sets:
        grid = lock_or_key.strip().split("\n")
        nums = convert_to_nums(grid=grid)
        if "#" in grid[0]:
            total += num_not_overlapping(one=nums, others=keys)
            locks.append(nums)
        else:
            total += num_not_overlapping(one=nums, others=locks)
            keys.append(nums)
    return total


def part2(contents: str) -> int:
    total = 0
    # TODO part 2
    return total


def main():
    contents = FILE.read_text().strip()

    _start1 = time.perf_counter()
    result1 = part1(contents=contents)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(contents=contents)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
