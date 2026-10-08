// LESSON 13: Concurrency - threads, channels, Arc<Mutex<T>>
//
// THEORY
// - `thread::spawn` runs a closure on a new OS thread; `join()` waits for it.
// - Data races are impossible in safe Rust: types must be `Send`/`Sync` to
//   cross threads. Share ownership with `Arc` (atomic Rc) and guard mutation
//   with `Mutex`. Or avoid sharing: pass messages through channels.
// Docs: https://doc.rust-lang.org/book/ch16-00-concurrency.html

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    // 1) spawn + move + join
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || data.iter().sum::<i32>());
    println!("sum from thread = {}", handle.join().unwrap());

    // 2) Arc<Mutex<T>>: shared mutable counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter = {}", *counter.lock().unwrap()); // always 8000

    // 3) Channels: multiple producers, single consumer
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("worker {id} done")).unwrap();
        });
    }
    drop(tx); // close the original sender so the receiver loop can end
    let mut messages: Vec<String> = rx.iter().collect();
    messages.sort();
    for m in messages {
        println!("{m}");
    }

    // 4) Scoped threads: borrow local data without Arc
    let mut numbers = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = numbers.split_at_mut(3);
    thread::scope(|s| {
        s.spawn(|| left.iter_mut().for_each(|x| *x *= 10));
        s.spawn(|| right.iter_mut().for_each(|x| *x *= 100));
    });
    println!("{numbers:?}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 13: Parallel sum
// Implement `parallel_sum(data: Vec<u64>, parts: usize) -> u64` that splits
// the data into `parts` chunks, sums each chunk in its own thread, and adds up
// the results (try BOTH a channel version and an Arc<Mutex<u64>> version).
// Run: cargo test --example 13_concurrency
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn parallel_sum(data: Vec<u64>, parts: usize) -> u64 {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        let data: Vec<u64> = (1..=1000).collect();
        assert_eq!(parallel_sum(data, 4), 500_500);
    }
}
