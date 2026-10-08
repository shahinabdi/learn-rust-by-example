// SOLUTION 24: Safe wrappers around unsafe

fn swap_ends(values: &mut [i32]) {
    let len = values.len();
    if len < 2 {
        return;
    }
    let ptr = values.as_mut_ptr();
    // SAFETY: len >= 2, so index 0 and len-1 are in bounds and distinct.
    unsafe {
        std::ptr::swap(ptr, ptr.add(len - 1));
    }
}
// The safe `values.swap(0, len - 1)` does the same with bounds checking and no
// `unsafe`, so it is preferred. Unsafe only pays off when safe code can't
// express the idea (like two &mut into one slice).

fn first_and_last_mut(values: &mut [i32]) -> Option<(&mut i32, &mut i32)> {
    let len = values.len();
    if len < 2 {
        return None;
    }
    let ptr = values.as_mut_ptr();
    // SAFETY: len >= 2 so indices 0 and len-1 differ: the references never alias,
    // and both point inside the borrowed slice for its full lifetime.
    unsafe { Some((&mut *ptr, &mut *ptr.add(len - 1))) }
}

fn main() {
    let mut v = [1, 2, 3, 4];
    swap_ends(&mut v);
    println!("{v:?}");
    if let Some((a, b)) = first_and_last_mut(&mut v) {
        *a += 100;
        *b += 200;
    }
    println!("{v:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps() {
        let mut v = [1, 2, 3];
        swap_ends(&mut v);
        assert_eq!(v, [3, 2, 1]);
        let mut one = [9];
        swap_ends(&mut one);
        assert_eq!(one, [9]);
        swap_ends(&mut []);
    }

    #[test]
    fn two_mut_refs() {
        let mut v = [1, 2, 3];
        let (a, b) = first_and_last_mut(&mut v).unwrap();
        std::mem::swap(a, b);
        assert_eq!(v, [3, 2, 1]);
        assert!(first_and_last_mut(&mut [1]).is_none());
    }
}
