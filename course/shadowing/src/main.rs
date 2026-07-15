fn main() {
    let n = 5;

    let n = n + 1;

    {
        let n = n * 2;
        println!("inner {n}");
    }

    println!("outer {n}");

    // shadowing
    let spaces = "      ";
    let spaces = spaces.len(); // shadowing

    println!("spaces {spaces}");

    {
        let spaces = 10;
        println!("inner {spaces}");
    }

    println!("outer {spaces}");
}
