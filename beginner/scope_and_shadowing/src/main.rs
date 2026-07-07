fn main() {
    println!("=== Variable Scope ===");

    // This variable lives for the entire duration of `main`.
    let outer_value = 1;

    // Inner block with its own scope.
    {
        let inner_value = 2;

        println!("Inner value: {}", inner_value);
    } // `inner_value` goes out of scope here.

    // Error! `inner_value` is no longer in scope.
    // println!("Inner value: {}", inner_value);

    println!("Outer value: {}", outer_value);

    println!("\n=== Variable Shadowing ===");

    let value = 1;

    {
        println!("Before shadowing: {}", value);

        // Shadows the outer `value`.
        let value = "abc";

        println!("Shadowed inside inner block: {}", value);
    }

    // The outer `value` is visible again.
    println!("Outside inner block: {}", value);

    // Shadow the outer integer with a new integer.
    let value = 2;

    println!("Shadowed in outer block: {}", value);
}
