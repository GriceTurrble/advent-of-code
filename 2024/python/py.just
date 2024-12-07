# justfile recipes specific to the Python project in AoC 2024

# Show these help docs
help:
    @just --list --unsorted --justfile {{ source_file() }}


# Add new day for Python solutions
new-day DAY:
    uv run cli create-day {{DAY}}
