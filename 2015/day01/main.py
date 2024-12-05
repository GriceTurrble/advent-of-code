from pathlib import Path
from collections import Counter


def part1(contents: str) -> int:
    c = Counter(contents)
    return c["("] - c[")"]


def part2(contents: str) -> int:
    floor = 0
    for idx, char in enumerate(contents):
        if char == "(":
            floor += 1
        else:
            floor -= 1
        if floor == -1:
            break
    return idx + 1


def main():
    contents = (Path(__file__).parent / "inputs.txt").read_text()
    result1 = part1(contents)
    print(f"Part 1: {result1}")

    result2 = part2(contents)
    print(f"Part 1: {result2}")


main()
