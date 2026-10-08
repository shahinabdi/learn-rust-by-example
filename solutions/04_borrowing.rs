// SOLUTION 4: Slice utilities

fn largest(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn double_all(values: &mut [i32]) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}

fn last_word(s: &str) -> &str {
    s.split_whitespace().last().unwrap_or("")
}

fn main() {
    println!("{:?}", largest(&[3, 9, 2]));
    let mut v = [1, 2, 3];
    double_all(&mut v);
    println!("{v:?} / {}", last_word("the quick fox"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_works() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&[]), None);
    }

    #[test]
    fn double_works() {
        let mut v = [1, 2, 3];
        double_all(&mut v);
        assert_eq!(v, [2, 4, 6]);
    }

    #[test]
    fn last_word_works() {
        assert_eq!(last_word("the quick fox"), "fox");
        assert_eq!(last_word("solo"), "solo");
    }
}
