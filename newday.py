import argparse
import json
from copy import deepcopy
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent

MAIN_TEMPLATE = {
    "cells": [
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [],
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "outputs": [],
            "source": [
                "from pathlib import Path\n",
                "\n",
                "INPUTS = Path('input.txt').read_text().strip().split('\\n')",
            ],
        },
    ],
    "metadata": {
        "language_info": {
            "name": "python",
        },
        "orig_nbformat": 4,
    },
    "nbformat": 4,
    "nbformat_minor": 2,
}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "year",
        type=int,
        choices=[2020, 2021],
    )
    parser.add_argument(
        "day",
        type=int,
        choices=list(range(1, 26)),
    )

    options = parser.parse_args()
    day_dir = BASE_DIR / str(options.year) / f"day{options.day:0>2}"
    day_dir.mkdir(exist_ok=True)

    input_file = day_dir / "input.txt"
    if not input_file.exists():
        input_file.touch()
        print(">> Generated input file")

    main_file = day_dir / "main.ipynb"
    if not main_file.exists():
        content = deepcopy(MAIN_TEMPLATE)
        content["cells"][0]["source"] = [
            f"# Day {options.day} - \n",
            "\n",
            f"https://adventofcode.com/{options.year}/day/{options.day}",
        ]
        main_file.write_text(json.dumps(content))
        print(">> Generated main file")


if __name__ == "__main__":
    main()
