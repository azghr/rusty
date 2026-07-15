const MAX_POINTS: u32 = 100;

// error
// let mut MAX_POINTS: u32 = 200;

fn main() {
    // Let
    let number = 10;
    println!("number {number}");
    println!("number {}", number);

    // error
    // number = 20;
    // println!("number {number}");

    // number += 20;
    // println!("number {number}");

    let mut new_number = 10;
    println!("number {new_number}");
    new_number = 20;
    println!("number {new_number}");
    new_number += 20;
    println!("number {new_number}");

    // const
    const PI: f64 = 3.14159;
    println!("PI {PI}");

    // error
    // PI = 200;
    // println!("PI {PI}");

    println!("MAX_POINTS {MAX_POINTS}");
}
