// LESSON 23: Declarative macros (macro_rules!)
//
// THEORY
// - A macro writes code at compile time. `macro_rules!` matches the tokens you
//   pass against patterns (like `match`) and expands to Rust code. Unlike
//   functions, macros can take a variable number of arguments and generate
//   items (structs, impls, tests).
// - Fragment kinds: `expr`, `ident`, `ty`, `tt` (any token tree), `literal`,
//   `pat`, `stmt`. Repetition: `$( ... ),*` (zero or more) / `+` (one or more).
// - Prefer functions and generics; reach for macros only when they can't do it.
// Docs: https://doc.rust-lang.org/book/ch19-06-macros.html
//       https://doc.rust-lang.org/reference/macros-by-example.html

use std::collections::HashMap;

// Variable number of arguments, built with repetition
macro_rules! hashmap {
    ($( $k:expr => $v:expr ),* $(,)?) => {{
        let mut m = HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

// Recursive macro: maximum of any number of expressions
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = max!($($rest),+);
        if a > b { a } else { b }
    }};
}

// Generating items: a struct plus a constructor
macro_rules! make_struct {
    ($name:ident { $($field:ident : $ty:ty),* }) => {
        #[derive(Debug, Default)]
        #[allow(dead_code)]
        struct $name {
            $( $field: $ty ),*
        }
        impl $name {
            fn field_names() -> Vec<&'static str> {
                vec![ $( stringify!($field) ),* ]
            }
        }
    };
}

make_struct!(Config { host: String, port: u16, verbose: bool });

// Generating tests / repetitive impls
macro_rules! impl_double {
    ($($t:ty),*) => {
        $( impl Double for $t { fn double(&self) -> Self { *self * 2 as $t } } )*
    };
}

trait Double {
    fn double(&self) -> Self;
}
impl_double!(i32, u64, f64);

// A tiny assertion helper that shows the expression text
macro_rules! check {
    ($cond:expr) => {
        println!("{} => {}", stringify!($cond), $cond)
    };
}

fn main() {
    let m = hashmap! { "a" => 1, "b" => 2, };
    let mut keys: Vec<_> = m.iter().collect();
    keys.sort();
    println!("{keys:?}");

    println!("{}", max!(3, 9, 4));
    println!("{}", max!(1.5, 0.2));

    let c = Config::default();
    println!("{c:?} fields={:?}", Config::field_names());

    println!("{} {} {}", 4i32.double(), 4u64.double(), 1.5f64.double());
    check!(1 + 1 == 2);
    check!("rust".len() > 10);

    // Built-in macros worth knowing
    println!("{}", format!("{:>6.2}|{:<5}|{:^7}|{:08.3}|{:#x}|{:#b}", 3.14159, "ab", "mid", 2.5, 255, 5));
    let v = vec![1; 3];
    assert_eq!(v, [1, 1, 1]);
    debug_assert!(v.len() == 3);
    println!("{} line {}", file!(), line!());
    // todo!(), unimplemented!(), unreachable!(), dbg!(x), matches!(..), write!, vec!
    let x = dbg!(2 * 3); // prints to stderr with file:line
    println!("{x}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 23: Write your own macros
//  1) `min!(a, b, c, ...)` mirroring `max!`.
//  2) `square_all!(a, b, c)` expanding to the array `[a*a, b*b, c*c]`.
//  3) `enum_str!(enum Color { Red, Green, Blue })`: generates the enum AND an
//     `as_str(&self) -> &'static str` method (hint: `$( $variant:ident ),*`,
//     `stringify!($variant)`, and `match self { $( Self::$variant => ... ),* }`).
//  Bonus: read about procedural macros (`#[derive(...)]`) in the Book.
// (See solutions/23_macros.rs)
// ---------------------------------------------------------------------------
