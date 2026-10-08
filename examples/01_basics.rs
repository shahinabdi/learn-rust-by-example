// LESSON 1: Variables, types, shadowing, tuples, arrays
//
// THEORY
// - Variables are immutable by default; add `mut` to change them.
// - Rust is statically typed: every value has a type known at compile time,
//   usually inferred. Integers: i8..i128, u8..u128, isize/usize; floats: f32/f64.
// - Shadowing (`let x = ...` again) creates a NEW variable and may change type.
// Docs: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

fn main() {
    // Immutable vs mutable
    let language = "Rust";
    let mut score = 10;
    score += 5;
    println!("{language} score = {score}");

    // Explicit types and constants (constants are always typed, UPPER_CASE)
    const MAX_LEVEL: u32 = 100;
    let ratio: f64 = 3.0 / 4.0;
    let is_ready: bool = true;
    let letter: char = 'R';
    println!("max={MAX_LEVEL} ratio={ratio} ready={is_ready} letter={letter}");

    // Shadowing: change the type while reusing the name
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    // Integer division vs float division
    println!("7 / 2 = {}, 7.0 / 2.0 = {}, 7 % 2 = {}", 7 / 2, 7.0 / 2.0, 7 % 2);

    // Casting must be explicit
    let big: i32 = 300;
    let small = big as u8; // wraps around: 300 - 256 = 44
    println!("300 as u8 = {small}");

    // Tuples: fixed size, mixed types
    let person: (&str, u32, f64) = ("Ada", 36, 1.75);
    let (name, age, height) = person; // destructuring
    println!("{name} {age} {height} / first field: {}", person.0);

    // Arrays: fixed size, same type, stored on the stack
    let days = ["Mon", "Tue", "Wed"];
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("{:?} {:?} len={}", days, zeros, days.len());
}

// ---------------------------------------------------------------------------
// CHALLENGE 1: Temperature converter
// Write `celsius_to_fahrenheit` (F = C * 9/5 + 32) and `fahrenheit_to_celsius`.
// Then, in main, print a table of 0, 10, 20, ... 100 °C (tip: use an array).
// Run: cargo test --example 01_basics
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!("implement me")
}

#[allow(dead_code, unused_variables)]
fn fahrenheit_to_celsius(f: f64) -> f64 {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_to_f() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn f_to_c() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }
}
