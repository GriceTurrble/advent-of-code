from __future__ import annotations

import time
from dataclasses import dataclass
from pathlib import Path
from typing import Protocol

DIR = Path(__file__).parent
FILE = DIR.parents[3] / "inputs" / f"{DIR.stem}.txt"


class OpCallable(Protocol):
    def __call__(self, operand: int) -> int | str | None: ...


@dataclass
class Program:
    program: list[int]
    reg_a: int = 0
    reg_b: int = 0
    reg_c: int = 0
    max_len_reg: int = 0

    @classmethod
    def from_inputs(cls, contents: str):
        registers_str, program_str = contents.strip().split("\n\n")

        # Parse the initial values of the registers
        reg_a, reg_b, reg_c = (
            int(x.split(": ")[1]) for x in registers_str.strip().split("\n")
        )
        max_len_reg = max(
            len(str(reg_a)),
            len(str(reg_b)),
            len(str(reg_c)),
        )

        return cls(
            reg_a=reg_a,
            reg_b=reg_b,
            reg_c=reg_c,
            program=list(map(int, program_str.strip().split(": ")[1].split(","))),
            max_len_reg=max_len_reg,
        )

    def combo_val(self, combo: int) -> int:
        """Calculates the combo value of an operand.

        0 - 3: literal value, return unchanged
        4: return A (register 0)
        5: return B (register 1)
        6: return C (register 2)

        All other values should not exist, and will throw an error.
        """
        if 0 <= combo <= 3:  # noqa: PLR2004
            return combo
        if combo > 6:  # noqa: PLR2004
            raise ValueError("what?")
        return {
            0: self.reg_a,
            1: self.reg_b,
            2: self.reg_c,
        }[combo - 4]

    def _a_reg_divided(self, operand: int) -> int:
        """Calculates register A (0) divided by 2 ** combo value.

        Used by 'adv', 'bdv', and 'cdv' ops to determine where this value is stored.
        """
        return self.reg_a // (2 ** self.combo_val(combo=operand))

    def adv(self, operand: int) -> None:
        """`0`: Division op of A (0) and combo op `operand`'s value.

        Result is stored back in A (0).
        """
        self.reg_a = self._a_reg_divided(operand=operand)

    def bxl(self, operand: int) -> None:
        """`1`: Bitwise XOR of B (1) and the literal `operand` value.

        Result is stored in B (1).
        """
        self.reg_b ^= operand

    def bst(self, operand: int) -> None:
        """`2`: Takes combo op `operand` modulo 8

        Result is stored in B (1).
        """
        self.reg_b = self.combo_val(combo=operand) % 8

    def jnz(self, operand: int) -> int | None:
        """`3`: Jump op.

        If A (0) is 0, returns None (no jump).
        Otherwise, returns `operand` unchanged. This should be used to indicate
        the index where the instructions should start from.

        NOTE: this seems to indicate that a loop will be present in all programs?
        Figuring out how to reduce this loop might be key to performance here.
        """
        if self.reg_a == 0:
            return None
        return operand

    def bxc(self, *args, **kwargs) -> None:
        """`4`: Bitwise XOR of B (1) and C (2) (ignores args).

        Result is stored in C (2).
        """
        self.reg_b ^= self.reg_c

    def out(self, operand: int) -> str:
        """`5`: returns combo op `operand` modulo 8 as a string."""
        return str(self.combo_val(combo=operand) % 8)

    def bdv(self, operand: int) -> None:
        """`6`: Similar to 'adv', with result stored in B (1)."""
        self.reg_b = self._a_reg_divided(operand=operand)

    def cdv(self, operand: int) -> None:
        """`7`: Similar to 'adv', with result stored in C (2)."""
        self.reg_c = self._a_reg_divided(operand=operand)

    def run(self) -> str:
        mapping: dict[int, OpCallable] = {
            0: self.adv,
            1: self.bxl,
            2: self.bst,
            3: self.jnz,
            4: self.bxc,
            5: self.out,
            6: self.bdv,
            7: self.cdv,
        }
        results: list[str] = []
        pointer = 0

        while True:
            opcode, operand = self.program[pointer], self.program[pointer + 1]
            operation = mapping[opcode]
            result = operation(operand=operand)
            print(
                (
                    f"{opcode},{operand} "
                    f"-> OP {operation.__name__} "
                    f"| A: {self.reg_a:>{self.max_len_reg}} "
                    f"| B: {self.reg_b:>{self.max_len_reg}} "
                    f"| C: {self.reg_c:>{self.max_len_reg}} "
                ),
                end="",
            )
            if isinstance(result, int):
                # Jump code
                pointer = result
                print(f">> JUMPED to {pointer}")
                continue
            if isinstance(result, str):
                print(f">> OUT {result}", end="")
                results.append(result)
            print()
            pointer += 2
            if pointer >= len(self.program):
                break

        return ",".join(results)


def part1(contents: str) -> str:
    program = Program.from_inputs(contents)
    return program.run()


def part2(contents: str) -> str:
    program = Program.from_inputs(contents)
    return program.run()


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
