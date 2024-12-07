from __future__ import annotations

import time
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"


def _split_line(line: str) -> tuple[int, list[int]]:
    val, nums = line.split(":")
    return (
        int(val),
        list(map(int, nums.split())),
    )


def _is_line_valid(nums: list[int], val: int, part2: bool = False) -> bool:
    # Note, we cannot use `.pop()`, as that modifies the list in place.
    # We would quickly exhaust the list and be unable to test in with multiple branching
    # recursion levels.
    last_num = nums[-1]
    if len(nums) == 1:
        # There is only one num in this line, so check that it matches our target
        return last_num == val

    left_remaining = nums[:-1]
    # We recurse from right to left, checking how `val` at this recursion level
    # interacts with the last number of `nums`.

    # First, the '*' operator.
    # If the `val` is divisible by the last num (`val % last_num == 0`),
    # we can see if the remaining left part of the line is valid for the new target
    # `val / last_num`. If it does, then the line is valid; otherwise, we can test
    # other operators

    # EXAMPLE: val=3267, nums=[81, 40, 27]
    #   3267 // 27 = 121
    #   RECURSE: Is the line `[81, 40]` valid for the value `121`?
    if val % last_num == 0 and _is_line_valid(
        nums=left_remaining,
        val=val // last_num,
        part2=part2,
    ):
        return True

    # Next, '+' operator.
    # Similar story as above: if `val - last_num` remains in the positive,
    # then we can test to see if the remaining left part is valid for the new target
    # `val - last_num`.

    # EXAMPLE: val=121, nums=[81, 40]
    #   121 - 40 = 81
    #   RECURSE: Is the line `[81]` valid for the value `81`?
    #   (obviously so, but it does need to recurse once to get our base case)
    if val - last_num > 0 and _is_line_valid(
        nums=left_remaining,
        val=val - last_num,
        part2=part2,
    ):
        return True

    if part2:
        # Finally, for part 2 of the challenge, the '||' operator for concatenation:
        str_val, str_last = str(val), str(last_num)
        if (
            # Check that our value ends with the digits of the last_num
            str_val.endswith(str_last)
            # If it does, capture the string version of the new target first.
            # There are cases where `new_val` results in an empty string,
            # so make this part of the `if` conditions to exit early in that case.
            and (new_val := str_val[: -len(str_last)])
            # Finally, the line may be valid if the remaining nums are valid
            # for the `int` version of the `new_val`.
            and _is_line_valid(
                nums=left_remaining,
                val=int(new_val),
                part2=part2,
            )
        ):
            return True

    # If none of the above conditions work at this stage, then the line is invalid.
    return False


def part1(contents: str) -> int:
    total = 0
    for line in contents.strip().split("\n"):
        val, nums = _split_line(line)
        if _is_line_valid(nums=nums, val=val):
            total += val
    return total


def part2(contents: str) -> int:
    total = 0
    for line in contents.strip().split("\n"):
        val, nums = _split_line(line)
        if _is_line_valid(nums=nums, val=val, part2=True):
            total += val
    return total


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
