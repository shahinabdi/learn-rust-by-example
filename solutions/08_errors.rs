// SOLUTION 8: Safe calculator (with the bonus: a custom error enum)

use std::fmt;

#[derive(Debug, PartialEq)]
enum CalcError {
    Format,
    BadNumber(String),
    UnknownOp(String),
    DivideByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Format => write!(f, "expected '<number> <op> <number>'"),
            CalcError::BadNumber(s) => write!(f, "'{s}' is not a number"),
            CalcError::UnknownOp(s) => write!(f, "unknown operator '{s}'"),
            CalcError::DivideByZero => write!(f, "division by zero"),
        }
    }
}

fn number(s: &str) -> Result<f64, CalcError> {
    s.parse().map_err(|_| CalcError::BadNumber(s.to_string()))
}

fn calc_typed(expr: &str) -> Result<f64, CalcError> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    let [a, op, b] = parts[..] else {
        return Err(CalcError::Format);
    };
    let (a, b) = (number(a)?, number(b)?);
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" if b == 0.0 => Err(CalcError::DivideByZero),
        "/" => Ok(a / b),
        other => Err(CalcError::UnknownOp(other.to_string())),
    }
}

// The signature requested by the challenge
fn calc(expr: &str) -> Result<f64, String> {
    calc_typed(expr).map_err(|e| e.to_string())
}

fn main() {
    for e in ["6 / 3", "2.5 * 2", "1 / 0", "1 ^ 2", "x + 1", "1 +"] {
        match calc(e) {
            Ok(v) => println!("{e} = {v}"),
            Err(msg) => println!("{e} -> error: {msg}"),
        }
    }
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
        assert_eq!(calc_typed("1 / 0"), Err(CalcError::DivideByZero));
    }
}
