//! Expression calculator: tokenizer -> recursive-descent parser -> AST -> eval.
//!
//! Grammar (lowest to highest precedence):
//!   expr   := term (('+' | '-') term)*
//!   term   := unary (('*' | '/') unary)*
//!   unary  := '-' unary | power
//!   power  := atom ('^' unary)?          (right associative)
//!   atom   := NUMBER | '(' expr ')'

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
pub enum CalcError {
    UnexpectedChar(char),
    BadNumber(String),
    UnexpectedToken(Token),
    UnexpectedEnd,
    DivisionByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::UnexpectedChar(c) => write!(f, "unexpected character '{c}'"),
            CalcError::BadNumber(s) => write!(f, "bad number '{s}'"),
            CalcError::UnexpectedToken(t) => write!(f, "unexpected token {t:?}"),
            CalcError::UnexpectedEnd => write!(f, "unexpected end of input"),
            CalcError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for CalcError {}

pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut chars: Peekable<Chars> = input.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut s = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        s.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n = s.parse::<f64>().map_err(|_| CalcError::BadNumber(s))?;
                tokens.push(Token::Num(n));
            }
            '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                chars.next();
                tokens.push(match c {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '^' => Token::Caret,
                    '(' => Token::LParen,
                    _ => Token::RParen,
                });
            }
            other => return Err(CalcError::UnexpectedChar(other)),
        }
    }
    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>), // Box: a recursive type needs a known size -> heap indirection
    Bin(Box<Expr>, char, Box<Expr>),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    fn expr(&mut self) -> Result<Expr, CalcError> {
        let mut left = self.term()?;
        while let Some(op) = match self.peek() {
            Some(Token::Plus) => Some('+'),
            Some(Token::Minus) => Some('-'),
            _ => None,
        } {
            self.next();
            left = Expr::Bin(Box::new(left), op, Box::new(self.term()?));
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, CalcError> {
        let mut left = self.unary()?;
        while let Some(op) = match self.peek() {
            Some(Token::Star) => Some('*'),
            Some(Token::Slash) => Some('/'),
            _ => None,
        } {
            self.next();
            left = Expr::Bin(Box::new(left), op, Box::new(self.unary()?));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, CalcError> {
        if self.peek() == Some(&Token::Minus) {
            self.next();
            return Ok(Expr::Neg(Box::new(self.unary()?)));
        }
        self.power()
    }

    fn power(&mut self) -> Result<Expr, CalcError> {
        let base = self.atom()?;
        if self.peek() == Some(&Token::Caret) {
            self.next();
            return Ok(Expr::Bin(Box::new(base), '^', Box::new(self.unary()?)));
        }
        Ok(base)
    }

    fn atom(&mut self) -> Result<Expr, CalcError> {
        match self.next() {
            Some(Token::Num(n)) => Ok(Expr::Num(n)),
            Some(Token::LParen) => {
                let e = self.expr()?;
                match self.next() {
                    Some(Token::RParen) => Ok(e),
                    Some(t) => Err(CalcError::UnexpectedToken(t)),
                    None => Err(CalcError::UnexpectedEnd),
                }
            }
            Some(t) => Err(CalcError::UnexpectedToken(t)),
            None => Err(CalcError::UnexpectedEnd),
        }
    }
}

pub fn parse(input: &str) -> Result<Expr, CalcError> {
    let mut p = Parser { tokens: tokenize(input)?, pos: 0 };
    let e = p.expr()?;
    match p.next() {
        None => Ok(e),
        Some(t) => Err(CalcError::UnexpectedToken(t)),
    }
}

pub fn eval(e: &Expr) -> Result<f64, CalcError> {
    Ok(match e {
        Expr::Num(n) => *n,
        Expr::Neg(inner) => -eval(inner)?,
        Expr::Bin(l, op, r) => {
            let (a, b) = (eval(l)?, eval(r)?);
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' if b == 0.0 => return Err(CalcError::DivisionByZero),
                '/' => a / b,
                '^' => a.powf(b),
                _ => unreachable!("parser only produces + - * / ^"),
            }
        }
    })
}

pub fn calculate(input: &str) -> Result<f64, CalcError> {
    eval(&parse(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence() {
        assert_eq!(calculate("2 + 3 * 4"), Ok(14.0));
        assert_eq!(calculate("(2 + 3) * 4"), Ok(20.0));
    }

    #[test]
    fn left_associativity() {
        assert_eq!(calculate("10 - 4 - 3"), Ok(3.0));
        assert_eq!(calculate("100 / 10 / 2"), Ok(5.0));
    }

    #[test]
    fn power_is_right_associative() {
        assert_eq!(calculate("2 ^ 3 ^ 2"), Ok(512.0));
    }

    #[test]
    fn unary_minus() {
        assert_eq!(calculate("-3 + 5"), Ok(2.0));
        assert_eq!(calculate("--4"), Ok(4.0));
        assert_eq!(calculate("2 * -3"), Ok(-6.0));
    }

    #[test]
    fn errors() {
        assert_eq!(calculate("1 / 0"), Err(CalcError::DivisionByZero));
        assert_eq!(calculate("2 +"), Err(CalcError::UnexpectedEnd));
        assert_eq!(calculate("(1 + 2"), Err(CalcError::UnexpectedEnd));
        assert_eq!(calculate("1 $ 2"), Err(CalcError::UnexpectedChar('$')));
        assert_eq!(calculate("1.2.3"), Err(CalcError::BadNumber("1.2.3".into())));
        assert!(matches!(calculate("1 2"), Err(CalcError::UnexpectedToken(_))));
    }

    #[test]
    fn tokenizer() {
        assert_eq!(tokenize("1+2").unwrap(), vec![Token::Num(1.0), Token::Plus, Token::Num(2.0)]);
    }
}
