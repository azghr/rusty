fn main() {
    let user_input = "100";
    let converted: u32 = user_input.trim().parse().expect("Not a number");
    println!("The number is: {}", converted);
    println!("The number is: {}", converted + 100);

    // scalar data types
    let number: i8 = 10;
    let pi: f32 = 3.14;
    let turned_on: bool = false;
    let delta: char = 's';

    println!("the data types are : {:?}", (number, pi, turned_on, delta));

    // compound data types
    let coordinates: (f32, f32) = (1.5, 2.4);
    let people: [&str; 3] = ["John", "Alice", "Bob"];
    println!("coordinates: {:?}", coordinates);
    println!("people: {:?}", people);
}
