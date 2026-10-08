# Learn Rust by Example

Each lesson is one runnable file in [examples/](examples): a short **theory** note, a **working program** with many variations, and a **challenge** where you build something similar yourself. Reference answers are in [solutions/](solutions) and bigger capstones in [projects/](projects). Content follows [The Rust Book](https://doc.rust-lang.org/book/).

## Setup

Install Rust from <https://rustup.rs>, then in this folder:

```
cargo run --example 01_basics     # see the lesson run
cargo test --example 01_basics    # check your challenge solution
```

Challenges start as `todo!()` stubs, so their tests fail until you implement them. Lessons 5, 6, 9 and 12 have no stubs: write the code and tests yourself. Lessons 14-24 describe their challenge in comments; write it in a scratch file, then compare with the solution.

**Solutions** (try first!): each is registered as an example named `sol_<lesson>`:

```
cargo test --example sol_03_ownership
cargo run --example sol_03_ownership
```
> Windows note: if you get `link.exe not found` you lack the MSVC build tools. Either install them (Visual Studio Build Tools) or use the GNU toolchain: `rustup toolchain install stable-x86_64-pc-windows-gnu` plus a MinGW gcc, and set `RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu`.

## Path

| # | File | Topic | Key idea |
|---|------|-------|----------|
| 1 | `01_basics` | Variables, types, tuples, arrays | immutable by default, shadowing |
| 2 | `02_control_flow` | Functions, if/loops | everything is an expression |
| 3 | `03_ownership` | Ownership, stack vs heap | move, clone, drop |
| 4 | `04_borrowing` | References, slices | one `&mut` XOR many `&` |
| 5 | `05_structs` | Structs, methods | `impl`, `&self`, derive |
| 6 | `06_enums_match` | Enums, `match`, `Option` | no null, exhaustive matching |
| 7 | `07_collections` | Vec, String, HashMap | `entry` API, UTF-8 |
| 8 | `08_errors` | `Result`, `?`, custom errors | recoverable vs panic |
| 9 | `09_traits_generics` | Traits, generics, `dyn` | static vs dynamic dispatch |
| 10 | `10_lifetimes` | Lifetimes | references can't dangle |
| 11 | `11_closures_iterators` | Closures, iterators | lazy, zero-cost |
| 12 | `12_smart_pointers` | Box, Rc, RefCell | heap, shared, interior mutability |
| 13 | `13_concurrency` | Threads, channels, Mutex | fearless concurrency |
| 14 | `14_modules` | Modules, visibility | `pub`, `use`, encapsulation |
| 15 | `15_conversions_operators` | From/Into, operator traits | `impl Add`, `Display` |
| 16 | `16_custom_iterators` | Implementing `Iterator` | one `next` gives 70 adapters |
| 17 | `17_advanced_patterns` | Patterns, guards, bindings | destructure everything |
| 18 | `18_more_collections` | BTreeMap, HashSet, VecDeque, Cow | pick the right container |
| 19 | `19_io_files` | Files, stdin, paths | `BufRead`, `?` with I/O |
| 20 | `20_testing` | Unit/doc tests, debugging | find the bug via failing tests |
| 21 | `21_design_patterns` | Builder, newtype, typestate, RAII | make illegal states unrepresentable |
| 22 | `22_generics_advanced` | Assoc. types, const generics, PhantomData | zero-cost abstraction |
| 23 | `23_macros` | `macro_rules!` | code that writes code |
| 24 | `24_unsafe_ffi` | `unsafe`, raw pointers, FFI | safe wrappers around unsafe |

## Projects (capstones)

Each is its own Cargo package in [projects/](projects) with a README listing how to build it yourself plus extension ideas. `cd projects/<name>` then `cargo test`.

| Project | After lesson | You practise |
|---|---|---|
| `minigrep` | 8, 14, 19, 20 | lib+bin split, errors, integration tests |
| `calculator` | 6, 9, 12 | tokenizer, recursive-descent parser, `Box` AST |
| `lru_cache` | 9, 18, 22 | generics, trait bounds, data-structure design |
| `todo_cli` | 8, 15, 19 | `serde` JSON, custom errors with `From` |
| `async_chat` | 13 | `async`/`await` with `tokio` (needs internet for first build) |

## Roadmap after this

Rust Book -> Rustlings -> these projects -> [Rust by Example](https://doc.rust-lang.org/rust-by-example/) -> [Rustonomicon](https://doc.rust-lang.org/nomicon/) / [Async Book](https://rust-lang.github.io/async-book/) -> contribute to an open-source crate. Also run `cargo clippy` and `cargo fmt` on everything you write.
## Stack vs heap, in short

| | Stack | Heap |
|---|---|---|
| Size | known at compile time | can grow at runtime |
| Speed | very fast (move a pointer) | slower (allocator) |
| Examples | `i32`, `bool`, `[i32; 3]`, `&T` | `String`, `Vec<T>`, `Box<T>` |
| Cleanup | automatic on scope exit | automatic via owner's `Drop` |

## How to study

1. Read the theory, run the example, then **change it** and break it on purpose (many files have commented-out lines that fail to compile - uncomment them and read the error).
2. Do the challenge without peeking; run its tests.
3. Next steps: [Rust By Example](https://doc.rust-lang.org/rust-by-example/), [Rustlings](https://github.com/rust-lang/rustlings), the [std docs](https://doc.rust-lang.org/std/).
