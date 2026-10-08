// SOLUTION 11: Iterator drills

fn sum_of_odd_squares(v: &[i32]) -> i32 {
    v.iter().filter(|&&n| n % 2 != 0).map(|n| n * n).sum()
}

fn longest_word(s: &str) -> Option<&str> {
    // max_by_key returns the LAST max, so fold manually to keep the first
    s.split_whitespace()
        .fold(None, |best: Option<&str>, w| match best {
            Some(b) if b.len() >= w.len() => Some(b),
            _ => Some(w),
        })
}

fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

fn main() {
    println!("{}", sum_of_odd_squares(&[1, 2, 3, 4]));
    println!("{:?}", longest_word("a quick brown fox"));
    println!("{}", is_palindrome("A man, a plan, a canal: Panama"));
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
