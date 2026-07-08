use std::fmt;
use std::str::FromStr;

// A type that can be converted to a `String`.
struct Circle {
    radius: u32,
}

// Implement `Display` for `Circle`.
// This automatically provides the `ToString` trait.
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}

// A type that can be parsed from a string.
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Parse a string in the form "x,y".
impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("expected format: x,y")?;

        Ok(Point {
            x: x.trim().parse().map_err(|_| "invalid x value")?,
            y: y.trim().parse().map_err(|_| "invalid y value")?,
        })
    }
}

fn main() {
    // `Display` automatically provides `ToString`.
    let circle = Circle { radius: 6 };
    let circle_string = circle.to_string();

    println!("{}", circle);
    println!("{}", circle_string);

    // Parse strings into numbers.
    let parsed: i32 = "10".parse().unwrap();
    let turbo_parsed = "20".parse::<i32>().unwrap();

    println!("Sum: {}", parsed + turbo_parsed);

    // Parse a string into a custom type.
    let point: Point = "3, 7".parse().unwrap();

    println!("{:?}", point);

    assert_eq!(point, Point { x: 3, y: 7 });
}
