# Root justfile for all AoC stuff

mod y2024 '2024/2024.just'

CURRENT_YEAR := "2024"

# Aliases for ANSI color sequences. Makes our output easier to read!
COLOR_GREEN := "\\033[0;32m"
COLOR_STOP := "\\033[0m"

# The result should be `\\[ \\]`, but we need to escape those slashes again here to make it work:
GREP_TARGET := "\\\\[gone\\\\]"


# Show these help docs
help:
    @just --list --unsorted --justfile {{ source_file() }}


# Switches to `main` branch, then prunes local branches deleted from remote.
prune_dead_branches:
    @echo "{{ COLOR_GREEN }}>> 'Removing dead branches...{{ COLOR_STOP }}"
    @git switch main
    @git fetch --prune
    @git branch -v | grep "{{ GREP_TARGET }}" | awk '{print $1}' | xargs -I{} git branch -D {}


# Add new day for solutions in the current year
new-day DAY:
    @just y{{CURRENT_YEAR}}::new-day {{DAY}}
