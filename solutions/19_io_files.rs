// SOLUTION 19: Mini word count (wc)
// Try: cargo run --example sol_19_io_files -- Cargo.toml
//      or pipe stdin:  Get-Content Cargo.toml | cargo run --example sol_19_io_files

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

fn count<R: BufRead>(mut reader: R) -> io::Result<Counts> {
    let mut counts = Counts::default();
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        counts.lines += 1;
        counts.words += line.split_whitespace().count();
        counts.bytes += n;
    }
    Ok(counts)
}

fn main() -> io::Result<()> {
    let counts = match std::env::args().nth(1) {
        Some(path) => count(BufReader::new(File::open(path)?))?,
        None => count(io::stdin().lock())?,
    };
    println!("{} {} {}", counts.lines, counts.words, counts.bytes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn counts_text() {
        let c = count(Cursor::new("hello world\nsecond line here\n")).unwrap();
        assert_eq!(c, Counts { lines: 2, words: 5, bytes: 29 });
    }

    #[test]
    fn empty_input() {
        assert_eq!(count(Cursor::new("")).unwrap(), Counts::default());
    }

    #[test]
    fn no_trailing_newline() {
        let c = count(Cursor::new("a b")).unwrap();
        assert_eq!(c, Counts { lines: 1, words: 2, bytes: 3 });
    }
}
