// SOLUTION 9: Shapes with traits (+ bonus Display)

use std::f64::consts::PI;
use std::fmt;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}

struct Circle {
    r: f64,
}
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
    fn name(&self) -> String {
        "circle".into()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> String {
        "square".into()
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(r={})", self.r)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Square(side={})", self.side)
    }
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn biggest<'a>(shapes: &'a [Box<dyn Shape>]) -> Option<&'a Box<dyn Shape>> {
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 1.0 }), Box::new(Square { side: 3.0 })];
    println!("total = {:.2}", total_area(&shapes));
    if let Some(b) = biggest(&shapes) {
        println!("biggest = {} ({:.2})", b.name(), b.area());
    }
    println!("{} {}", Circle { r: 2.0 }, Square { side: 2.0 });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_and_biggest() {
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 1.0 }), Box::new(Square { side: 3.0 })];
        assert!((total_area(&shapes) - (PI + 9.0)).abs() < 1e-9);
        assert_eq!(biggest(&shapes).unwrap().name(), "square");
        assert!(biggest(&[]).is_none());
    }
}
