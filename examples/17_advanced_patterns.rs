// LESSON 17: Advanced pattern matching
//
// THEORY
// - Patterns are everywhere: `let`, function parameters, `match`, `if let`,
//   `while let`, `for`. They can destructure tuples, structs, enums, slices
//   and references, bind parts of a value, and add `if` guards.
// - `x @ pattern` binds the matched value to a name while testing it.
// - Matches must be exhaustive; `_` ignores a value, `..` ignores the rest.
// Docs: https://doc.rust-lang.org/book/ch18-00-patterns.html

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

fn describe(m: &Message) -> String {
    match m {
        Message::Quit => "quit".into(),
        Message::Move { x: 0, y: 0 } => "stay".into(),
        Message::Move { x, y: 0 } => format!("horizontal {x}"),
        Message::Move { x, y } if x == y => format!("diagonal {x}"),
        Message::Move { .. } => "somewhere".into(),
        Message::Write(s) if s.is_empty() => "empty text".into(),
        Message::Write(s) => format!("text '{s}'"),
        Message::Color(r, _, _) if *r > 200 => "reddish".into(),
        Message::Color(..) => "color".into(),
    }
}

fn age_group(age: u32) -> &'static str {
    match age {
        0 => "newborn",
        n @ 1..=12 if n < 3 => "toddler",
        1..=12 => "child",
        13..=19 => "teen",
        _ => "adult",
    }
}

fn slice_info(v: &[i32]) -> String {
    match v {
        [] => "empty".into(),
        [x] => format!("one: {x}"),
        [first, .., last] if first == last => format!("bookends {first}"),
        [first, rest @ ..] => format!("first {first}, {} more", rest.len()),
    }
}

fn main() {
    for m in [
        Message::Quit,
        Message::Move { x: 0, y: 0 },
        Message::Move { x: 5, y: 0 },
        Message::Move { x: 3, y: 3 },
        Message::Move { x: 1, y: 2 },
        Message::Write(String::new()),
        Message::Write("hi".into()),
        Message::Color(255, 0, 0),
        Message::Color(0, 0, 0),
    ] {
        println!("{m:?} => {}", describe(&m));
    }

    for a in [0, 2, 8, 15, 40] {
        println!("{a}: {}", age_group(a));
    }

    for v in [&[][..], &[7], &[1, 2, 1], &[1, 2, 3, 4]] {
        println!("{v:?} => {}", slice_info(v));
    }

    // Destructuring nested data
    let ((a, b), Point { x, y: py }) = ((1, 2), Point { x: 10, y: 20 });
    println!("{a} {b} {x} {py}");

    // while let: pop until empty
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // let-else and if-let chains of Options
    let input = "42";
    let Ok(n) = input.parse::<i32>() else {
        println!("not a number");
        return;
    };
    println!("parsed {n}");

    // matches! macro: a pattern test returning bool
    let m = Message::Write("x".into());
    println!("is write? {}", matches!(m, Message::Write(_)));
    println!("is small? {}", matches!(n, 1..=50 if n % 2 == 0));

    // Matching references and `ref`/`ref mut`
    let mut pair = (String::from("a"), 5);
    let (ref mut s, _) = pair;
    s.push('!');
    println!("{pair:?}");

    // Option combinations
    match (Some(1), None::<i32>) {
        (Some(a), Some(b)) => println!("both {a} {b}"),
        (Some(a), None) | (None, Some(a)) => println!("only {a}"),
        (None, None) => println!("neither"),
    }
}

// ---------------------------------------------------------------------------
// CHALLENGE 17: RPN calculator and classifiers
//  1) `eval_rpn(tokens: &[&str]) -> Result<i64, String>`: evaluate Reverse
//     Polish Notation ("3 4 + 2 *" => 14). Use a Vec as a stack and match on
//     the token with patterns ("+" | "-" | "*" | "/" => ..., number => ...).
//     Errors: stack underflow, bad token, division by zero, leftover values.
//  2) `classify(&[i32]) -> &'static str` returning "empty", "single",
//     "pair", "sorted" (len > 2, ascending) or "other" using slice patterns.
// (See solutions/17_advanced_patterns.rs)
// ---------------------------------------------------------------------------
