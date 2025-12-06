from pathlib import Path


def get_input(day: int) -> str:
    day_filename = f"day{day:0>2}.txt"
    file = Path(__file__).parent / "inputs" / day_filename
    if not file.exists():
        raise ValueError(f"Input file for day #{day} not found")
    return file.read_text().strip()


def run_part(
    func,
    test_inputs,
    test_expected,
    real_inputs=None,
):
    print("--- TEST ---")
    print(">> EXPECTED:", test_expected)
    test_result = func(test_inputs)

    mark = "✅" if test_result == test_expected else "❌"
    extra = ""
    if test_result != test_expected:
        extra = "(too high!)" if test_result > test_expected else "(too low!)"

    print(">> RESULT:  ", test_result, mark, extra)
    if real_inputs is not None:
        print()
        print("--- REAL ---")
        print(">> RESULT:  ", func(real_inputs))
