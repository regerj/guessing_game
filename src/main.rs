use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Notes on match, can be used not only as a switch for execution, but also as a kind
// of turnary operator of sorts, see the shadowing assignment.
fn main() {
    println!("Guess the number!");

    // Randomly generate a number using a generator retrieved as "rand::thread_rng()"
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Inferred but statically typed mutable variable guess
        let mut guess = String::new();

        // Inferred but statically typed variable cmd_line (similar to "rand::thread_rng()")
        let cmd_line = io::stdin();

        // Calling read_line() method of the command line with error checking via expect()
        // note explicitely mutable reference, implied immutability
        cmd_line.read_line(&mut guess).expect("Failed to read line");

        // Explicitly statically typed variable shadowing guess, includes error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Match is like a sexy switch statement? Again passes as reference, though immutable
        // No default case, prob bad practice xd
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}