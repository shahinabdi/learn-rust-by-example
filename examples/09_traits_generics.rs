// LESSON 9: Traits and generics
//
// THEORY
// - A trait defines shared behavior (like an interface). Types implement it.
// - Generics (`fn f<T: Trait>`) are resolved at compile time via
//   monomorphization: zero runtime cost ("static dispatch").
// - Trait objects (`Box<dyn Trait>`) choose the method at runtime
//   ("dynamic dispatch") and allow mixed types in one collection.
// Docs: https://doc.rust-lang.org/book/ch10-00-generics.html

use std::fmt::Display;

trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    // default method
    fn speak(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}

struct Dog;
struct Cat {
    lives: u8,
}

impl Animal for Dog {
    fn name(&self) -> String {
        "Dog".into()
    }
    fn sound(&self) -> String {
        "woof".into()
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        format!("Cat({} lives)", self.lives)
    }
    fn sound(&self) -> String {
        "meow".into()
    }
    fn speak(&self) -> String {
        "The cat ignores you.".into() // overrides the default
    }
}

// Static dispatch: generic with a trait bound
fn introduce<T: Animal>(a: &T) {
    println!("{}", a.speak());
}

// Dynamic dispatch: a heterogeneous collection
fn chorus(animals: &[Box<dyn Animal>]) {
    for a in animals {
        println!("{}", a.speak());
    }
}

// Generic function with multiple bounds
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0];
    for &i in items {
        if i > max {
            max = i;
        }
    }
    max
}

// Generic struct
struct Pair<T> {
    a: T,
    b: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn show_largest(&self) {
        if self.a >= self.b {
            println!("largest = {}", self.a);
        } else {
            println!("largest = {}", self.b);
        }
    }
}

fn main() {
    introduce(&Dog);
    introduce(&Cat { lives: 9 });

    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat { lives: 7 })];
    chorus(&zoo);

    println!("{} {} {}", largest(&[3, 8, 1]), largest(&[1.5, 0.2]), largest(&['a', 'z', 'q']));
    Pair { a: "apple", b: "pear" }.show_largest();
}

// ---------------------------------------------------------------------------
// CHALLENGE 9: Shapes with traits
// Define `trait Shape { fn area(&self) -> f64; fn name(&self) -> String; }`
// Implement it for `Circle { r: f64 }` and `Square { side: f64 }`.
// Then write `fn total_area(shapes: &[Box<dyn Shape>]) -> f64` and
// `fn biggest<'a>(shapes: &'a [Box<dyn Shape>]) -> Option<&'a Box<dyn Shape>>`.
// Bonus: implement `std::fmt::Display` for both shapes.
// ---------------------------------------------------------------------------
