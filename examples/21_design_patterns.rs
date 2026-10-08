// LESSON 21: Idiomatic design patterns - newtype, builder, typestate, RAII
//
// THEORY
// - Rust encodes rules in TYPES so invalid states cannot compile ("make
//   illegal states unrepresentable").
// - Newtype: wrap a primitive in a tuple struct (`struct UserId(u64)`) so a
//   `UserId` can't be mixed up with an `OrderId`. Zero runtime cost.
// - Builder: construct complex objects step by step with chained methods.
// - Typestate: different states are different TYPES, so calling a method in
//   the wrong state is a compile error.
// - RAII: acquiring a resource = creating a value; `Drop` releases it, even on
//   early return or panic. That's how `File`, `MutexGuard`, `Rc` work.
// Docs: https://rust-unofficial.github.io/patterns/

use std::fmt;

// ---------- Newtype ----------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);
#[derive(Debug, Clone, Copy, PartialEq)]
struct Feet(f64);

impl From<Feet> for Meters {
    fn from(f: Feet) -> Meters {
        Meters(f.0 * 0.3048)
    }
}

fn run_track(length: Meters) -> String {
    format!("track of {:.1} m", length.0)
}

// ---------- Builder ----------
#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    timeout_secs: u32,
}

struct RequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    timeout_secs: u32,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        RequestBuilder { url: url.into(), method: "GET".into(), headers: vec![], timeout_secs: 30 }
    }
    fn method(mut self, m: &str) -> Self {
        self.method = m.into();
        self
    }
    fn header(mut self, k: &str, v: &str) -> Self {
        self.headers.push((k.into(), v.into()));
        self
    }
    fn timeout(mut self, secs: u32) -> Self {
        self.timeout_secs = secs;
        self
    }
    fn build(self) -> Result<Request, String> {
        if !self.url.starts_with("http") {
            return Err(format!("invalid url: {}", self.url));
        }
        Ok(Request { url: self.url, method: self.method, headers: self.headers, timeout_secs: self.timeout_secs })
    }
}

// ---------- Typestate ----------
struct Open;
struct Closed;

struct Door<State> {
    name: String,
    _state: std::marker::PhantomData<State>,
}

impl Door<Closed> {
    fn new(name: &str) -> Self {
        Door { name: name.into(), _state: std::marker::PhantomData }
    }
    fn open(self) -> Door<Open> {
        println!("{} opens", self.name);
        Door { name: self.name, _state: std::marker::PhantomData }
    }
}

impl Door<Open> {
    fn close(self) -> Door<Closed> {
        println!("{} closes", self.name);
        Door { name: self.name, _state: std::marker::PhantomData }
    }
    fn walk_through(&self) {
        println!("walking through {}", self.name);
    }
}

// ---------- RAII guard ----------
struct Timer {
    label: &'static str,
    start: std::time::Instant,
}

impl Timer {
    fn start(label: &'static str) -> Timer {
        Timer { label, start: std::time::Instant::now() }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("[{}] finished after {:?}", self.label, self.start.elapsed());
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ({} headers, {}s)", self.method, self.url, self.headers.len(), self.timeout_secs)
    }
}

fn main() {
    // Newtype: the compiler stops unit mix-ups
    let track = run_track(Feet(100.0).into());
    println!("{track}");
    // run_track(Feet(100.0)); // ERROR: expected Meters, found Feet

    let req = RequestBuilder::new("https://example.com")
        .method("POST")
        .header("Accept", "json")
        .timeout(5)
        .build();
    match &req {
        Ok(r) => println!("{r}"),
        Err(e) => println!("{e}"),
    }
    println!("{:?}", RequestBuilder::new("ftp://nope").build().map(|r| r.method));

    let door = Door::new("front door");
    // door.walk_through(); // ERROR: no method on Door<Closed>
    let door = door.open();
    door.walk_through();
    let _door = door.close();

    {
        let _t = Timer::start("work");
        println!("doing work...");
    } // Timer dropped here automatically
}

// ---------------------------------------------------------------------------
// CHALLENGE 21: Typestate download + validated newtype
//  1) `struct Email(String)` with `Email::parse(&str) -> Result<Email, String>`
//     (must contain exactly one '@' with text on both sides). Since the field
//     is private, an `Email` is ALWAYS valid.
//  2) A typestate `Connection<Disconnected | Connected | Authenticated>`:
//     `connect() -> Connection<Connected>`, `login(user) ->
//     Connection<Authenticated>`, and `query()` only exists when
//     Authenticated.
//  3) A `Guard` that prints "lock acquired" in `new()` and "lock released" in
//     Drop; show that it releases even when a function returns early.
// (See solutions/21_design_patterns.rs)
// ---------------------------------------------------------------------------
