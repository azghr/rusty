use std::cmp;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::random_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Input you guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
