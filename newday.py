import json
from copy import deepcopy
from pathlib import Path

import click

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


@click.command()
@click.argument("year", type=click.IntRange(2020, 2022))
@click.argument("day", type=click.IntRange(1, 26))
def main(year, day):
    day_dir = BASE_DIR / str(year) / f"day{day:0>2}"
    day_dir.mkdir(exist_ok=True)

    input_file = day_dir / "input.txt"
    if not input_file.exists():
        input_file.touch()
        print(">> Generated input file")

    main_file = day_dir / "main.ipynb"
    if not main_file.exists():
        content = deepcopy(MAIN_TEMPLATE)
        content["cells"][0]["source"] = [
            f"# Day {day} - \n",
            "\n",
            f"https://adventofcode.com/{year}/day/{day}",
        ]
        main_file.write_text(json.dumps(content))
        print(">> Generated main file")


if __name__ == "__main__":
    main()
