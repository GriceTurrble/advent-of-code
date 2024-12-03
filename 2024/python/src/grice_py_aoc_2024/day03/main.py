from __future__ import annotations

import re
import time
from pathlib import Path

FILE = Path(__file__).parent / "inputs.txt"


def part1(contents: str) -> int:
    mul_pat = re.compile(r"mul\(\d+,\d+\)")
    content = mul_pat.findall(contents)
    result = 0
    for bit in content:
        left, right = map(int, bit[4:-1].split(","))
        result += left * right
    return result


def part2(contents: str) -> int:
    mul_pat = re.compile(r"do\(\)|don't\(\)|mul\(\d+,\d+\)")
    content = mul_pat.findall(contents)
    result = 0
    enabled = True
    for bit in content:
        if bit in ("do()", "don't()"):
            enabled = bit == "do()"
            continue
        if enabled:
            left, right = map(int, bit[4:-1].split(","))
            result += left * right
    return result


def main():
    contents = FILE.read_text()

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
