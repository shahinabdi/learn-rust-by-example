// SOLUTION 7: Anagram groups

use std::collections::HashMap;

fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        let mut letters: Vec<char> = word.chars().collect();
        letters.sort_unstable();
        let key: String = letters.into_iter().collect();
        map.entry(key).or_default().push(word.to_string());
    }
    let mut groups: Vec<Vec<String>> = map.into_values().collect();
    for g in groups.iter_mut() {
        g.sort();
    }
    groups.sort(); // Vec<String> is Ord, so groups sort lexicographically
    groups
}

fn main() {
    let out = group_anagrams(&["listen", "silent", "enlist", "google", "gogole", "cat"]);
    println!("{out:?}");
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
