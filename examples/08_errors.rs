// LESSON 8: Error handling
//
// THEORY
// - Recoverable errors use `Result<T, E>` (`Ok(value)` or `Err(error)`).
//   Unrecoverable bugs use `panic!`. Prefer Result for anything that can
//   reasonably fail (parsing, I/O).
// - The `?` operator returns early with the error if there is one, converting
//   it via `From` - this keeps error handling short and readable.
// Docs: https://doc.rust-lang.org/book/ch09-00-error-handling.html

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Empty,
    BadNumber(ParseIntError),
    OutOfRange(i32),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Empty => write!(f, "input was empty"),
            ConfigError::BadNumber(e) => write!(f, "not a number: {e}"),
            ConfigError::OutOfRange(n) => write!(f, "{n} is outside 1..=100"),
        }
    }
}

impl std::error::Error for ConfigError {}

// Lets `?` convert ParseIntError into ConfigError automatically
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

fn parse_percent(input: &str) -> Result<i32, ConfigError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::Empty);
    }
    let n: i32 = trimmed.parse()?; // ParseIntError -> ConfigError
    if !(1..=100).contains(&n) {
        return Err(ConfigError::OutOfRange(n));
    }
    Ok(n)
}

// main can return Result too; Box<dyn Error> accepts any error type
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for input in ["42", "  7 ", "", "abc", "250"] {
        match parse_percent(input) {
            Ok(n) => println!("{input:?} -> ok {n}"),
            Err(e) => println!("{input:?} -> error: {e}"),
        }
    }

    // Quick-and-dirty helpers
    let n: i32 = "12".parse().unwrap_or(0);
    let m = "zz".parse::<i32>().unwrap_or_default();
    println!("{n} {m}");

    let total = parse_percent("10")? + parse_percent("20")?;
    println!("total = {total}");
    Ok(())
}

// ---------------------------------------------------------------------------
// CHALLENGE 8: Safe calculator
// Implement `calc(expr: &str) -> Result<f64, String>` that evaluates
// "<number> <op> <number>" where op is + - * /  (e.g. "6 / 3").
// Return Err with a helpful message for: wrong format, bad numbers,
// unknown operator, division by zero.
// Bonus: replace String with your own error enum like ConfigError above.
// Run: cargo test --example 08_errors
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn calc(expr: &str) -> Result<f64, String> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(calc("6 / 3"), Ok(2.0));
        assert_eq!(calc("2 + 3"), Ok(5.0));
        assert_eq!(calc("2.5 * 2"), Ok(5.0));
    }

    #[test]
    fn fails() {
        assert!(calc("1 / 0").is_err());
        assert!(calc("1 ^ 2").is_err());
        assert!(calc("x + 1").is_err());
        assert!(calc("1 +").is_err());
    }
}
