fn main() {
    let n = 5;

    // ------------------------------------------------------------
    // `if` / `else`
    // ------------------------------------------------------------

    if n < 0 {
        println!("{n} is negative");
    } else if n > 0 {
        println!("{n} is positive");
    } else {
        println!("{n} is zero");
    }

    // ------------------------------------------------------------
    // `if` is an expression
    // ------------------------------------------------------------
    //
    // Because `if` is an expression, it evaluates to a value and can
    // be assigned directly to a variable.
    //
    // Both branches must return the same type.

    let big_n = if (-10..10).contains(&n) {
        println!("{n} is a small number, increasing it ten-fold.");

        // Returned from this branch.
        10 * n
    } else {
        println!("{n} is a big number, halving it.");

        // This branch must also return an `i32`.
        n / 2
    };

    println!("{n} -> {big_n}");
}
