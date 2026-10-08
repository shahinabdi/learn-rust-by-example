// SOLUTION 21: Validated newtype, typestate connection, RAII guard

use std::marker::PhantomData;

// 1) Newtype with a private field: every Email is valid
#[derive(Debug, PartialEq)]
struct Email(String);

impl Email {
    fn parse(s: &str) -> Result<Email, String> {
        match s.split_once('@') {
            Some((user, domain)) if !user.is_empty() && !domain.is_empty() && !domain.contains('@') => {
                Ok(Email(s.to_string()))
            }
            _ => Err(format!("'{s}' is not a valid email")),
        }
    }
    fn as_str(&self) -> &str {
        &self.0
    }
}

// 2) Typestate
struct Disconnected;
struct Connected;
struct Authenticated {
    user: String,
}

struct Connection<S> {
    state: S,
}

impl Connection<Disconnected> {
    fn new() -> Self {
        Connection { state: Disconnected }
    }
    fn connect(self) -> Connection<Connected> {
        Connection { state: Connected }
    }
}

impl Connection<Connected> {
    fn login(self, user: &str) -> Connection<Authenticated> {
        Connection { state: Authenticated { user: user.to_string() } }
    }
}

impl Connection<Authenticated> {
    fn query(&self, sql: &str) -> String {
        format!("{} ran '{sql}'", self.state.user)
    }
}

// 3) RAII guard
struct Guard<'a> {
    log: &'a std::cell::RefCell<Vec<String>>,
}

impl<'a> Guard<'a> {
    fn new(log: &'a std::cell::RefCell<Vec<String>>) -> Guard<'a> {
        log.borrow_mut().push("lock acquired".into());
        Guard { log }
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.log.borrow_mut().push("lock released".into());
    }
}

fn risky(log: &std::cell::RefCell<Vec<String>>, fail: bool) -> Result<(), String> {
    let _guard = Guard::new(log);
    if fail {
        return Err("early return".into()); // guard still released
    }
    log.borrow_mut().push("work done".into());
    Ok(())
}

#[allow(dead_code)]
struct Unused(PhantomData<()>); // PhantomData appears in lesson 21; kept to show the import is intentional

fn main() {
    println!("{:?} {:?}", Email::parse("a@b.com").map(|e| e.as_str().to_string()), Email::parse("@x"));
    let conn = Connection::new().connect().login("ada");
    println!("{}", conn.query("SELECT 1"));
    // Connection::new().query("x"); // ERROR: no method `query` on Connection<Disconnected>

    let log = std::cell::RefCell::new(vec![]);
    let _ = risky(&log, true);
    let _ = risky(&log, false);
    println!("{:?}", log.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emails() {
        assert!(Email::parse("user@example.com").is_ok());
        assert!(Email::parse("no-at").is_err());
        assert!(Email::parse("a@@b").is_err());
        assert!(Email::parse("@b").is_err());
        assert!(Email::parse("a@").is_err());
    }

    #[test]
    fn guard_releases_on_early_return() {
        let log = std::cell::RefCell::new(vec![]);
        assert!(risky(&log, true).is_err());
        assert_eq!(*log.borrow(), vec!["lock acquired", "lock released"]);
    }

    #[test]
    fn typestate() {
        let c = Connection::new().connect().login("bob");
        assert_eq!(c.query("q"), "bob ran 'q'");
    }
}
