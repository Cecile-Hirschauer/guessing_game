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

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable: guess is a new, empty instance of a String

        io::stdin() // stdin function returns an instance of std::io::Stdin . stdin: standard input handle for the terminal (io::stdin())
            .read_line(&mut guess) // read_line method on the standard input handle to get input from the user. & indicates that this argument is a reference
            .expect("Failed to read line");

        // introducing guess as a new variable with a new type: u32 (unsigned 32-bit integer)
        // trim: method on the string to eliminate whitespace* (spaces, tabs, etc.) and newlines
        // parse: method on strings that parses a string into some kind of number
        // expect: method that allows us to handle potential failure by providing an error message
        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing: reusing the guess variable name
        println!("You guessed: {}", guess);

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
