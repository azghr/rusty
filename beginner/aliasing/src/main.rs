// Type aliases create alternative names for existing types.
// They do not create new, distinct types.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // All of these aliases are equivalent to `u64`.
    let nanoseconds: NanoSecond = 5u64;
    let inches: Inch = 2 as U64;

    // Since type aliases are only alternative names, they can be used
    // interchangeably without requiring explicit conversions.
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
