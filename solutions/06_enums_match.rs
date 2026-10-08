// SOLUTION 6: Traffic light state machine

#[derive(Debug, Clone, Copy, PartialEq)]
enum Light {
    Red,
    Green,
    Yellow,
    Flashing(u8), // bonus: carries the blink count
}

impl Light {
    fn next(&self) -> Light {
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
            Light::Flashing(0) => Light::Red,
            Light::Flashing(n) => Light::Flashing(n - 1),
        }
    }

    fn seconds(&self) -> u32 {
        match self {
            Light::Red => 30,
            Light::Green => 25,
            Light::Yellow => 5,
            Light::Flashing(_) => 1,
        }
    }
}

fn main() {
    let mut light = Light::Red;
    for _ in 0..4 {
        println!("{light:?} for {}s", light.seconds());
        light = light.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        assert_eq!(Light::Red.next(), Light::Green);
        assert_eq!(Light::Green.next(), Light::Yellow);
        assert_eq!(Light::Yellow.next(), Light::Red);
    }

    #[test]
    fn durations() {
        assert_eq!(Light::Red.seconds(), 30);
        assert_eq!(Light::Green.seconds(), 25);
        assert_eq!(Light::Yellow.seconds(), 5);
    }

    #[test]
    fn flashing_counts_down() {
        assert_eq!(Light::Flashing(2).next(), Light::Flashing(1));
        assert_eq!(Light::Flashing(0).next(), Light::Red);
    }
}
