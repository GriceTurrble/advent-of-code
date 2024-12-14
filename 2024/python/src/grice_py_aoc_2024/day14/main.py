from __future__ import annotations

import time
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"

IntPair = tuple[int, int]


@dataclass
class Robot:
    position: IntPair
    velocity: IntPair

    def simulate(self, seconds: int, grid_size: IntPair) -> IntPair:
        return (
            (self.position[0] + self.velocity[0] * seconds) % grid_size[0],
            (self.position[1] + self.velocity[1] * seconds) % grid_size[1],
        )


def display_grid(positions: list[IntPair], grid_size: IntPair) -> None:
    grid = [["_"] * grid_size[0] for _ in range(grid_size[1])]
    for posx, posy in positions:
        try:
            grid[posy][posx] = "█"
        except IndexError:
            print(f"{posx=}, {posy=}")
            raise
    for line in grid:
        print("".join(line))


GRID_SIZE = (101, 103)


def part1(
    robots: list[Robot],
    grid_size: IntPair = GRID_SIZE,
) -> int:
    quads = [
        0,  # top-left
        0,  # top-right
        0,  # bottom-left
        0,  # bottom-right
    ]
    for robot in robots:
        pos = robot.simulate(seconds=100, grid_size=grid_size)
        if pos[0] < grid_size[0] // 2:
            if pos[1] < grid_size[1] // 2:
                quads[0] += 1
            elif pos[1] > grid_size[1] // 2:
                quads[1] += 1
        elif pos[0] > grid_size[0] // 2:
            if pos[1] < grid_size[1] // 2:
                quads[2] += 1
            elif pos[1] > grid_size[1] // 2:
                quads[3] += 1
    return quads[0] * quads[1] * quads[2] * quads[3]


POTENTIAL_LINE = 15


def part2(
    robots: list[Robot],
    grid_size: IntPair = GRID_SIZE,
) -> int:
    for seconds in range(1, 10_000):
        positions = [x.simulate(seconds=seconds, grid_size=grid_size) for x in robots]
        cx = Counter([x[0] for x in positions])
        cy = Counter([x[1] for x in positions])
        if (
            cx.most_common(2)[1][1] > POTENTIAL_LINE
            and cy.most_common(2)[1][1] > POTENTIAL_LINE
        ):
            display_grid(positions=positions, grid_size=grid_size)
            break
    return seconds


def parse_input(contents: str) -> list[Robot]:
    robots: list[Robot] = []
    for line in contents.strip().split("\n"):
        p, v = line.split(" ")
        robots.append(
            Robot(
                position=tuple(map(int, p[2:].split(","))),  # type:ignore
                velocity=tuple(map(int, v[2:].split(","))),  # type:ignore
            ),
        )
    return robots


def main():
    robots = parse_input(FILE.read_text())

    _start1 = time.perf_counter()
    result1 = part1(robots=robots)
    _delta1 = time.perf_counter() - _start1
    print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(robots=robots)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
