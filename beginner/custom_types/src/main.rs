// A compile-time constant.
// `const` values must have an explicit type and are inlined
// wherever they are used.
const PI: f64 = 3.1415926535;

// A static variable.
// `static` values have a fixed memory location and live for
// the entire duration of the program.
static APP_NAME: &str = "Rust Custom Types Demo";

// `derive(Debug)` automatically implements the `Debug` trait,
// allowing us to print the struct using `{:?}`.
#[derive(Debug)]
struct Person {
    // The person's name.
    name: String,

    // The person's age.
    age: u8,
}

// `impl` is used to define methods associated with a struct.
impl Person {
    // `&self` means this method borrows the current object
    // without taking ownership.
    fn introduce(&self) {
        println!(
            "Hello! My name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

// An enum allows a value to be one of several possible variants.
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Match on the enum and perform a different action for each variant.
fn move_direction(direction: Direction) {
    match direction {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West"),
    }
}

fn main() {
    // Accessing a compile-time constant.
    println!("PI = {}", PI);

    // Accessing a static variable.
    println!("Application: {}", APP_NAME);

    // Create an instance of the Person struct.
    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };

    // Print the entire struct using the Debug formatter.
    println!("{:?}", person);

    // Call a method defined in the impl block.
    person.introduce();

    // Create an array containing all enum variants.
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    // Iterate through each direction and call the function.
    for direction in directions {
        move_direction(direction);
    }
}
