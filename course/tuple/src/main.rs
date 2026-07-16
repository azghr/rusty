fn main() {
    // let data: (i32, f64, char, bool, &str) = (10, 5.4, 'a', true, "hello");

    let data = (10, 5.4, 'a', true, "hello");
    println!("data={:?}", data);

    // destructuring
    let (num, floating_point, character, boolean, string) = data;
    println!("{}", num);
    println!("{}", floating_point);
    println!("{}", character);
    println!("{}", boolean);
    println!("{}", string);

    // Accessing elements directly
    println!("Element at index 0: {}", data.0);
    println!("Element at index 1: {}", data.1);
    println!("Element at index 2: {}", data.2);
    println!("Element at index 3: {}", data.3);
    println!("Element at index 4: {}", data.4);

    // destructuring with ignoring elements
    let (_, _, _, _, string) = data;
    println!("string: {}", string);

    // empty tuple
    let empty = ();
    println!("{:?}", empty);
}
