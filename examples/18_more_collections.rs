// LESSON 18: More collections, sorting and Cow
//
// THEORY
// - Pick the structure by access pattern:
//   Vec (fast index/push) | VecDeque (fast both ends) | HashMap/HashSet (O(1)
//   lookup, unordered) | BTreeMap/BTreeSet (sorted, O(log n)) | BinaryHeap
//   (always gives the largest item: priority queue).
// - `Cow<str>` ("clone on write") borrows when no change is needed and only
//   allocates when you modify - great for functions that RARELY change input.
// Docs: https://doc.rust-lang.org/std/collections/index.html

use std::borrow::Cow;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn normalize(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_")) // allocates only when needed
    } else {
        Cow::Borrowed(input)
    }
}

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: u32,
}

fn main() {
    // HashSet: uniqueness and set algebra
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let mut inter: Vec<_> = a.intersection(&b).copied().collect();
    inter.sort();
    let mut uni: Vec<_> = a.union(&b).copied().collect();
    uni.sort();
    println!("intersection={inter:?} union={uni:?}");

    // Dedup trick: insert returns false if already present
    let mut seen = HashSet::new();
    let dups: Vec<_> = [1, 2, 1, 3, 2].into_iter().filter(|x| !seen.insert(*x)).collect();
    println!("duplicates={dups:?}");

    // BTreeMap: iterates in key order, supports ranges
    let mut grades = BTreeMap::new();
    grades.insert("carol", 90);
    grades.insert("alice", 85);
    grades.insert("bob", 70);
    println!("{grades:?}");
    println!("first={:?} range={:?}", grades.iter().next(), grades.range("b".."c").collect::<Vec<_>>());

    // VecDeque: a queue (BFS, sliding windows)
    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_front(0);
    println!("{q:?}");
    println!("pop_front={:?} then {q:?}", q.pop_front());

    // BinaryHeap: max-heap; wrap in Reverse for a min-heap
    let mut heap = BinaryHeap::from(vec![5, 1, 8, 3]);
    print!("max-first: ");
    while let Some(x) = heap.pop() {
        print!("{x} ");
    }
    let mut min_heap = BinaryHeap::new();
    for x in [5, 1, 8] {
        min_heap.push(Reverse(x));
    }
    println!("| min={:?}", min_heap.pop().map(|Reverse(x)| x));

    // Sorting
    let mut people = vec![
        Person { name: "Zed", age: 30 },
        Person { name: "Amy", age: 25 },
        Person { name: "Bob", age: 30 },
    ];
    people.sort_by_key(|p| p.age);
    println!("{:?}", people.iter().map(|p| p.name).collect::<Vec<_>>());
    people.sort_by(|a, b| b.age.cmp(&a.age).then(a.name.cmp(b.name))); // age desc, name asc
    println!("{:?}", people.iter().map(|p| p.name).collect::<Vec<_>>());
    let mut nums = vec![3, 1, 2, 3, 1];
    nums.sort_unstable();
    nums.dedup();
    println!("{nums:?} binary_search(2)={:?}", nums.binary_search(&2));

    // Grouping with entry().or_default()
    let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
    let mut by_letter: HashMap<char, Vec<&str>> = HashMap::new();
    for w in words {
        by_letter.entry(w.chars().next().unwrap()).or_default().push(w);
    }
    let mut keys: Vec<_> = by_letter.keys().collect();
    keys.sort();
    for k in keys {
        println!("{k}: {:?}", by_letter[k]);
    }

    // Cow
    for s in ["no_spaces", "has some spaces"] {
        let out = normalize(s);
        let kind = if matches!(out, Cow::Borrowed(_)) { "borrowed" } else { "owned" };
        println!("{out} ({kind})");
    }
}

// ---------------------------------------------------------------------------
// CHALLENGE 18: Data-structure toolbox
//  1) `top_n(words: &[&str], n: usize) -> Vec<(String, usize)>`: the n most
//     frequent words, ties broken alphabetically. (HashMap + sort)
//  2) `shortest_path(grid: &[&str]) -> Option<usize>`: BFS with VecDeque on a
//     maze where '#' is a wall, 'S' start, 'E' end, '.' open. Returns the
//     number of steps.
//  3) `merge_sorted(lists: Vec<Vec<i32>>) -> Vec<i32>`: merge many sorted
//     lists using a BinaryHeap of Reverse((value, list_idx, element_idx)).
// (See solutions/18_more_collections.rs)
// ---------------------------------------------------------------------------
