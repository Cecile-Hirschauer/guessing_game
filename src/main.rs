use std::io; // prelude: bring io library into scope

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable: guess is a new, empty instance of a String

    io::stdin() // stdin function returns an instance of std::io::Stdin . stdin: standard input handle for the terminal (io::stdin())
        .read_line(&mut guess) // read_line method on the standard input handle to get input from the user. & indicates that this argument is a reference
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
