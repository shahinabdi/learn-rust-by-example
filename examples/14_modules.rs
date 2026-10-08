// LESSON 14: Modules and visibility
//
// THEORY
// - Modules (`mod`) organise code into namespaces. Everything is PRIVATE to its
//   module by default; `pub` exposes it, `pub(crate)` exposes it crate-wide,
//   `pub(super)` only to the parent.
// - `use` brings paths into scope; `pub use` re-exports them to build a clean
//   public API. In real projects each module is a file (`src/shop.rs`) or a
//   folder (`src/shop/mod.rs`) - here they are inline so one file runs.
// - A crate is a compilation unit (binary or library); a package (Cargo.toml)
//   holds one or more crates; a workspace holds many packages.
// Docs: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

mod shop {
    pub mod inventory {
        #[derive(Debug)]
        pub struct Item {
            pub name: String,    // public field
            price_cents: u32,    // private field: only this module can touch it
        }

        impl Item {
            pub fn new(name: &str, price_cents: u32) -> Item {
                Item { name: name.to_string(), price_cents }
            }

            pub fn price(&self) -> f64 {
                self.price_cents as f64 / 100.0
            }

            // visible anywhere in this crate, but not to outside users
            pub(crate) fn raw_price(&self) -> u32 {
                self.price_cents
            }
        }

        // private helper: callable only inside `inventory` (and its children)
        fn _internal_check() {}
    }

    pub mod checkout {
        use super::inventory::Item; // `super` = parent module

        pub fn total(items: &[Item]) -> u32 {
            items.iter().map(|i| i.raw_price()).sum()
        }
    }

    // Re-export: users write `shop::Item` instead of `shop::inventory::Item`
    pub use self::inventory::Item;
}

use shop::checkout;
use shop::Item;

mod math {
    pub const TAU: f64 = std::f64::consts::TAU;

    pub fn double(x: i32) -> i32 {
        x * 2
    }

    #[cfg(test)]
    mod tests {
        // child modules can use private items of their parents
        use super::*;
        #[test]
        fn doubles() {
            assert_eq!(double(2), 4);
        }
    }
}

fn main() {
    let items = vec![Item::new("pen", 150), Item::new("book", 1299)];
    println!("{} costs {:.2}", items[0].name, items[0].price());
    println!("total cents = {}", checkout::total(&items));
    // println!("{}", items[0].price_cents); // ERROR: field `price_cents` is private
    println!("{} {}", math::double(21), math::TAU);

    // Glob and nested imports
    use std::collections::{BTreeMap, HashSet};
    let _s: HashSet<i32> = HashSet::new();
    let _m: BTreeMap<i32, i32> = BTreeMap::new();
}

// ---------------------------------------------------------------------------
// CHALLENGE 14: Library system
// Create `mod library` containing:
//   - `pub struct Book { pub title: String, available: bool }` (note the
//     private field!) with `Book::new(title)`, `is_available()`.
//   - `pub struct Library { books: Vec<Book> }` with `new()`, `add(title)`,
//     `checkout(title) -> Result<(), String>` (error when missing or already
//     out) and `available_titles() -> Vec<String>`.
// Keep `available` and `books` private so the only way to change state is via
// your methods. Write tests for it. (See solutions/14_modules.rs)
// ---------------------------------------------------------------------------
