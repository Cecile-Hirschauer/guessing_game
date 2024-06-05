use std::io; // prelude: bring io library into scope
use std::cmp::Ordering; // prelude: bring Ordering enum into scope - cmp: comparison crate (dependency in Cargo.toml)
use rand::Rng; // prelude: bring Rng trait into scope - rand: random number generator crate (dependency in Cargo.toml) Rng: random number generator

fn main() {
    println!("Guess the number!");

    // thread_rng: function that gives us the particular random number generator that we're going to use.
    // gen_range: method that generates a random number between the two numbers we pass as arguments
    // (inclusive on the lower bound but exclusive on the upper bound)
    let secret_number = rand::thread_rng().gen_range(1..=101);

    println!("The secret number is: {}", secret_number);

   // loop: keyword that creates an infinite loop
    // Allowing the player to guess multiple times until they guess the correct number
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable: guess is a new, empty instance of a String

        io::stdin() // stdin function returns an instance of std::io::Stdin . stdin: standard input handle for the terminal (io::stdin())
            .read_line(&mut guess) // read_line method on the standard input handle to get input from the user. & indicates that this argument is a reference
            .expect("Failed to read line");

        // trim: method on the String instance to eliminate whitespace at the beginning and end of the string
        // parse: method that parses a string into some kind of number. parse can parse a variety of number types,
        // so we need to tell Rust the exact number type we want by using annotations
        let guess: u32 = match guess.trim().parse() { // match: keyword that allows us to compare the result of parse to Ok and Err
            Ok(num) => num, // Ok: variant that matches if the parse method returns a number
            Err(_) => continue, // continue: keyword that tells the program to go to the next iteration of the loop
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // Ordering::Equal: variant that matches if the guess is equal to the secret number
            Ordering::Equal => {
                println!("You win!");
                break; // break: keyword that exits the loop
            }
        }
    }
}
