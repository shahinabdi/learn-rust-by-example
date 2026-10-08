// LESSON 16: Writing your own iterators
//
// THEORY
// - The `Iterator` trait needs just one method: `next(&mut self) -> Option<Item>`.
//   `type Item` is an ASSOCIATED TYPE: each implementor picks exactly one.
// - Implement it and you get ~70 adapters (`map`, `filter`, `zip`, `sum`...) free.
// - `IntoIterator` is what `for` loops call; implement it for `T`, `&T`, `&mut T`
//   to make your collection loop-friendly.
// Docs: https://doc.rust-lang.org/std/iter/index.html

struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

// Stateful, infinite iterator
struct Fib {
    a: u64,
    b: u64,
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let out = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(out)
    }
}

// A collection that can be iterated by reference
struct Playlist {
    songs: Vec<String>,
}

impl<'a> IntoIterator for &'a Playlist {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.songs.iter()
    }
}

impl IntoIterator for Playlist {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.songs.into_iter()
    }
}

// Returning an iterator without naming its type
fn evens_up_to(n: u32) -> impl Iterator<Item = u32> {
    (0..=n).filter(|x| x % 2 == 0)
}

fn main() {
    println!("{:?}", Countdown(5).collect::<Vec<_>>());
    println!("{:?}", Fib { a: 0, b: 1 }.take(10).collect::<Vec<_>>());

    // Free adapters on our own type
    let s: u32 = Countdown(4).zip(Countdown(10)).map(|(a, b)| a * b).sum();
    println!("zip-sum = {s}");

    let list = Playlist { songs: vec!["a".into(), "b".into()] };
    for song in &list {
        print!("{song} ");
    }
    println!();
    for song in list {
        print!("{} ", song.to_uppercase());
    }
    println!();

    println!("{:?}", evens_up_to(10).collect::<Vec<_>>());

    // peekable: look ahead without consuming - handy for parsers
    let mut chars = "123abc".chars().peekable();
    let mut digits = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            digits.push(c);
            chars.next();
        } else {
            break;
        }
    }
    println!("digits={digits} rest={}", chars.collect::<String>());

    // Other useful adapters
    let nested = vec![vec![1, 2], vec![3], vec![]];
    println!("{:?}", nested.iter().flatten().collect::<Vec<_>>());
    println!("{:?}", (1..=3).flat_map(|n| (0..n).map(move |m| (n, m))).count());
    println!("{:?}", [1, 5, 2].iter().scan(0, |acc, &x| { *acc += x; Some(*acc) }).collect::<Vec<_>>());
    let (even, odd): (Vec<i32>, Vec<i32>) = (1..=6).partition(|n| n % 2 == 0);
    println!("{even:?} {odd:?}");
    println!("{:?}", [3, 1, 2].iter().rev().step_by(2).collect::<Vec<_>>());
}

// ---------------------------------------------------------------------------
// CHALLENGE 16: Iterator workshop
//  1) `struct Collatz(u64)` yielding the Collatz sequence starting at the
//     number and ending at 1 (even -> n/2, odd -> 3n+1). Collatz(6) yields
//     6,3,10,5,16,8,4,2,1.
//  2) `struct Primes` - an infinite iterator of prime numbers.
//  3) `struct Grid { w: usize, h: usize }` with `cells(&self)` returning
//     `impl Iterator<Item = (usize, usize)>` over every (x, y), row by row.
// (See solutions/16_custom_iterators.rs)
// ---------------------------------------------------------------------------
