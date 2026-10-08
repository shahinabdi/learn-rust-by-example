// LESSON 24: Unsafe Rust and FFI
//
// THEORY
// - `unsafe` does NOT turn off the borrow checker. It unlocks exactly five
//   extra powers: dereference raw pointers, call unsafe functions, access
//   mutable statics, implement unsafe traits, access union fields.
// - You use it when YOU can prove an invariant the compiler cannot. The
//   standard pattern: wrap the small unsafe core in a SAFE API and document
//   the invariants (`// SAFETY:` comments).
// - FFI (`extern "C"`) calls C code or exposes Rust to C.
// Docs: https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html
//       https://doc.rust-lang.org/nomicon/

use std::slice;

// Raw pointers can be created in safe code; only dereferencing is unsafe.
fn raw_pointers() {
    let mut n = 10;
    let p1 = &n as *const i32;
    let p2 = &mut n as *mut i32;
    unsafe {
        *p2 += 5;
        println!("via raw pointers: {} {}", *p1, *p2);
    }
}

// A SAFE function built on unsafe internals: our own split_at_mut.
// Why unsafe? The borrow checker can't see that the two halves don't overlap.
fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len); // the invariant that makes the unsafe block sound
    // SAFETY: both ranges are inside the original slice and do not overlap.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Calling C functions from the C standard library
unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

// Mutable static: global state is unsafe because it's a data race hazard
static mut COUNTER: u32 = 0;

fn bump() -> u32 {
    // SAFETY: this demo is single threaded.
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

// Unsafe function: callers must uphold the documented contract.
/// # Safety
/// `index` must be less than `slice.len()`.
unsafe fn get_unchecked_demo(slice: &[i32], index: usize) -> i32 {
    // SAFETY: guaranteed by the caller.
    unsafe { *slice.get_unchecked(index) }
}

fn main() {
    raw_pointers();

    let mut v = [1, 2, 3, 4, 5];
    let (a, b) = my_split_at_mut(&mut v, 2);
    a[0] = 100;
    b[0] = 300;
    println!("{v:?}");

    // SAFETY: abs has no preconditions.
    println!("C abs(-7) = {}", unsafe { abs(-7) });

    bump();
    println!("counter = {}", bump());

    let data = [10, 20, 30];
    // SAFETY: index 1 < len 3.
    println!("{}", unsafe { get_unchecked_demo(&data, 1) });

    // Safe alternatives are usually better: Atomic types instead of static mut
    use std::sync::atomic::{AtomicU32, Ordering};
    static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("atomic = {}", SAFE_COUNTER.load(Ordering::SeqCst));

    // Memory layout facts
    println!(
        "sizes: i32={} &str={} Box<i32>={} Option<Box<i32>>={} (niche optimisation!)",
        std::mem::size_of::<i32>(),
        std::mem::size_of::<&str>(),
        std::mem::size_of::<Box<i32>>(),
        std::mem::size_of::<Option<Box<i32>>>()
    );
}

// ---------------------------------------------------------------------------
// CHALLENGE 24: Safe wrapper around unsafe
//  1) `fn swap_ends(values: &mut [i32])` that swaps first and last using raw
//     pointers (`std::ptr::swap`). Handle len < 2 safely, then compare with
//     the safe `values.swap(0, len - 1)` - why is the safe one preferred?
//  2) Write `fn first_and_last_mut(values: &mut [i32]) -> Option<(&mut i32,
//     &mut i32)>` using unsafe to hand out two mutable references. Document
//     the `// SAFETY:` reasoning (len >= 2 so indices differ).
//  3) Run `cargo miri test` (rustup +nightly component add miri) to detect
//     undefined behaviour if you want to go deeper.
// (See solutions/24_unsafe_ffi.rs)
// ---------------------------------------------------------------------------
