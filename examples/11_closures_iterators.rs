// LESSON 11: Closures and iterators
//
// THEORY
// - A closure is an anonymous function that can capture variables from its
//   environment: `|x| x + offset`.
// - Iterators are lazy: adapters like `map`/`filter` do nothing until a
//   consumer (`collect`, `sum`, `for`) pulls values. They compile down to
//   loops as fast as hand-written ones ("zero-cost abstractions").
// Docs: https://doc.rust-lang.org/book/ch13-00-functional-features.html

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // `move` takes ownership of n
}

fn main() {
    // Closures
    let offset = 10;
    let add_offset = |x| x + offset; // borrows offset
    println!("{}", apply(add_offset, 5));
    let add5 = make_adder(5);
    println!("{}", add5(1));

    let mut count = 0;
    let mut inc = || count += 1; // mutably borrows count
    inc();
    inc();
    println!("count = {count}");

    // Iterator pipeline
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    println!("{even_squares:?}");

    // Common consumers
    let sum: i32 = numbers.iter().sum();
    let product: i64 = numbers.iter().map(|&n| n as i64).product();
    println!("sum={sum} product={product}");
    println!("any>9? {} all>0? {}", numbers.iter().any(|&n| n > 9), numbers.iter().all(|&n| n > 0));
    println!("first>4: {:?}", numbers.iter().find(|&&n| n > 4));
    println!("position of 7: {:?}", numbers.iter().position(|&n| n == 7));
    println!("max={:?} min={:?}", numbers.iter().max(), numbers.iter().min());

    // enumerate, zip, take, skip, windows, chunks
    for (i, ch) in "abc".chars().enumerate() {
        print!("{i}:{ch} ");
    }
    println!();
    let names = ["a", "b", "c"];
    let ages = [1, 2, 3];
    let zipped: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("{zipped:?}");
    println!("{:?}", numbers.iter().skip(2).take(3).collect::<Vec<_>>());
    println!("{:?}", numbers.windows(3).map(|w| w.iter().sum::<i32>()).collect::<Vec<_>>());
    println!("{:?}", numbers.chunks(4).collect::<Vec<_>>());

    // fold: the general-purpose consumer
    let csv = numbers.iter().fold(String::new(), |acc, n| {
        if acc.is_empty() { n.to_string() } else { format!("{acc},{n}") }
    });
    println!("{csv}");

    // Infinite lazy iterator
    let powers: Vec<u32> = (0..).map(|i| 2u32.pow(i)).take_while(|&p| p < 100).collect();
    println!("{powers:?}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 11: Iterator drills (use iterator chains, NO for loops)
//  1) `sum_of_odd_squares(&[i32]) -> i32`
//  2) `longest_word(&str) -> Option<&str>` (first one on ties)
//  3) `is_palindrome(&str) -> bool` (ignore case & non-alphanumerics)
// Run: cargo test --example 11_closures_iterators
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn sum_of_odd_squares(v: &[i32]) -> i32 {
    todo!("implement me")
}

#[allow(dead_code, unused_variables)]
fn longest_word(s: &str) -> Option<&str> {
    todo!("implement me")
}

#[allow(dead_code, unused_variables)]
fn is_palindrome(s: &str) -> bool {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_squares() {
        assert_eq!(sum_of_odd_squares(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn longest() {
        assert_eq!(longest_word("a quick brown fox"), Some("quick"));
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome("hello"));
    }
}
