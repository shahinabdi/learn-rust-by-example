// LESSON 22: Advanced generics and trait features
//
// THEORY
// - `where` clauses keep complex bounds readable. `impl Trait` in argument or
//   return position is shorthand for an anonymous generic.
// - Const generics (`[T; N]`) let a type depend on a compile-time VALUE.
// - Blanket impls (`impl<T: Display> MyTrait for T`) give every qualifying type
//   a trait at once. Supertraits (`trait A: B`) require B too.
// - Associated types (`type Output`) vs generic parameters: use an associated
//   type when each implementor has ONE natural choice.
// - `dyn Trait` needs the trait to be "object safe" (no generic methods).
// Docs: https://doc.rust-lang.org/book/ch10-00-generics.html
//       https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

// where clause + multiple bounds
fn summarize<T, U>(a: &[T], b: &[U]) -> String
where
    T: Display,
    U: Debug,
{
    format!("{} items / {:?}", a.len(), b)
}

// Generic container
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    fn push(&mut self, t: T) {
        self.items.push(t);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

// Implementation available only when T meets a bound
impl<T: Display> Stack<T> {
    fn render(&self) -> String {
        self.items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" | ")
    }
}

// Const generic: array wrapper whose size is part of the type
struct Matrix<const R: usize, const C: usize> {
    data: [[i32; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn transpose(&self) -> Matrix<C, R> {
        let mut data = [[0; R]; C];
        for r in 0..R {
            for c in 0..C {
                data[c][r] = self.data[r][c];
            }
        }
        Matrix { data }
    }
}

// Supertrait + blanket impl
trait Named {
    fn name(&self) -> String;
}

trait Greeter: Named {
    fn greet(&self) -> String {
        format!("Hello from {}", self.name())
    }
}

// Every Named type is automatically a Greeter
impl<T: Named> Greeter for T {}

struct Robot;
impl Named for Robot {
    fn name(&self) -> String {
        "R2".into()
    }
}

// Associated type with a custom trait
trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

impl<T> Container for Stack<T> {
    type Item = T;
    fn first(&self) -> Option<&T> {
        self.items.first()
    }
}

fn show_first<C>(c: &C) -> String
where
    C: Container,
    C::Item: Debug,
{
    format!("{:?}", c.first())
}

// Generic function returning a closure
fn counter_by<T>(step: T) -> impl FnMut() -> T
where
    T: Copy + std::ops::AddAssign + Default,
{
    let mut total = T::default();
    move || {
        total += step;
        total
    }
}

// Generic cache with trait-bounded keys
struct Memo<K, V, F>
where
    F: Fn(&K) -> V,
{
    f: F,
    cache: HashMap<K, V>,
}

impl<K: Hash + Eq + Clone, V: Clone, F: Fn(&K) -> V> Memo<K, V, F> {
    fn new(f: F) -> Self {
        Memo { f, cache: HashMap::new() }
    }
    fn get(&mut self, k: K) -> V {
        if let Some(v) = self.cache.get(&k) {
            return v.clone();
        }
        let v = (self.f)(&k);
        self.cache.insert(k, v.clone());
        v
    }
}

fn main() {
    println!("{}", summarize(&[1, 2, 3], &["a", "b"]));

    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    println!("{} peek={:?} first={}", s.render(), s.peek(), show_first(&s));
    println!("pop={:?}", s.pop());

    let m = Matrix::<2, 3> { data: [[1, 2, 3], [4, 5, 6]] };
    let t = m.transpose(); // type is Matrix<3, 2>
    println!("{:?}", t.data);

    println!("{}", Robot.greet());

    let mut c = counter_by(5);
    println!("{} {} {}", c(), c(), c());
    let mut f = counter_by(0.5);
    println!("{} {}", f(), f());

    let mut calls = 0;
    let mut memo = Memo::new(|n: &u64| (1..=*n).product::<u64>());
    for n in [5, 5, 10] {
        let _ = memo.get(n);
        calls += 1;
    }
    println!("{} lookups, {} cached entries, 10! = {}", calls, memo.cache.len(), memo.get(10));
}

// ---------------------------------------------------------------------------
// CHALLENGE 22: Generic toolkit
//  1) `struct Queue<T>` with `enqueue`, `dequeue`, `len`, `is_empty`, backed
//     by two Vecs (or a VecDeque). Add `impl<T: Display> Queue<T>` with `render`.
//  2) `fn largest_by_key<T, K: PartialOrd>(items: &[T], key: impl Fn(&T) -> K)
//     -> Option<&T>`.
//  3) `struct Ring<T, const N: usize>`: a fixed-capacity ring buffer
//     (`push` overwrites the oldest value when full, `iter()` yields oldest
//     to newest).
// (See solutions/22_generics_advanced.rs)
// ---------------------------------------------------------------------------
