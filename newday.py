import argparse
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
STARTING_FILES = [
    'input.txt',
    'main.ipynb',
]

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

    for filename in STARTING_FILES:
        newfile = day_dir / filename
        if not newfile.exists():
            newfile.touch()
            print(f">> Generated {newfile}")


if __name__ == "__main__":
    main()
