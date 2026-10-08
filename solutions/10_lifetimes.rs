// SOLUTION 10: Longest line + Parser<'a>

fn longest_line<'a>(text: &'a str) -> Option<&'a str> {
    let mut best: Option<&str> = None;
    for line in text.lines() {
        // strict > keeps the FIRST line on ties
        if best.map_or(true, |b| line.len() > b.len()) {
            best = Some(line);
        }
    }
    best
}

struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    // Returning &'a str (tied to the original text) instead of the lifetime of
    // &mut self means the words stay usable after the next call to next_word.
    fn next_word(&mut self) -> Option<&'a str> {
        let trimmed = self.input.trim_start();
        if trimmed.is_empty() {
            self.input = trimmed;
            return None;
        }
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let (word, rest) = trimmed.split_at(end);
        self.input = rest;
        Some(word)
    }
}

fn main() {
    println!("{:?}", longest_line("a\nbbb\ncc\nddd"));
    let mut p = Parser { input: "  hello big   world " };
    let first = p.next_word();
    let second = p.next_word(); // `first` is still valid thanks to 'a
    println!("{first:?} {second:?} {:?} {:?}", p.next_word(), p.next_word());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest() {
        assert_eq!(longest_line("a\nbbb\ncc\nddd"), Some("bbb"));
        assert_eq!(longest_line(""), None);
    }

    #[test]
    fn parser() {
        let mut p = Parser { input: " one  two " };
        assert_eq!(p.next_word(), Some("one"));
        assert_eq!(p.next_word(), Some("two"));
        assert_eq!(p.next_word(), None);
    }
}
