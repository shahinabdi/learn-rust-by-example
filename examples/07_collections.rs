// LESSON 7: Collections - Vec, String, HashMap
//
// THEORY
// - Vec<T>: growable array on the HEAP. String: growable UTF-8 text (a Vec<u8>
//   underneath), so you cannot index it by integer - use slices/chars().
// - HashMap<K, V>: key -> value lookup, average O(1). Use the `entry` API to
//   insert-or-update in one step.
// Docs: https://doc.rust-lang.org/book/ch08-00-common-collections.html

use std::collections::HashMap;

fn main() {
    // ---- Vec ----
    let mut v = vec![10, 20, 30];
    v.push(40);
    v.insert(1, 15);
    let popped = v.pop(); // Option<i32>
    println!("{v:?} popped={popped:?} len={} cap>={}", v.len(), v.capacity());

    // Safe access: get returns Option, indexing panics if out of range
    println!("{:?} {:?}", v.get(1), v.get(99));

    for x in v.iter_mut() {
        *x += 1;
    }
    println!("{v:?}");

    // ---- String ----
    let mut s = String::from("Hello");
    s.push(' ');
    s.push_str("Rust");
    let shout = s.to_uppercase();
    println!("{s} | {shout} | contains Rust? {}", s.contains("Rust"));
    println!("chars: {}, bytes: {}", "héllo".chars().count(), "héllo".len());
    let joined = format!("{s}-{}", 2024);
    println!("{joined}");
    for word in "split these words".split_whitespace() {
        print!("[{word}]");
    }
    println!();

    // ---- HashMap: word frequency ----
    let text = "the quick brown fox jumps over the lazy dog the end";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = counts.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0))); // by count desc, then name
    for (word, n) in pairs.iter().take(3) {
        println!("{word}: {n}");
    }

    // Lookup
    if let Some(n) = counts.get("fox") {
        println!("fox appears {n} time(s)");
    }
    println!("has 'cat'? {}", counts.contains_key("cat"));
}

// ---------------------------------------------------------------------------
// CHALLENGE 7: Anagram groups
// Implement `group_anagrams(words: &[&str]) -> Vec<Vec<String>>`.
// Words that use the same letters belong in the same group.
// Hint: sort the characters of each word and use that as the HashMap key.
// Sort groups (and words inside) so the output is deterministic.
// Run: cargo test --example 07_collections
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups() {
        let out = group_anagrams(&["listen", "silent", "enlist", "google", "gogole", "cat"]);
        assert_eq!(
            out,
            vec![
                vec!["cat".to_string()],
                vec!["enlist".to_string(), "listen".to_string(), "silent".to_string()],
                vec!["gogole".to_string(), "google".to_string()],
            ]
        );
    }
}
