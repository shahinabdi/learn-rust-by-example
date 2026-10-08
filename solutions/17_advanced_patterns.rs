// SOLUTION 17: RPN calculator and slice classifier

fn eval_rpn(tokens: &[&str]) -> Result<i64, String> {
    let mut stack: Vec<i64> = Vec::new();
    for &tok in tokens {
        match tok {
            "+" | "-" | "*" | "/" => {
                let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else {
                    return Err("stack underflow".into());
                };
                stack.push(match tok {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ if b == 0 => return Err("division by zero".into()),
                    _ => a / b,
                });
            }
            number => {
                let n = number.parse::<i64>().map_err(|_| format!("bad token '{number}'"))?;
                stack.push(n);
            }
        }
    }
    match stack[..] {
        [result] => Ok(result),
        [] => Err("empty expression".into()),
        _ => Err("leftover values on the stack".into()),
    }
}

fn classify(v: &[i32]) -> &'static str {
    match v {
        [] => "empty",
        [_] => "single",
        [_, _] => "pair",
        _ if v.windows(2).all(|w| w[0] <= w[1]) => "sorted",
        _ => "other",
    }
}

fn main() {
    println!("{:?}", eval_rpn(&["3", "4", "+", "2", "*"]));
    println!("{:?}", eval_rpn(&["1", "0", "/"]));
    println!("{} {} {}", classify(&[]), classify(&[1, 2, 3]), classify(&[3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpn_ok() {
        assert_eq!(eval_rpn(&["3", "4", "+", "2", "*"]), Ok(14));
        assert_eq!(eval_rpn(&["5", "1", "2", "+", "4", "*", "+", "3", "-"]), Ok(14));
    }

    #[test]
    fn rpn_errors() {
        assert!(eval_rpn(&["+"]).is_err());
        assert!(eval_rpn(&["1", "0", "/"]).is_err());
        assert!(eval_rpn(&["x"]).is_err());
        assert!(eval_rpn(&["1", "2"]).is_err());
        assert!(eval_rpn(&[]).is_err());
    }

    #[test]
    fn classify_slices() {
        assert_eq!(classify(&[]), "empty");
        assert_eq!(classify(&[1]), "single");
        assert_eq!(classify(&[2, 1]), "pair");
        assert_eq!(classify(&[1, 2, 3]), "sorted");
        assert_eq!(classify(&[3, 1, 2]), "other");
    }
}
