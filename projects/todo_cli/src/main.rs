use std::path::Path;
use std::process;
use todo_cli::{TodoError, TodoList};

const FILE: &str = "todos.json";

fn run(args: &[String]) -> Result<(), TodoError> {
    let path = Path::new(FILE);
    let mut list = TodoList::load(path)?;
    let parse_id = |s: Option<&String>| s.and_then(|s| s.parse::<u32>().ok());

    match args.first().map(String::as_str) {
        Some("add") => {
            let id = list.add(&args[1..].join(" "))?;
            println!("added #{id}");
        }
        Some("done") => {
            let id = parse_id(args.get(1)).ok_or(TodoError::EmptyTitle)?;
            list.complete(id)?;
            println!("completed #{id}");
        }
        Some("rm") => {
            let id = parse_id(args.get(1)).ok_or(TodoError::EmptyTitle)?;
            println!("removed '{}'", list.remove(id)?.title);
        }
        Some("list") | None => {
            for t in list.tasks() {
                println!("[{}] {:>3}  {}", if t.done { 'x' } else { ' ' }, t.id, t.title);
            }
            return Ok(());
        }
        Some(other) => {
            eprintln!("unknown command '{other}'. usage: todo [add <title> | done <id> | rm <id> | list]");
            process::exit(2);
        }
    }
    list.save(path)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
