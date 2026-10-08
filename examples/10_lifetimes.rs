// LESSON 10: Lifetimes
//
// THEORY
// - Every reference has a lifetime: the region of code where it is valid.
//   Most are inferred. You annotate (`'a`) only when the compiler cannot tell
//   which input a returned reference came from.
// - Annotations do not change how long data lives; they describe the
//   relationship between references so dangling pointers are rejected.
// Docs: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// The result lives no longer than BOTH inputs.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Only `text` is returned, so only it needs to match the output lifetime.
fn first_sentence<'a>(text: &'a str, separator: &str) -> &'a str {
    text.split(separator).next().unwrap_or(text)
}

// A struct holding a reference needs a lifetime parameter
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, msg: &str) -> &str {
        // elision rule: &self's lifetime is used for the output
        println!("Attention: {msg}");
        self.part
    }
}

fn main() {
    let a = String::from("long string");
    let result;
    {
        let b = String::from("short");
        result = longest(&a, &b);
        println!("longest inside: {result}");
    }
    // println!("{result}"); // ERROR if uncommented: `b` does not live long enough

    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt { part: first_sentence(&novel, ".") };
    println!("{}", excerpt.announce("reading"));

    // 'static: lives for the entire program (all string literals)
    let s: &'static str = "I live forever";
    println!("{s}");
}

// ---------------------------------------------------------------------------
// CHALLENGE 10: Longest line
// Implement `longest_line<'a>(text: &'a str) -> Option<&'a str>` returning the
// longest line (first one on ties), or None for empty text.
// Then create `struct Parser<'a> { input: &'a str }` with a method
// `next_word(&mut self) -> Option<&'a str>` that returns successive words and
// advances `input`. Explain to yourself why the return type uses 'a, not
// the lifetime of &mut self.
// Run: cargo test --example 10_lifetimes
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn longest_line<'a>(text: &'a str) -> Option<&'a str> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest() {
        assert_eq!(longest_line("a\nbbb\ncc\nddd"), Some("bbb"));
        assert_eq!(longest_line(""), None);
    }
}
