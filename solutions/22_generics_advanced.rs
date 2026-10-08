// SOLUTION 22: Generic toolkit

use std::collections::VecDeque;
use std::fmt::Display;

struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: VecDeque::new() }
    }
    fn enqueue(&mut self, t: T) {
        self.items.push_back(t);
    }
    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Display> Queue<T> {
    fn render(&self) -> String {
        self.items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" <- ")
    }
}

fn largest_by_key<T, K: PartialOrd>(items: &[T], key: impl Fn(&T) -> K) -> Option<&T> {
    let mut best: Option<&T> = None;
    for item in items {
        match best {
            Some(b) if key(item) <= key(b) => {}
            _ => best = Some(item),
        }
    }
    best
}

struct Ring<T, const N: usize> {
    buf: [Option<T>; N],
    start: usize,
    len: usize,
}

impl<T, const N: usize> Ring<T, N> {
    fn new() -> Self {
        Ring { buf: std::array::from_fn(|_| None), start: 0, len: 0 }
    }

    fn push(&mut self, value: T) {
        if self.len < N {
            let idx = (self.start + self.len) % N;
            self.buf[idx] = Some(value);
            self.len += 1;
        } else {
            self.buf[self.start] = Some(value); // overwrite the oldest
            self.start = (self.start + 1) % N;
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len).filter_map(move |i| self.buf[(self.start + i) % N].as_ref())
    }
}

fn main() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    println!("{} len={} first={:?} empty={}", q.render(), q.len(), q.dequeue(), q.is_empty());

    let words = ["hi", "hello", "hey"];
    println!("{:?}", largest_by_key(&words, |w| w.len()));

    let mut r: Ring<i32, 3> = Ring::new();
    for i in 1..=5 {
        r.push(i);
    }
    println!("{:?}", r.iter().collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_is_fifo() {
        let mut q = Queue::new();
        q.enqueue("a");
        q.enqueue("b");
        assert_eq!(q.render(), "a <- b");
        assert_eq!(q.dequeue(), Some("a"));
        assert_eq!(q.len(), 1);
        assert_eq!(q.dequeue(), Some("b"));
        assert_eq!(q.dequeue(), None);
        assert!(q.is_empty());
    }

    #[test]
    fn largest() {
        assert_eq!(largest_by_key(&["hi", "hello", "hey"], |w| w.len()), Some(&"hello"));
        assert_eq!(largest_by_key(&[1.5, 0.2], |x| -x), Some(&0.2));
        assert_eq!(largest_by_key::<i32, i32>(&[], |x| *x), None);
    }

    #[test]
    fn ring_overwrites_oldest() {
        let mut r: Ring<i32, 3> = Ring::new();
        for i in 1..=2 {
            r.push(i);
        }
        assert_eq!(r.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
        for i in 3..=5 {
            r.push(i);
        }
        assert_eq!(r.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5]);
    }
}
