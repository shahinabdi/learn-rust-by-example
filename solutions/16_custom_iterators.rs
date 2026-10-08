// SOLUTION 16: Iterator workshop

struct Collatz(u64);

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.0 == 0 {
            return None; // finished (0 is our "done" marker)
        }
        let current = self.0;
        self.0 = match current {
            1 => 0,
            n if n % 2 == 0 => n / 2,
            n => 3 * n + 1,
        };
        Some(current)
    }
}

struct Primes {
    found: Vec<u64>,
    candidate: u64,
}

impl Primes {
    fn new() -> Primes {
        Primes { found: vec![], candidate: 2 }
    }
}

impl Iterator for Primes {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            let c = self.candidate;
            self.candidate += 1;
            // only test against known primes up to sqrt(c)
            if self.found.iter().take_while(|&&p| p * p <= c).all(|&p| c % p != 0) {
                self.found.push(c);
                return Some(c);
            }
        }
    }
}

struct Grid {
    w: usize,
    h: usize,
}

impl Grid {
    fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
        let w = self.w;
        (0..self.h).flat_map(move |y| (0..w).map(move |x| (x, y)))
    }
}

fn main() {
    println!("{:?}", Collatz(6).collect::<Vec<_>>());
    println!("{:?}", Primes::new().take(10).collect::<Vec<_>>());
    println!("{:?}", Grid { w: 2, h: 2 }.cells().collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz() {
        assert_eq!(Collatz(6).collect::<Vec<_>>(), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(Collatz(1).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn primes() {
        assert_eq!(Primes::new().take(8).collect::<Vec<_>>(), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(Primes::new().nth(99), Some(541));
    }

    #[test]
    fn grid() {
        let cells: Vec<_> = Grid { w: 2, h: 2 }.cells().collect();
        assert_eq!(cells, vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
        assert_eq!(Grid { w: 0, h: 5 }.cells().count(), 0);
    }
}
