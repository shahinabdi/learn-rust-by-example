// LESSON 19: Input/output, files and command-line arguments
//
// THEORY
// - I/O is fallible, so almost every function returns `io::Result<T>`; combine
//   with `?`. Wrap readers in `BufReader` / writers in `BufWriter` for speed.
// - `Read`/`Write`/`BufRead` are TRAITS: the same code works for files, stdin,
//   network sockets and in-memory buffers (great for testing!).
// - `std::env::args()` gives command-line arguments; `Path`/`PathBuf` are the
//   portable way to handle file paths.
// Docs: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
//       https://doc.rust-lang.org/std/io/index.html

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Write};
use std::path::PathBuf;

// Generic over ANY reader: works with files, stdin, or a byte slice in tests
fn count_lines<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut n = 0;
    for line in reader.lines() {
        line?; // propagate read errors
        n += 1;
    }
    Ok(n)
}

fn write_report<W: Write>(mut out: W, rows: &[(&str, u32)]) -> io::Result<()> {
    for (name, score) in rows {
        writeln!(out, "{name:<8}{score:>4}")?;
    }
    out.flush()
}

fn main() -> io::Result<()> {
    // Command-line arguments (try: cargo run --example 19_io_files -- hello world)
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("args = {args:?}");

    // Work in a temp folder so we never touch your real files
    let dir: PathBuf = std::env::temp_dir().join("rust_tutorial_io");
    fs::create_dir_all(&dir)?;
    let path = dir.join("notes.txt");

    // Write a whole file at once
    fs::write(&path, "first line\nsecond line\nthird line\n")?;

    // Read a whole file at once
    let content = fs::read_to_string(&path)?;
    println!("{} bytes, {} lines", content.len(), content.lines().count());

    // Stream it line by line
    let reader = BufReader::new(File::open(&path)?);
    for (i, line) in reader.lines().enumerate() {
        println!("{:>2}: {}", i + 1, line?);
    }

    // Append with OpenOptions
    let mut f = fs::OpenOptions::new().append(true).open(&path)?;
    writeln!(f, "appended line")?;

    // Buffered writing
    let report_path = dir.join("report.txt");
    write_report(BufWriter::new(File::create(&report_path)?), &[("ada", 95), ("linus", 88)])?;
    print!("{}", fs::read_to_string(&report_path)?);

    // Same functions on in-memory data - no files needed
    println!("lines in memory = {}", count_lines(Cursor::new("a\nb\nc"))?);
    let mut buf = Vec::new();
    write_report(&mut buf, &[("x", 1)])?;
    println!("{:?}", String::from_utf8_lossy(&buf));

    // Handling a missing file gracefully
    match File::open(dir.join("missing.txt")) {
        Ok(_) => println!("exists?!"),
        Err(e) if e.kind() == io::ErrorKind::NotFound => println!("missing.txt not found (expected)"),
        Err(e) => return Err(e),
    }

    // Paths
    println!("{} {:?} {:?}", path.display(), path.file_name(), path.extension());

    // Reading from stdin (non-blocking demo: only if input is piped)
    // let mut line = String::new();
    // io::stdin().read_line(&mut line)?;
    let mut s = String::new();
    Cursor::new("simulated stdin\n").read_to_string(&mut s)?;
    print!("{s}");

    fs::remove_dir_all(&dir)?; // clean up
    Ok(())
}

// ---------------------------------------------------------------------------
// CHALLENGE 19: Mini word-count (`wc`)
// Implement `count<R: BufRead>(reader: R) -> io::Result<Counts>` where
// `Counts { lines, words, bytes }`. Then write `fn main` so that
//   cargo run --example 19_io_files -- some_file.txt
// prints "lines words bytes" for a file, or reads stdin when no argument is
// given. Test it with Cursor, as in the lesson.
// (See solutions/19_io_files.rs)
// ---------------------------------------------------------------------------
