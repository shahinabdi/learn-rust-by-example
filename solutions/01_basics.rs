// SOLUTION 1: Temperature converter
// Run: cargo run --example sol_01_basics   |   cargo test --example sol_01_basics

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    let temps = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!("{:>5} | {:>6}", "°C", "°F");
    for c in temps {
        println!("{:>5} | {:>6.1}", c, celsius_to_fahrenheit(c as f64));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_to_f() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn f_to_c() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }
}
