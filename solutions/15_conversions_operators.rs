// SOLUTION 15: Money

use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Money {
    cents: i64,
}

impl Add for Money {
    type Output = Money;
    fn add(self, o: Money) -> Money {
        Money { cents: self.cents + o.cents }
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, o: Money) -> Money {
        Money { cents: self.cents - o.cents }
    }
}

impl Mul<i64> for Money {
    type Output = Money;
    fn mul(self, q: i64) -> Money {
        Money { cents: self.cents * q }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.abs();
        write!(f, "{sign}${}.{:02}", abs / 100, abs % 100)
    }
}

impl From<i64> for Money {
    fn from(dollars: i64) -> Money {
        Money { cents: dollars * 100 }
    }
}

impl FromStr for Money {
    type Err = String;
    fn from_str(s: &str) -> Result<Money, String> {
        let s = s.trim().trim_start_matches('$');
        let (dollars, cents) = match s.split_once('.') {
            Some((d, c)) if c.len() == 2 => (d, c),
            Some(_) => return Err(format!("'{s}' needs exactly two decimals")),
            None => (s, "00"),
        };
        let d: i64 = dollars.parse().map_err(|_| format!("bad dollars in '{s}'"))?;
        let c: i64 = cents.parse().map_err(|_| format!("bad cents in '{s}'"))?;
        Ok(Money { cents: d * 100 + c })
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
        iter.fold(Money::default(), |a, b| a + b)
    }
}

fn main() {
    let price: Money = "$12.34".parse().unwrap();
    let total = price * 3 + Money::from(5);
    println!("{price} x3 + $5 = {total}");
    let all: Money = vec![price, total].into_iter().sum();
    println!("{all} {}", Money::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_and_parse() {
        assert_eq!("12.34".parse::<Money>().unwrap().to_string(), "$12.34");
        assert_eq!("$7".parse::<Money>().unwrap(), Money { cents: 700 });
        assert!("abc".parse::<Money>().is_err());
        assert!("1.5".parse::<Money>().is_err());
        assert_eq!(Money { cents: -5 }.to_string(), "-$0.05");
    }

    #[test]
    fn arithmetic() {
        let a = Money::from(10);
        let b = Money { cents: 250 };
        assert_eq!(a + b, Money { cents: 1250 });
        assert_eq!(a - b, Money { cents: 750 });
        assert_eq!(b * 4, Money::from(10));
        assert!(a > b);
    }

    #[test]
    fn summing() {
        let total: Money = [Money::from(1), Money::from(2)].into_iter().sum();
        assert_eq!(total, Money::from(3));
    }
}
