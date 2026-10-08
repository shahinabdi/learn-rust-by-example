# Project 4: todo_cli

A to-do manager that saves to `todos.json`.

## Skills practised
External crates (`serde`, `serde_json`) and `#[derive(Serialize, Deserialize)]`, custom error type with `From` impls so `?` just works, `fs` I/O, `iter_mut`/`find`/`position`.

## Build it yourself
1. `cargo new todo_cli`, then `cargo add serde --features derive` and `cargo add serde_json`.
2. Define `Task { id, title, done }` and `TodoList`.
3. Implement `add`, `complete`, `remove`, `load`, `save`. A missing file must mean "empty list", not an error.
4. Write the CLI: `add <title>`, `done <id>`, `rm <id>`, `list`.
5. Extensions: use the `clap` crate for argument parsing, add due dates with `chrono`, priorities (an enum), `list --pending`, a `clear-done` command, atomic saves (write to temp file then rename).

## Run
```
cargo run -- add Learn lifetimes
cargo run -- list
cargo test
```
