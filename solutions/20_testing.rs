// SOLUTION 20: Fixed leap-year + TDD clamp_percent

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn clamp_percent(n: i32) -> u8 {
    n.clamp(0, 100) as u8
}

fn main() {
    for y in [1996, 1900, 2000, 2023, 2024] {
        println!("{y}: {}", is_leap_year(y));
    }
    println!("{} {} {}", clamp_percent(-5), clamp_percent(50), clamp_percent(500));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        let cases = [(1600, true), (1700, false), (1900, false), (2000, true), (2004, true), (2023, false), (2100, false)];
        for (year, expected) in cases {
            assert_eq!(is_leap_year(year), expected, "year {year}");
        }
    }

    // Written BEFORE clamp_percent existed (TDD)
    #[test]
    fn clamps() {
        assert_eq!(clamp_percent(-1), 0);
        assert_eq!(clamp_percent(0), 0);
        assert_eq!(clamp_percent(100), 100);
        assert_eq!(clamp_percent(101), 100);
        assert_eq!(clamp_percent(i32::MIN), 0);
        assert_eq!(clamp_percent(i32::MAX), 100);
    }
}
