from __future__ import annotations

import time
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"


def part1(contents: str) -> str:
    result = ""
    # TODO part 1
    return result


def part2(contents: str) -> str:
    result = ""
    # TODO part 2
    return result


def main():
    contents = FILE.read_text()

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
