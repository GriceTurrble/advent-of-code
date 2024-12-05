from __future__ import annotations

import time
import typing
from pathlib import Path

if typing.TYPE_CHECKING:
    from io import TextIOWrapper

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"
MIN_DELTA = 1
MAX_DELTA = 3


def _delta_is_safe(
    delta: int,
    increasing: bool,
) -> bool:
    if not (MIN_DELTA <= abs(delta) <= MAX_DELTA):
        # Not within the safe range of change = unsafe
        return False
    if increasing != (delta > 0):
        # Mismatch in increase/decrease is unsafe
        return False
    # Safe conditions detected
    return True


def _is_safe(report: list[int]) -> bool:
    incr = None
    for i in range(len(report) - 1):
        new_delta = report[i + 1] - report[i]
        # detect initial "increasing" value or leave it unchanged
        incr = new_delta > 0 if incr is None else incr
        if not _delta_is_safe(delta=new_delta, increasing=incr):
            return False
    # Reached the end and all conditions are fine.
    return True


def part1(inputs: TextIOWrapper) -> int:
    num_safe = 0
    for line in inputs:
        report = list(map(int, line.strip().split()))
        num_safe += 1 if _is_safe(report) else 0
    return num_safe


def part2(inputs: TextIOWrapper) -> int:
    num_safe = 0
    for line in inputs:
        report = list(map(int, line.strip().split()))
        if _is_safe(report):
            # If the unchanged report is safe, mark it and move on to the next one.
            num_safe += 1
            continue

        # This report is marked unsafe by the first pass

        for i in range(len(report)):
            # Create a new report that omits one level at a time
            new_report = report[:i] + report[i + 1 :]
            # ...then check that new version for safety
            if _is_safe(new_report):
                num_safe += 1
                break
    return num_safe


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
