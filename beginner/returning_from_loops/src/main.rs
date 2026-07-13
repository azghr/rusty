fn main() {
    let mut attempts = 0;

    let password = loop {
        attempts += 1;
        println!("Attempt #{attempts}");

        // Simulate checking a password.
        let is_correct = attempts == 3;

        if is_correct {
            println!("Access granted!");
            break "correct horse battery staple";
        }

        println!("Wrong password. Try again.\n");
    };

    println!("\nPassword used: {password}");
    println!("Logged in after {attempts} attempts.");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
