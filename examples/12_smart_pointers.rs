// LESSON 12: Smart pointers - Box, Rc, RefCell
//
// THEORY
// - Box<T>: puts a value on the HEAP with a single owner. Needed for recursive
//   types (whose size would otherwise be infinite) and trait objects.
// - Rc<T>: reference-counted shared OWNERSHIP (single thread). Clone = bump the
//   counter; the value is freed when the last Rc is dropped.
// - RefCell<T>: interior mutability - borrow rules checked at RUNTIME instead
//   of compile time (panics on violation). `Rc<RefCell<T>>` = shared + mutable.
// Docs: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

use std::cell::RefCell;
use std::rc::Rc;

// Recursive type: a singly linked list
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

impl List {
    fn sum(&self) -> i32 {
        match self {
            Cons(v, rest) => v + rest.sum(),
            Nil => 0,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    // Box: heap allocation
    let boxed = Box::new(5);
    println!("boxed = {boxed}, doubled = {}", *boxed * 2); // `*` dereferences

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?} sum={}", list.sum());

    // Rc: shared ownership
    let shared = Rc::new(String::from("shared data"));
    println!("count after create = {}", Rc::strong_count(&shared));
    let a = Rc::clone(&shared);
    {
        let _b = Rc::clone(&shared);
        println!("count with 3 owners = {}", Rc::strong_count(&shared));
    }
    println!("count after inner scope = {} ({a})", Rc::strong_count(&shared));

    // RefCell: mutate through a shared reference
    let cell = RefCell::new(vec![1, 2]);
    cell.borrow_mut().push(3);
    println!("{:?}", cell.borrow());
    // Two borrow_mut() alive at once would panic at runtime:
    // let _x = cell.borrow_mut(); let _y = cell.borrow_mut();

    // Rc<RefCell<T>>: multiple owners that can all mutate
    let leaf = Rc::new(RefCell::new(Node { value: 3, children: vec![] }));
    let root = Node { value: 10, children: vec![Rc::clone(&leaf)] };
    leaf.borrow_mut().value += 100;
    println!("root {} -> child value = {}", root.value, root.children[0].borrow().value);
}

// ---------------------------------------------------------------------------
// CHALLENGE 12: Binary search tree
// Build `enum Tree { Leaf, Node(Box<Tree>, i32, Box<Tree>) }` with:
//   - `insert(self, v: i32) -> Tree`
//   - `contains(&self, v: i32) -> bool`
//   - `in_order(&self) -> Vec<i32>` (sorted output)
// Bonus: make a shared counter `Rc<RefCell<u32>>` that two structs both
// increment, then print the final total.
// ---------------------------------------------------------------------------
