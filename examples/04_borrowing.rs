// LESSON 4: References, borrowing and slices
//
// THEORY
// - `&T` borrows a value without taking ownership (shared, read-only).
//   `&mut T` is an exclusive, writable borrow.
// - The borrow rules, enforced at compile time: at any moment you may have
//   EITHER any number of `&T` OR exactly one `&mut T`, never both. This
//   prevents data races and dangling pointers.
// - A slice (`&str`, `&[T]`) is a borrowed view into part of a collection.
// Docs: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
//       https://doc.rust-lang.org/book/ch04-03-slices.html

fn length(s: &String) -> usize {
    s.len() // read only, no ownership taken
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let mut text = String::from("hello");

    // Many shared borrows are fine
    let r1 = &text;
    let r2 = &text;
    println!("{r1} {r2} len={}", length(&text));

    // A mutable borrow is allowed once the shared ones are no longer used
    append_world(&mut text);
    println!("{text}");

    // This would not compile (uncomment to read the error):
    // let m = &mut text;
    // let s = &text;
    // m.push('!');
    // println!("{s}");

    // Slices
    let sentence = String::from("borrow checker rocks");
    let word = first_word(&sentence);
    println!("first word: {word}");

    let numbers = [1, 2, 3, 4, 5, 6];
    println!("whole={} first3={} last2={}", sum(&numbers), sum(&numbers[..3]), sum(&numbers[4..]));

    // Mutating through a slice
    let mut data = vec![3, 1, 2];
    let slice: &mut [i32] = &mut data;
    slice[0] = 99;
    slice.sort();
    println!("{data:?}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 4: Slice utilities
// 1) `largest(&[i32]) -> Option<i32>`: largest value, None if empty.
// 2) `double_all(&mut [i32])`: multiply each element by 2 in place.
// 3) `last_word(&str) -> &str`: the final whitespace-separated word.
// Run: cargo test --example 04_borrowing
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn largest(values: &[i32]) -> Option<i32> {
    todo!("implement me")
}

#[allow(dead_code, unused_variables)]
fn double_all(values: &mut [i32]) {
    todo!("implement me")
}

#[allow(dead_code, unused_variables)]
fn last_word(s: &str) -> &str {
    todo!("implement me")
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
