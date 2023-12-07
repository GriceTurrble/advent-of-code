# AoC 2023

Doing it in **Rust** this year!

## Cargo workspace

Docs: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

To add a new day to the workspace:

1. Open [Cargo.toml](Cargo.toml) here in the folder root.
2. Uncomment the target day.
3. Navigate to *this* directory in terminal, then run `cookiecutter` with the included `template/`:

   ```shell
   cookiecutter template
   ```

   You will be asked for the `day`, just enter the number of that day (it will fail, correctly so, if you enter a day that already exists).

4. Copy the day's samples and input to `sample.txt` and `input.txt` in the new directory, respectively.
5. (Optional) add notes to the generated `README.md` for that day.

The day's code and other files should be fully scaffolded and ready to use. Add stuff to `main.rs`, `part1.rs`, and/or `part2.rs` as needed.

## Usage

To run the code, head to *this* directory (containing this README) and use `cargo run`. Specify the day with `-p` and a path to an input file:

```shell
# sample data
cargo run -p dayXX -- dayXX/sample.txt
# inputs
cargo run -p dayXX -- dayXX/input.txt
```

There are also `make` targets to make (sic) this easier:

```shell
# run using sample data for day 7
make run_sample_07

# run using inputs for day 16
make run_16
```
