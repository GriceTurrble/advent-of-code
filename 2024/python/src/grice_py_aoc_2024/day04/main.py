from __future__ import annotations

import time
from collections import defaultdict
from pathlib import Path

FILE = Path(__file__).parent / "inputs.txt"

TARGET = "XMAS"
LEN_WORD = len(TARGET)


def _count_xmas(lines: list[str], x: int, y: int) -> int:
    # x is vertical (lines[x] returns a string row)
    # y is horizontal (lines[x][y] returns a char at coordinates)
    len_col = len(lines)
    len_row = len(lines[x])

    count = 0
    top = x < (LEN_WORD - 1)
    left = y < (LEN_WORD - 1)
    bottom = x > (len_col - LEN_WORD)
    right = y > (len_row - LEN_WORD)

    if not left:
        # horizontal, backwards
        # (capture from length of word on left, up to and including the current index;
        #  then reverse the result with [::-1])
        count += 1 if lines[x][y - LEN_WORD + 1 : y + 1][::-1] == TARGET else 0
    if not right:
        # horizontal, forwards
        count += 1 if lines[x][y : y + LEN_WORD] == TARGET else 0
    if not top:
        # vertical, going up
        section = lines[x - LEN_WORD + 1 : x + 1][::-1]
        count += 1 if [line[y] for line in section] == list(TARGET) else 0
        if not left:
            # diagonal up-left
            word = [
                section[0][y],
                section[1][y - 1],
                section[2][y - 2],
                section[3][y - 3],
            ]
            hit = word == list(TARGET)
            count += 1 if hit else 0
        if not right:
            # diagonal up-right
            word = [
                section[0][y],
                section[1][y + 1],
                section[2][y + 2],
                section[3][y + 3],
            ]
            hit = word == list(TARGET)
            count += 1 if hit else 0
    if not bottom:
        # vertical, going down
        section = lines[x : x + LEN_WORD]
        count += 1 if [line[y] for line in section] == list(TARGET) else 0
        if not left:
            # diagonal down-left
            word = [
                section[0][y],
                section[1][y - 1],
                section[2][y - 2],
                section[3][y - 3],
            ]
            hit = word == list(TARGET)
            count += 1 if hit else 0
        if not right:
            # diagonal down-right
            word = [
                section[0][y],
                section[1][y + 1],
                section[2][y + 2],
                section[3][y + 3],
            ]
            hit = word == list(TARGET)
            count += 1 if hit else 0

    return count


def part1(contents: str) -> int:
    lines = contents.strip().split("\n")
    result = 0
    for x, line in enumerate(lines):
        for y, char in enumerate(line):
            count = 0
            if char == "X":
                count = _count_xmas(lines=lines, x=x, y=y)
            result += count
    return result


TARGET_MS = 2


def _is_cross_mas(lines: list[str], x: int, y: int) -> bool:
    chars: dict[str, int] = defaultdict(int)
    chars[lines[x - 1][y - 1]] += 1
    chars[lines[x - 1][y + 1]] += 1
    chars[lines[x + 1][y - 1]] += 1
    chars[lines[x + 1][y + 1]] += 1
    return (
        chars["M"] == TARGET_MS
        and chars["S"] == TARGET_MS
        and lines[x - 1][y - 1] != lines[x + 1][y + 1]
    )


def part2(contents: str) -> int:
    lines = contents.strip().split("\n")
    result = 0
    for x, line in enumerate(lines[1:-1], start=1):
        for y, char in enumerate(line[1:-1], start=1):
            if char == "A" and _is_cross_mas(lines=lines, x=x, y=y):
                result += 1
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
