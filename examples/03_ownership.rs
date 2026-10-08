// LESSON 3: Ownership, stack vs heap
//
// THEORY
// - STACK: fast, fixed-size, last-in-first-out memory. Integers, bools, chars,
//   tuples/arrays of these live here. Copying them is cheap.
// - HEAP: flexible-size memory you allocate at runtime (String, Vec, Box).
//   The stack holds a small handle (pointer, length, capacity) to it.
// - Ownership rules: (1) each value has exactly one owner, (2) there is one
//   owner at a time, (3) when the owner goes out of scope the value is dropped
//   (heap memory freed automatically - no GC, no manual free).
// - Assigning a heap value MOVES ownership; the old variable becomes invalid.
//   Types that implement `Copy` (like i32) are copied instead.
// Docs: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn takes_ownership(s: String) {
    println!("took ownership of '{s}'");
} // `s` is dropped here

fn makes_copy(n: i32) {
    println!("copied {n}");
}

fn gives_back(s: String) -> String {
    s // returning moves ownership back to the caller
}

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    // Stack data: Copy
    let a = 5;
    let b = a; // copy; both usable
    println!("a={a} b={b}");
    makes_copy(a);
    println!("a is still usable: {a}");

    // Heap data: Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2
    // println!("{s1}"); // ERROR: borrow of moved value. Uncomment to see it!
    println!("s2 = {s2}");

    // Clone: explicit deep copy of the heap data
    let s3 = s2.clone();
    println!("s2={s2}, s3={s3}");

    // Moving into a function, and getting it back
    takes_ownership(s2);
    // println!("{s2}"); // ERROR: s2 was moved
    let s4 = gives_back(s3);
    println!("got back '{s4}'");

    // Drop order: reverse of creation, at end of scope
    let _first = Noisy("first");
    {
        let _inner = Noisy("inner");
        println!("leaving inner scope");
    }
    let _second = Noisy("second");
    println!("end of main");
}

// ---------------------------------------------------------------------------
// CHALLENGE 3: Ownership puzzle
// Implement `shout` which takes ownership of a String and returns it in
// upper case with "!" appended. Then in main, call it so that the ORIGINAL
// variable is still usable afterwards (hint: clone, or give-back pattern).
// Bonus: Which of these types are Copy? i32, String, (i32, i32), (i32, String)
// Verify by trying to assign them twice.
// Run: cargo test --example 03_ownership
// ---------------------------------------------------------------------------
#[allow(dead_code, unused_variables)]
fn shout(s: String) -> String {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouts() {
        assert_eq!(shout(String::from("hey")), "HEY!");
    }
}
