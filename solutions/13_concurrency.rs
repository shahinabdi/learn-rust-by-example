// SOLUTION 13: Parallel sum - channel version and Arc<Mutex> version

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn chunk_size(len: usize, parts: usize) -> usize {
    len.div_ceil(parts.max(1)).max(1)
}

fn parallel_sum(data: Vec<u64>, parts: usize) -> u64 {
    let (tx, rx) = mpsc::channel();
    for chunk in data.chunks(chunk_size(data.len(), parts)) {
        let tx = tx.clone();
        let chunk = chunk.to_vec(); // each thread owns its slice of the work
        thread::spawn(move || tx.send(chunk.iter().sum::<u64>()).unwrap());
    }
    drop(tx);
    rx.iter().sum()
}

fn parallel_sum_mutex(data: Vec<u64>, parts: usize) -> u64 {
    let total = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];
    for chunk in data.chunks(chunk_size(data.len(), parts)) {
        let total = Arc::clone(&total);
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            let part: u64 = chunk.iter().sum();
            *total.lock().unwrap() += part;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let result = *total.lock().unwrap();
    result
}

fn main() {
    let data: Vec<u64> = (1..=1000).collect();
    println!("{} {}", parallel_sum(data.clone(), 4), parallel_sum_mutex(data, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        let data: Vec<u64> = (1..=1000).collect();
        assert_eq!(parallel_sum(data.clone(), 4), 500_500);
        assert_eq!(parallel_sum_mutex(data, 4), 500_500);
        assert_eq!(parallel_sum(vec![], 4), 0);
    }
}
