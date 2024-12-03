from pathlib import Path

import click
import toml

DIR = Path(__file__).parent
MAIN_PY_TEMPLATE = DIR / "templates/main.py-tpl"
TEST_PY_TEMPLATE = DIR / "templates/test_day.py-tpl"


@click.group()
def cli():
    pass


def _add_day_to_pyproject_toml(day: int) -> None:
    pyproject_toml = DIR / "../../pyproject.toml"
    if not pyproject_toml.exists():
        raise click.ClickException(f"pyproject.toml not found at {pyproject_toml}")

    content = toml.loads(pyproject_toml.read_text())
    day_key = f"day{day:0>2}"
    scripts = content["project"]["scripts"]
    if day_key in scripts:
        raise click.ClickException(f"Day {day} script already exists in pyproject.toml")
    scripts[day_key] = f"grice_py_aoc_2024.{day_key}.main:main"

    pyproject_toml.write_text(toml.dumps(content))


def _create_day_files(day: int) -> None:
    day_path = DIR / f"day{day:0>2}"
    if day_path.exists():
        raise click.ClickException(f"Day {day} already exists at {day_path}")

    day_path.mkdir()
    (day_path / "__init__.py").touch()
    (day_path / "inputs.txt").touch()
    (day_path / "test_inputs_p1.txt").touch()
    (day_path / "test_inputs_p2.txt").touch()
    _create_templates(day=day, path=day_path)


def _create_templates(day: int, path: Path) -> None:
    # Create the file for the main code from template
    main_py = path / "main.py"
    main_py.touch()
    main_py.write_text(MAIN_PY_TEMPLATE.read_text())

    # Create the file for the test code from template
    test_py = path / f"test_day{day:0>2}.py"
    test_py.touch()
    test_py.write_text(TEST_PY_TEMPLATE.read_text())


@cli.command()
@click.argument("day", type=int)
def create_day(day: int) -> None:
    """Scaffold the files and pyproject.toml script for DAY."""
    day_path = DIR / f"day{day:0>2}"
    if day_path.exists():
        raise click.ClickException(f"Day {day} already exists at {day_path}")
    _create_day_files(day=day)
    _add_day_to_pyproject_toml(day=day)

    print(f">> Day {day} generated at:\n    {day_path}/main.py")
    print(f">> You can run the day's code using `uv run day{day:0>2}`")
    print("   and run tests for this and any other day using `uv run pytest`")


if __name__ == "__main__":
    cli()
