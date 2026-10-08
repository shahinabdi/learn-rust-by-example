# Project 1: minigrep

A tiny `grep`, based on the Rust Book's I/O project, split into a **library** (`src/lib.rs`) and a thin **binary** (`src/main.rs`).

## Skills practised
Cargo packages (lib + bin), error enums, `Result`/`?`, lifetimes (`Hit<'a>` borrows from the text), iterators, unit tests, integration tests in `tests/`, exit codes.

## Build it yourself
Create a new package (`cargo new minigrep`) and implement, in order:
1. `search(query, contents) -> Vec<&str>` returning matching lines. Write the test first.
2. `Config::build(args)` returning a `Result` with your own error type.
3. `run(&Config)` that reads the file with `fs::read_to_string`.
4. `main.rs`: print errors to **stderr** (`eprintln!`) and exit with a non-zero code.
5. Flags: `-i` (ignore case), `-n` (line numbers).
6. Extensions: `-c` (count only), `-v` (invert match), search a whole directory recursively with `fs::read_dir`, regex support via the `regex` crate, colored output.

Then compare with the reference code in this folder.

## Run
```
cargo run -- Rust ../../README.md -i -n
cargo test
```
