# AoC 2023

Doing it in **Rust** this year!

## Cargo workspace

Docs: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

To add a new day to the workspace:

1. Open [Cargo.toml](Cargo.toml) here in the folder root.
2. Uncomment the target day.
3. Run `cargo new <day>` in the console, with a name matching the given day:

   ```bash
   # for day02
   cargo new day02
   ```

4. Add the local `lube` dependency:

   ```toml
   [dependencies]
   lube = { path = "../lube" }
   ```

5. If you need other deps tagged in the workspace (such as `regex`), add it with `workspace = true`:

   ```toml
   [dependencies]
   # ...
   regex = { workspace = true }
   ```

   You should default to adding the crate as a dep in the overall workspace, to keep things tidy. If there's any confusion at the individual binary level, break this rule and add it as a dep in the day, instead.

## Getting started on a new day

Start from this sample:

```rust
/*!
## Advent of Code 2023 Day X

Link: <https://adventofcode.com/2023/day/X>
*/
use lube::{get_file_contents, get_input_file_path, ...};

/// Part 1 solution
fn part_one(contents: Vec<String>) {
    println!("Hello...");
}

/// Part 2 solution
fn part_two(contents: Vec<String>) {
    println!("...world!");
}

fn main() {
    let inp_file_path: std::path::PathBuf = get_input_file_path();
    let contents: Vec<String> = get_file_contents(inp_file_path);

    println!("PART 1:");
    part_one(contents.clone());
    println!("PART 2:");
    part_two(contents.clone());
    println!("DONE");
```

Then you can hack away one part at a time. Ideally move most of the work out to functions, so potentially a working solution for part 1 can be reused and customized for part 2 with less effort.
