// LESSON 2: Functions and control flow
//
// THEORY
// - Rust distinguishes statements (do something, no value) from expressions
//   (evaluate to a value). The last expression of a block, WITHOUT a semicolon,
//   is its return value. `if`, `match` and blocks are all expressions.
// - Loops: `loop` (infinite, can return a value), `while`, `for` over iterators.
// Docs: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
//       https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn square(x: i32) -> i32 {
    x * x // no semicolon => returned
}

fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1) // recursion
    }
}

fn main() {
    println!("square(7) = {}", square(7));
    for n in [-3, 0, 9] {
        println!("{n} is {}", classify(n));
    }

    // if as an expression
    let parity = if 10 % 2 == 0 { "even" } else { "odd" };
    println!("10 is {parity}");

    // loop that returns a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("loop result = {result}");

    // while
    let mut n = 3;
    while n > 0 {
        println!("countdown {n}");
        n -= 1;
    }

    // for with ranges: 1..4 is exclusive (1,2,3), 1..=4 is inclusive
    for i in 1..=4 {
        print!("{i} ");
    }
    println!();
    for i in (1..4).rev() {
        print!("{i} ");
    }
    println!();

    // Labeled breaks for nested loops
    'outer: for a in 1..10 {
        for b in 1..10 {
            if a * b == 42 {
                println!("found {a} * {b} = 42");
                break 'outer;
            }
        }
    }

    println!("5! = {}", factorial(5));
}

// ---------------------------------------------------------------------------
// CHALLENGE 2: FizzBuzz and Fibonacci
// 1) `fizzbuzz(n)` returns "Fizz" for multiples of 3, "Buzz" for 5,
//    "FizzBuzz" for both, otherwise the number as a String.
// 2) `fibonacci(n)` returns the n-th Fibonacci number (0, 1, 1, 2, 3, 5...)
//    using a loop (not recursion). Bonus: try both and compare.
// Run: cargo test --example 02_control_flow
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn fizzbuzz(n: u32) -> String {
    todo!("implement me")
}

#[allow(dead_code, unused_variables)]
fn fibonacci(n: u32) -> u64 {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn fib() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
