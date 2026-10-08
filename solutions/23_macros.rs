// SOLUTION 23: Your own macros

macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = min!($($rest),+);
        if a < b { a } else { b }
    }};
}

macro_rules! square_all {
    ($($x:expr),* $(,)?) => { [ $( $x * $x ),* ] };
}

macro_rules! enum_str {
    (enum $name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $name {
            $( $variant ),*
        }

        impl $name {
            fn as_str(&self) -> &'static str {
                match self {
                    $( Self::$variant => stringify!($variant) ),*
                }
            }

            fn all() -> Vec<$name> {
                vec![ $( $name::$variant ),* ]
            }
        }
    };
}

enum_str!(enum Color { Red, Green, Blue });

fn main() {
    println!("{} {}", min!(3, 9, 4), min!(2.5, 1.5));
    println!("{:?}", square_all!(1, 2, 3));
    for c in Color::all() {
        print!("{} ", c.as_str());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_macro() {
        assert_eq!(min!(5), 5);
        assert_eq!(min!(3, 1, 2), 1);
        assert_eq!(min!(1.5, 0.5), 0.5);
    }

    #[test]
    fn square_macro() {
        assert_eq!(square_all!(2, 3, 4), [4, 9, 16]);
    }

    #[test]
    fn enum_macro() {
        assert_eq!(Color::Green.as_str(), "Green");
        assert_eq!(Color::all().len(), 3);
    }
}
