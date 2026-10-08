// LESSON 5: Structs and methods
//
// THEORY
// - A struct groups related data under named fields. `impl` blocks attach
//   functions to it: methods take `self`, `&self` or `&mut self`; functions
//   without `self` are "associated functions" (like constructors: `Type::new`).
// - `#[derive(...)]` auto-implements common traits (Debug, Clone, PartialEq...).
// Docs: https://doc.rust-lang.org/book/ch05-00-structs.html

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // associated function (no self)
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn square(side: f64) -> Self {
        Self::new(side, side)
    }

    // &self: read-only method
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &mut self: modifies in place
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// Tuple struct and unit struct
#[derive(Debug)]
#[allow(dead_code)]
struct Color(u8, u8, u8);
struct Marker;

fn main() {
    let mut r = Rectangle::new(4.0, 3.0);
    println!("{r:?} area={}", r.area());

    r.scale(2.0);
    println!("scaled: {r:#?}"); // pretty debug

    let small = Rectangle::square(2.0);
    println!("r can hold small? {}", r.can_hold(&small));

    // Struct update syntax
    let wide = Rectangle { width: 100.0, ..small.clone() };
    println!("{wide:?} equal to small? {}", wide == small);

    let red = Color(255, 0, 0);
    println!("red = {:?}, r component = {}", red, red.0);
    let _m = Marker;
}

// ---------------------------------------------------------------------------
// CHALLENGE 5: Bank account
// Create a struct `Account { owner: String, balance: i64 }` with:
//   - `Account::new(owner: &str) -> Account` (balance 0)
//   - `deposit(&mut self, amount: i64)`
//   - `withdraw(&mut self, amount: i64) -> bool` (false, and no change,
//     if funds are insufficient)
// Uncomment the test module below and make it pass.
// ---------------------------------------------------------------------------

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn bank() {
//         let mut a = Account::new("Ada");
//         a.deposit(100);
//         assert!(a.withdraw(30));
//         assert!(!a.withdraw(500));
//         assert_eq!(a.balance, 70);
//         assert_eq!(a.owner, "Ada");
//     }
// }
