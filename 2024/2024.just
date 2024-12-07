# Root justfile for AoC 2024

mod py 'python/py.just'


# Show these help docs
help:
    @just --list --unsorted --justfile {{ source_file() }}


# Add new day for solutions in all configured languages
new-day DAY:
    @just y2024::py::new-day {{DAY}}
