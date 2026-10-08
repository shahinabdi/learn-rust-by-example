use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(std::env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("error: {err}\nusage: minigrep <query> <file> [-i] [-n]");
        process::exit(2);
    });

    match minigrep::run(&config) {
        Ok(lines) => {
            for l in &lines {
                println!("{l}");
            }
            if lines.is_empty() {
                process::exit(1); // like grep: 1 = no matches
            }
        }
        Err(e) => {
            eprintln!("error reading '{}': {e}", config.path);
            process::exit(2);
        }
    }
}
