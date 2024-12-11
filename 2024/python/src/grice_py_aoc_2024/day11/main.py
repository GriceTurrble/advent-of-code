from __future__ import annotations

import time
from functools import cache
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"


@cache
def blink(num: int, iters: int) -> int:
    # Borrowed some insight from folks in PyDis.
    # We don't need to store most of the real numbers, just the number of stones that
    # result from iterating on a particular stone N times.
    # Combined with caching results of stone X iterated N times,
    # we get a very quick response in both parts.
    if iters == 0:
        return 1
    if num == 0:
        return blink(1, iters - 1)
    if len(numstr := str(num)) % 2 == 0:
        half = len(numstr) // 2
        return blink(int(numstr[half:]), iters - 1) + blink(
            int(numstr[:half]),
            iters - 1,
        )
    return blink(num * 2024, iters - 1)


def part1(contents: list[int]) -> int:
    return sum(blink(x, 25) for x in contents)


def part2(contents: list[int]) -> int:
    return sum(blink(x, 75) for x in contents)


def main():
    contents = list(map(int, FILE.read_text().strip().split()))

    _start1 = time.perf_counter()
    result1 = part1(contents)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(contents)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
