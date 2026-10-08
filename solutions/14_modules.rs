// SOLUTION 14: Library system (private fields, public API)

mod library {
    #[derive(Debug)]
    pub struct Book {
        pub title: String,
        available: bool,
    }

    impl Book {
        pub fn new(title: &str) -> Book {
            Book { title: title.to_string(), available: true }
        }
        pub fn is_available(&self) -> bool {
            self.available
        }
    }

    #[derive(Default)]
    pub struct Library {
        books: Vec<Book>,
    }

    impl Library {
        pub fn new() -> Library {
            Library::default()
        }

        pub fn add(&mut self, title: &str) {
            self.books.push(Book::new(title));
        }

        pub fn checkout(&mut self, title: &str) -> Result<(), String> {
            let book = self
                .books
                .iter_mut()
                .find(|b| b.title == title)
                .ok_or_else(|| format!("'{title}' is not in the library"))?;
            if !book.available {
                return Err(format!("'{title}' is already checked out"));
            }
            book.available = false;
            Ok(())
        }

        pub fn available_titles(&self) -> Vec<String> {
            self.books.iter().filter(|b| b.is_available()).map(|b| b.title.clone()).collect()
        }
    }
}

use library::Library;

fn main() {
    let mut lib = Library::new();
    lib.add("Dune");
    lib.add("Emma");
    println!("{:?}", lib.checkout("Dune"));
    println!("{:?}", lib.checkout("Dune"));
    println!("{:?}", lib.checkout("Nope"));
    println!("{:?}", lib.available_titles());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkout_flow() {
        let mut lib = Library::new();
        lib.add("Dune");
        assert!(lib.checkout("Dune").is_ok());
        assert!(lib.checkout("Dune").is_err());
        assert!(lib.checkout("Missing").is_err());
        assert!(lib.available_titles().is_empty());
    }
}
