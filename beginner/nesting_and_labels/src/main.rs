#![allow(unreachable_code, unused_labels)]
fn main() {
    let mut row = 1;

    'outer: loop {
        println!("\nRow {row}");

        let mut col = 1;

        'inner: loop {
            println!("  Checking cell ({row}, {col})");

            // Skip the rest of this row and continue
            // with the next iteration of the outer loop.
            if col == 2 && row == 1 {
                println!("  -> Skip to the next row");
                row += 1;
                continue 'outer;
            }

            // Exit both loops completely.
            if row == 2 && col == 2 {
                println!("  -> Target found. Exiting all loops.");
                break 'outer;
            }

            // Exit only the inner loop.
            if col == 3 {
                println!("  -> End of current row");
                break;
            }

            col += 1;
        }

        row += 1;
    }

    println!("\nDone.");
}
