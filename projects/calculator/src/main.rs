use std::io::{self, BufRead, Write};

fn main() {
    println!("Calculator - type an expression, or 'quit'.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break;
        }
        let line = line.trim();
        if line == "quit" {
            break;
        }
        if line.is_empty() {
            continue;
        }
        match calculator::calculate(line) {
            Ok(v) => println!("{v}"),
            Err(e) => println!("error: {e}"),
        }
    }
}
