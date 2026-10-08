// LESSON 6: Enums, pattern matching, Option
//
// THEORY
// - An enum is a type that is exactly ONE of several variants, and each
//   variant can carry data. `match` must cover every variant (exhaustive).
// - Rust has no null. `Option<T>` is `Some(value)` or `None`, so absence is
//   encoded in the type and the compiler forces you to handle it.
// Docs: https://doc.rust-lang.org/book/ch06-00-enums.html

#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rect { w: f64, h: f64 }, // named fields
    Triangle(f64, f64, f64), // three sides
    Point,                   // no data
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect { w, h } => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0; // Heron's formula
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
        Shape::Point => 0.0,
    }
}

fn describe(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 => "small",
        4..=9 => "medium",
        x if x < 0 => "negative",
        _ => "large", // catch-all
    }
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn main() {
    let shapes = [
        Shape::Circle(1.0),
        Shape::Rect { w: 3.0, h: 4.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
        Shape::Point,
    ];
    for s in &shapes {
        println!("{s:?} -> area {:.2}", area(s));
    }

    for n in [0, 2, 7, -4, 50] {
        println!("{n}: {}", describe(n));
    }

    // Working with Option
    match divide(10, 2) {
        Some(v) => println!("10/2 = {v}"),
        None => println!("cannot divide"),
    }

    // if let: when you only care about one pattern
    if let Some(v) = divide(9, 3) {
        println!("9/3 = {v}");
    }

    // Handy Option helpers
    println!("{}", divide(1, 0).unwrap_or(-1));
    println!("{:?}", divide(8, 2).map(|v| v * 10));

    // let-else: bail out early
    let Some(q) = divide(6, 3) else { return };
    println!("q = {q}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 6: Traffic light state machine
// Define `enum Light { Red, Green, Yellow }` and implement:
//   - `fn next(&self) -> Light` (Red -> Green -> Yellow -> Red)
//   - `fn seconds(&self) -> u32` (Red 30, Green 25, Yellow 5)
// Bonus: add `Light::Flashing(u8)` carrying a blink count.
// Write your own tests, following the style of earlier lessons.
// ---------------------------------------------------------------------------
