// LESSON 20: Testing
//
// THEORY
// - `#[test]` functions live next to the code in a `#[cfg(test)]` module (unit
//   tests, may touch private items). Integration tests live in `tests/` and
//   use only the public API. Doc-tests are code examples inside `///` comments
//   that `cargo test` compiles and runs - they keep docs honest.
// - Useful macros: assert!, assert_eq!, assert_ne!, `#[should_panic]`, and
//   tests that return `Result<(), E>` so you can use `?`.
// - Run a subset with `cargo test name`, show output with `-- --nocapture`.
// Docs: https://doc.rust-lang.org/book/ch11-00-testing.html
//
// This lesson is a TEST-FIRST challenge. `is_leap_year` below is BUGGY.

/// Returns true if `year` is a leap year in the Gregorian calendar.
///
/// Rules: divisible by 4, except centuries, unless divisible by 400.
///
/// ```
/// // (doc-tests only run for library crates, shown here as documentation)
/// // assert!(is_leap_year(2024));
/// ```
fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 // BUG: ignores the century rules
}

/// Splits "key=value" pairs. Returns None when '=' is missing.
fn parse_pair(s: &str) -> Option<(&str, &str)> {
    s.split_once('=')
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

fn main() {
    for y in [1996, 1900, 2000, 2023, 2024] {
        println!("{y}: {}", is_leap_year(y));
    }
    println!("{:?}", parse_pair("a=1"));
    println!("{}", divide(6, 3));
    println!("Now run: cargo test --example 20_testing");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_leap_years() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
    }

    // Tests that expose the bug. They FAIL until you fix `is_leap_year`.
    #[test]
    fn century_is_not_leap() {
        assert!(!is_leap_year(1900), "1900 is a century, not divisible by 400");
    }

    #[test]
    fn four_hundred_is_leap() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn pair_parsing() {
        assert_eq!(parse_pair("a=1"), Some(("a", "1")));
        assert_eq!(parse_pair("a=b=c"), Some(("a", "b=c")));
        assert_eq!(parse_pair("nothing"), None);
        assert_ne!(parse_pair("x=1"), None);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics() {
        divide(1, 0);
    }

    // Tests can return Result so you can use `?`
    #[test]
    fn parses_numbers() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }

    // Table-driven test: many cases, one loop, a message that says which failed
    #[test]
    fn leap_table() {
        let cases = [(1600, true), (1700, false), (1800, false), (2004, true), (2100, false)];
        for (year, expected) in cases {
            assert_eq!(is_leap_year(year), expected, "year {year}");
        }
    }

    #[test]
    #[ignore = "slow; run with: cargo test -- --ignored"]
    fn slow_test() {
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

// ---------------------------------------------------------------------------
// CHALLENGE 20: Make the red tests green
//  1) Run `cargo test --example 20_testing` and read the failures.
//  2) Fix `is_leap_year` (do NOT edit the tests).
//  3) Write `fn clamp_percent(n: i32) -> u8` (0..=100) and write the tests
//     FIRST (below zero, above 100, boundaries) - this is test-driven
//     development (TDD): red -> green -> refactor.
//  4) Bonus: create a real `tests/` folder with an integration test in a
//     library crate (see projects/minigrep for a layout to copy).
// (See solutions/20_testing.rs)
// ---------------------------------------------------------------------------
