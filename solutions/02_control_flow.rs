// SOLUTION 2: FizzBuzz and Fibonacci

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

// Iterative: O(n) time, O(1) memory
fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

// Recursive version for comparison: O(2^n) - try fib_recursive(40) and feel the pain
fn fib_recursive(n: u32) -> u64 {
    if n < 2 { n as u64 } else { fib_recursive(n - 1) + fib_recursive(n - 2) }
}

fn main() {
    for n in 1..=15 {
        print!("{} ", fizzbuzz(n));
    }
    println!();
    println!("fib(10) = {}, recursive = {}", fibonacci(10), fib_recursive(10));
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
        assert_eq!(fib_recursive(10), 55);
    }
}
