# Project 2: calculator

Tokenizer -> recursive-descent parser -> AST (`Expr`) -> evaluator. Std only.

## Skills practised
Enums with data, `Box` for recursive types, `Peekable` iterators, pattern matching, `Result` with custom errors, operator precedence/associativity.

## Build it yourself
1. Tokenize `"1 + 2"` into `Vec<Token>`.
2. Evaluate only `+ -` (left to right).
3. Add `* /` with correct precedence (one grammar function per level).
4. Add parentheses, unary minus, and `^` (right-associative).
5. Make every failure a `CalcError` - no `unwrap` in the library.
6. Extensions: variables (`x = 3`), functions (`sqrt(9)`), a `ans` previous-result variable, `%`, print the AST as an S-expression, a bytecode/stack-VM backend.

## Run
```
cargo run
cargo test
```
