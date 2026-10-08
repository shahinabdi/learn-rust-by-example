// SOLUTION 3: Ownership puzzle

fn shout(s: String) -> String {
    let mut out = s.to_uppercase();
    out.push('!');
    out
}

fn main() {
    let original = String::from("hey");

    // Option A: clone - the callee gets its own copy, `original` stays valid
    let loud = shout(original.clone());
    println!("{original} -> {loud}");

    // Option B: give-back pattern - move in, move the result out
    let again = shout(original);
    println!("{again}");
    // `original` is moved now; using it here would not compile.

    // Bonus: Copy or not?
    let a = 1;
    let _b = a;
    let _a2 = a; // i32: Copy
    let t = (1, 2);
    let _t1 = t;
    let _t2 = t; // (i32, i32): Copy (all fields are Copy)
    let s = String::from("x");
    let _s1 = s;
    // let _s2 = s;     // String: NOT Copy
    let u = (1, String::from("y"));
    let _u1 = u;
    // let _u2 = u;     // (i32, String): NOT Copy
    println!("Copy: i32, (i32, i32). Move: String, (i32, String).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouts() {
        assert_eq!(shout(String::from("hey")), "HEY!");
    }
}
