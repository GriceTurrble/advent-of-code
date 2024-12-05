from __future__ import annotations

import time
from collections import defaultdict, deque
from functools import cmp_to_key, partial
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"


def _is_line_correct(
    line_nums: list[int],
    rules_dict: dict[int, set[int]],
) -> bool:
    line_q = deque(line_nums)
    left_q: set[int] = set()

    while line_q:
        val = line_q.popleft()
        if left_q & rules_dict[val]:
            return False
        if set(line_q) - rules_dict[val]:
            return False
        left_q.add(val)
    return True


def _sort_cmp(
    a: int,
    b: int,
    rules_dict: dict[int, set[int]],
) -> int:
    if b in rules_dict[a]:
        return -1
    return 1


def _get_result(contents: str, part2: bool = False) -> int:
    rules, lines = contents.strip().split("\n\n")
    rules_dict: dict[int, set[int]] = defaultdict(set)
    for rule in rules.strip().split("\n"):
        left, right = rule.split("|")
        rules_dict[int(left)].add(int(right))

    sorter = partial(_sort_cmp, rules_dict=rules_dict)

    result = 0
    for line in lines.strip().split("\n"):
        line_nums = list(map(int, line.split(",")))
        if _is_line_correct(line_nums=line_nums, rules_dict=rules_dict):
            result += line_nums[len(line_nums) // 2] if not part2 else 0
        elif part2:
            new_line = sorted(line_nums, key=cmp_to_key(sorter))
            result += new_line[len(new_line) // 2]

    return result


def part1(contents: str) -> int:
    return _get_result(contents=contents)


def part2(contents: str):
    return _get_result(contents=contents, part2=True)


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
