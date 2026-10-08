// LESSON 15: Conversions, operator overloading and standard traits
//
// THEORY
// - `From<T>`/`Into<T>` are infallible conversions (implement From, get Into
//   free). `TryFrom` is fallible. `FromStr` powers `"..".parse::<T>()`.
// - Operators are traits in `std::ops` (`Add`, `Mul`, `Neg`, `Index`...).
// - `Default`, `Display`, `PartialEq`, `PartialOrd`, `Hash` make your types
//   behave like built-ins. Most can be `#[derive]`d.
// Docs: https://doc.rust-lang.org/std/convert/index.html
//       https://doc.rust-lang.org/std/ops/index.html

use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, AddAssign, Index, Mul, Neg};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2 { x: self.x + o.x, y: self.y + o.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) {
        self.x += o.x;
        self.y += o.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        Vec2 { x: self.x * k, y: self.y * k }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl Index<usize> for Vec2 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index {i} out of range"),
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

// From: tuple -> Vec2 (Into<Vec2> for (f64, f64) comes for free)
impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vec2 { x, y }
    }
}

// FromStr: "3,4".parse::<Vec2>()
impl FromStr for Vec2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').ok_or("expected 'x,y'")?;
        let x = a.trim().parse::<f64>().map_err(|e| e.to_string())?;
        let y = b.trim().parse::<f64>().map_err(|e| e.to_string())?;
        Ok(Vec2 { x, y })
    }
}

// TryFrom: a validated type
#[derive(Debug, PartialEq)]
struct Even(u32);

impl TryFrom<u32> for Even {
    type Error = String;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n % 2 == 0 { Ok(Even(n)) } else { Err(format!("{n} is odd")) }
    }
}

// Accepting anything convertible: `impl Into<String>` avoids forcing callers
// to write `.to_string()`
fn greet(name: impl Into<String>) -> String {
    let name: String = name.into();
    format!("Hello, {name}!")
}

fn main() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b: Vec2 = (3.0, 4.0).into();
    let mut c = a + b * 2.0;
    c += -a;
    println!("{a} {b} {c} x={} y={}", c[0], c[1]);
    println!("default = {:?}", Vec2::default());
    println!("{:?} {:?}", "3, 4".parse::<Vec2>(), "oops".parse::<Vec2>());
    println!("{:?} {:?}", Even::try_from(8), Even::try_from(7));
    println!("{} / {}", greet("Ada"), greet(String::from("Linus")));

    // Built-in numeric conversions
    let small: u8 = 200;
    let wide: u32 = small.into(); // lossless: From
    let narrow = u8::try_from(300u32); // fallible: Err
    println!("{wide} {narrow:?} {}", i64::from(7i32));

    // Sorting floats needs partial_cmp (f64 is not Ord because of NaN)
    let mut v = vec![3.2, 1.5, 2.8];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{v:?}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 15: Money
// Create `struct Money { cents: i64 }` and implement:
//   - `Add`, `Sub` and `Mul<i64>` (quantity)
//   - `Display` as "$12.34"
//   - `FromStr` accepting "12.34" or "$12.34" (Err for junk)
//   - `From<i64>` meaning whole dollars
//   - `Default` = $0.00, and derive PartialEq, PartialOrd, Debug, Clone, Copy
//   - `std::iter::Sum` so `vec_of_money.into_iter().sum::<Money>()` works
// (See solutions/15_conversions_operators.rs)
// ---------------------------------------------------------------------------
