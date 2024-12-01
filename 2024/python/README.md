# AoC 2024 Python solutions

## Getting started

This project uses `uv` for all tooling.

To install `uv`, such as through Homebrew:

```shell
brew install uv
```

Run any given day using `uv run dayXX`, where `XX` is the number of the day:

```shell
uv run day01
```

Run tests across the project using `uv run pytest`.

## CLI options

There is a `cli` script included in the project which, among other things (maybe) can scaffold the files for a new day.

Run `uv run cli --help` for details.

To create a new day, use the `create-day` command, followed by the number of the new day:

```shell
uv run cli create-day 5
```

If a directory for the given day already exists, an error is raised.
It will also attempt to add a script entry point to [`pyproject.toml`](pyproject.toml) (in the `[project.scripts]` table) for that new day.
If such a script already exists, an error is raised.
