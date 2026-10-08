// SOLUTION 18: Data-structure toolbox

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn top_n(words: &[&str], n: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for w in words {
        *counts.entry(w).or_insert(0) += 1;
    }
    let mut pairs: Vec<(String, usize)> = counts.into_iter().map(|(w, c)| (w.to_string(), c)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.truncate(n);
    pairs
}

fn shortest_path(grid: &[&str]) -> Option<usize> {
    let cells: Vec<Vec<char>> = grid.iter().map(|r| r.chars().collect()).collect();
    let find = |target: char| {
        cells.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == target).map(|x| (x, y)))
    };
    let start = find('S')?;
    let end = find('E')?;

    let mut queue = VecDeque::from([(start, 0usize)]);
    let mut seen = HashSet::from([start]);
    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps);
        }
        let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let open = cells.get(ny).and_then(|r| r.get(nx)).is_some_and(|&c| c != '#');
            if open && seen.insert((nx, ny)) {
                queue.push_back(((nx, ny), steps + 1));
            }
        }
    }
    None
}

fn merge_sorted(lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for (li, list) in lists.iter().enumerate() {
        if let Some(&v) = list.first() {
            heap.push(Reverse((v, li, 0usize)));
        }
    }
    let mut out = Vec::new();
    while let Some(Reverse((v, li, ei))) = heap.pop() {
        out.push(v);
        if let Some(&next) = lists[li].get(ei + 1) {
            heap.push(Reverse((next, li, ei + 1)));
        }
    }
    out
}

fn main() {
    println!("{:?}", top_n(&["a", "b", "a", "c", "b", "a"], 2));
    println!("{:?}", shortest_path(&["S.#", "..#", "#.E"]));
    println!("{:?}", merge_sorted(vec![vec![1, 4, 7], vec![2, 5], vec![0, 9]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top() {
        let out = top_n(&["b", "a", "b", "a", "c"], 2);
        assert_eq!(out, vec![("a".to_string(), 2), ("b".to_string(), 2)]);
    }

    #[test]
    fn maze() {
        assert_eq!(shortest_path(&["S.#", "..#", "#.E"]), Some(4));
        assert_eq!(shortest_path(&["S#E"]), None);
    }

    #[test]
    fn merge() {
        assert_eq!(merge_sorted(vec![vec![1, 4], vec![2, 3], vec![]]), vec![1, 2, 3, 4]);
    }
}
