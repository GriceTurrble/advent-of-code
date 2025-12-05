from pathlib import Path


def get_input(day: int) -> str:
    day_filename = f"day{day:0>2}.txt"
    file = Path(__file__).parent / "inputs" / day_filename
    if not file.exists():
        raise ValueError(f"Input file for day #{day} not found")
    return file.read_text()
