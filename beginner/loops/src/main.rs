fn main() {
    let mut count = 0;

    // ------------------------------------------------------------
    // `loop`
    // ------------------------------------------------------------
    //
    // An infinite loop that only stops when explicitly broken out of.
    //
    // This is the most basic form of looping and is useful when you
    // want something to repeat indefinitely until a condition is met.

    loop {
        count += 1;

        if count == 5 {
            // Exit the loop when `count` reaches 5.
            break;
        }
    }
    println!("loop reached {count}");

    // ------------------------------------------------------------
    // `while`
    // ------------------------------------------------------------
    //
    // A conditional loop that continues as long as the predicate is true.
    //
    // This is used when you want to repeat an action only while a certain
    // condition holds, and stop when it no longer does.

    count = 0;
    while count < 5 {
        count += 1;
    }
    println!("while reached {count}");

    // ------------------------------------------------------------
    // `for`
    // ------------------------------------------------------------
    //
    // A looping construct for iterating over a sequence.
    //
    // This is the preferred way to loop when you have a known number of
    // iterations or when you want to process each item in a collection.
    //
    // `(1..=5)` creates an inclusive range from 1 to 5.

    count = 0;
    for n in 1..=5 {
        count += n;
    }
    println!("for reached {count}");

    // ------------------------------------------------------------
    // `loop` with `break` and `continue`
    // ------------------------------------------------------------

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
