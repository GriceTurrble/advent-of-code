from __future__ import annotations

import time
from dataclasses import dataclass, field
from pathlib import Path

DIR = Path(__file__).parent
# FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"
FILE = DIR.parents[3] / "inputs/tests" / f"test_{DIR.stem}_ex1.txt"


IntPair = tuple[int, int]


DIRECTION_MAP = {
    "<": (0, -1),
    ">": (0, 1),
    "v": (1, 0),
    "^": (-1, 0),
}


@dataclass
class Warehouse:
    directions: str
    robot: IntPair = field(default_factory=lambda: (0, 0))
    boxes: set[IntPair] = field(default_factory=set)
    walls: set[IntPair] = field(default_factory=set)
    part2: bool = False

    ROBOTS = ["@", "@."]
    WALLS = ["#", "##"]
    BOXES = ["O", "[]"]
    EMPTIES = [".", ".."]

    @classmethod
    def from_input(cls, contents: str, part2: bool = False):
        """Build the warehouse from the `contents` input.

        If `part2` is True, calculate all positions as double-wide
        """
        grid, instructions = contents.strip().split("\n\n")
        obj = cls(
            directions=instructions.replace("\n", ""),
            part2=part2,
        )
        # In part 2, positions are double wide
        # In our list-of-lists arrangement, y is the row.
        # Thus, multiply the y coordinates by this value:
        y_mult = 2 if part2 else 1

        for x, line in enumerate(grid.split("\n")):
            for y, char in enumerate(line):
                # We'll determine the position of an object to be its left-most point.
                # In part 2 calculations, we just need to consider that
                # location `(x, y)` AND `(x, y+1)` as being the same object.
                pos = (x, y * y_mult)
                if char == cls.ROBOTS[0]:
                    obj.robot = pos
                elif char == cls.WALLS[0]:
                    obj.walls.add(pos)
                elif char == cls.BOXES[0]:
                    obj.boxes.add(pos)
        return obj

    def move_robot(self, direction: str) -> None:
        if self.part2:
            self.move_robot_p2(direction=direction)
        self.move_robot_p1(direction=direction)

    def move_robot_p1(self, direction: str) -> None:
        """Moves the robot one space in `direction`, moving boxes accordingly.

        If no empty space is found between the robot and the next wall, no moves are
        taken. Otherwise, if a box is adjacent to the robot in this direction, that box
        will be moved into first empty space found. This assumes that the first empty
        space will be somewhere on the other side of a line of boxes.
        """
        if direction not in DIRECTION_MAP:
            raise ValueError(f"Unexpected direction '{direction}', aborted")
        delta = DIRECTION_MAP[direction]
        adjacent_box: IntPair | None = None
        next_space = (self.robot[0] + delta[0], self.robot[1] + delta[1])
        if next_space in self.boxes:
            adjacent_box = next_space
        while True:
            if next_space in self.walls:
                # Hit a wall without finding an empty space.
                return
            if next_space not in self.boxes:
                # This is an empty space!
                # 1. The adjacent box, if any, is removed from the set of boxes
                # 2. This space is added to boxes as the new position of the box,
                #    at the end of the line of boxes.
                # 3. The robot is moved one space in `direction`, completing the move.
                if adjacent_box is not None:
                    # There is an adjacent box to be moved
                    self.boxes.remove(adjacent_box)
                    self.boxes.add(next_space)
                self.robot = (self.robot[0] + delta[0], self.robot[1] + delta[1])
                return
            next_space = (next_space[0] + delta[0], next_space[1] + delta[1])

    def move_robot_p2(self, direction: str) -> None:
        """Move the robot one space in `direction`, per part 2 instructions."""
        if not self.part2:
            raise ValueError("Not configured for part2; positions may be incorrect!")
        if direction not in DIRECTION_MAP:
            raise ValueError(f"Unexpected direction '{direction}', aborted")
        # TODO

    def sum_box_coords(self) -> int:
        total = 0
        for box in self.boxes:
            total += (100 * box[0]) + box[1]
        return total

    def display_grid(self) -> None:
        out_char = 1 if self.part2 else 0
        dimensions = max(self.walls)
        output = []
        for x in range(dimensions[0] + 1):
            line = ""
            # Objects on the grid are double-wide,
            # so we need to step by 2 in part 2 to avoid printing empties
            # in every other position.
            for y in range(0, dimensions[1] + 1, 2 * out_char or 1):
                curr = (x, y)
                if curr in self.boxes:
                    line += self.BOXES[out_char]
                elif curr in self.walls:
                    line += self.WALLS[out_char]
                elif curr == self.robot:
                    line += self.ROBOTS[out_char]
                else:
                    line += self.EMPTIES[out_char]
            output.append(line)
        print("\n".join(output))


def part1(contents: str) -> int:
    warehouse = Warehouse.from_input(
        contents=contents,
    )
    for direction in warehouse.directions:
        warehouse.move_robot_p1(direction=direction)
    total = warehouse.sum_box_coords()
    return total


def part2(contents: str) -> int:
    warehouse = Warehouse.from_input(
        contents=contents,
        part2=True,
    )
    total = 0
    warehouse.display_grid()
    return total


def main():
    contents = FILE.read_text()

    # _start1 = time.perf_counter()
    # result1 = part1(contents=contents)
    # _delta1 = time.perf_counter() - _start1
    # print(f">> Part 1: {result1} ({_delta1:.6f}s)")

    _start2 = time.perf_counter()
    result2 = part2(contents=contents)
    _delta2 = time.perf_counter() - _start2
    print(f">> Part 2: {result2} ({_delta2:.6f}s)")


if __name__ == "__main__":
    main()
