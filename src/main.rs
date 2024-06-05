use std::io; // prelude: bring io library into scope
use rand::Rng; // prelude: bring Rng trait into scope - rand: random number generator crate (dependency in Cargo.toml) Rng: random number generator

fn main() {
    println!("Guess the number!");

    // thread_rng: function that gives us the particular random number generator that we're going to use.
    // gen_range: method that generates a random number between the two numbers we pass as arguments
    // (inclusive on the lower bound but exclusive on the upper bound)
    let secret_number = rand::thread_rng().gen_range(1..= 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable: guess is a new, empty instance of a String

    io::stdin() // stdin function returns an instance of std::io::Stdin . stdin: standard input handle for the terminal (io::stdin())
        .read_line(&mut guess) // read_line method on the standard input handle to get input from the user. & indicates that this argument is a reference
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
