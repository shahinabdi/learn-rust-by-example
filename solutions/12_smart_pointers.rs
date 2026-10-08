// SOLUTION 12: Binary search tree + shared counter bonus

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Tree {
    Leaf,
    Node(Box<Tree>, i32, Box<Tree>),
}

impl Tree {
    fn insert(self, v: i32) -> Tree {
        match self {
            Tree::Leaf => Tree::Node(Box::new(Tree::Leaf), v, Box::new(Tree::Leaf)),
            Tree::Node(l, x, r) => {
                if v < x {
                    Tree::Node(Box::new(l.insert(v)), x, r)
                } else if v > x {
                    Tree::Node(l, x, Box::new(r.insert(v)))
                } else {
                    Tree::Node(l, x, r) // duplicate: ignore
                }
            }
        }
    }

    fn contains(&self, v: i32) -> bool {
        match self {
            Tree::Leaf => false,
            Tree::Node(l, x, r) => {
                if v == *x {
                    true
                } else if v < *x {
                    l.contains(v)
                } else {
                    r.contains(v)
                }
            }
        }
    }

    fn in_order(&self) -> Vec<i32> {
        match self {
            Tree::Leaf => vec![],
            Tree::Node(l, x, r) => {
                let mut out = l.in_order();
                out.push(*x);
                out.extend(r.in_order());
                out
            }
        }
    }
}

struct Worker {
    counter: Rc<RefCell<u32>>,
}

impl Worker {
    fn work(&self) {
        *self.counter.borrow_mut() += 1;
    }
}

fn main() {
    let tree = [5, 3, 8, 1, 4, 7, 9].into_iter().fold(Tree::Leaf, Tree::insert);
    println!("{:?} contains 4? {} contains 6? {}", tree.in_order(), tree.contains(4), tree.contains(6));

    let counter = Rc::new(RefCell::new(0));
    let a = Worker { counter: Rc::clone(&counter) };
    let b = Worker { counter: Rc::clone(&counter) };
    a.work();
    b.work();
    b.work();
    println!("total = {}", counter.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst() {
        let t = [5, 3, 8, 1, 3].into_iter().fold(Tree::Leaf, Tree::insert);
        assert_eq!(t.in_order(), vec![1, 3, 5, 8]);
        assert!(t.contains(8));
        assert!(!t.contains(2));
    }
}
