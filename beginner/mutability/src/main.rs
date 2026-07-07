fn main() {
    // Mutable variable
    let mut counter = 5;
    println!("The value of counter is: {counter}");

    counter = 6;
    println!("The value of counter is: {counter}");

    // Immutable variable
    let number = 5;
    println!("The value of number is: {number}");

    // Shadowing (re-binding)
    let number = number + 1;
    println!("After first shadowing: {number}");

    let number = number * 2;
    println!("After second shadowing: {number}");

    // `mut` changes the same variable.
    // Shadowing creates a new variable with the same name.

    let _unused_value = 1;
    let mut score = 1;

    println!("Score before update: {}", score);

    score += 1;

    println!("Score after update: {}", score);

    // Uncommenting the line below will cause a compilation error
    // because `unused_value` is immutable.
    // _unused_value += 1;
}
