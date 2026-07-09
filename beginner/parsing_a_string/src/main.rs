use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Circle {
            radius: s.trim().parse()?,
        })
    }
}

fn main() -> Result<(), ParseIntError> {
    // ------------------------------------------------------------------
    // Basic string parsing
    // ------------------------------------------------------------------

    // Type inferred from the variable.
    let parsed: i32 = "5".parse().unwrap();

    // Explicit type using turbofish syntax.
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("parsed       = {}", parsed);
    println!("turbo_parsed = {}", turbo_parsed);
    println!("sum          = {}", sum);

    // ------------------------------------------------------------------
    // Parsing into a custom type
    // ------------------------------------------------------------------

    let radius = "    3 ";
    let circle: Circle = radius.parse()?;

    println!("circle = {:?}", circle);
    println!("radius = {}", circle.radius);

    Ok(())
}
