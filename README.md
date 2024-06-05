# Guessing Game in Rust

Welcome to the Guessing Game project! This is a simple guessing game written in Rust, following the tutorial from the official Rust documentation. The game will prompt the user to guess a number between 1 and 100, and will provide feedback on whether the guess is too low, too high, or correct.

## Concepts Introduced

This project introduces several fundamental Rust concepts, each of which is explained briefly below:

### 1. **Crates and Modules**
- **Crates:** A crate is a compilation unit in Rust. The project uses the `rand` crate to generate random numbers. This is included in the `Cargo.toml` file.
- **Modules:** Modules allow you to organize your code. Although this project is simple and does not use custom modules, understanding crates is the first step.

### 2. **Use Statements**
- The `use` keyword is used to bring items into scope. For example:
  ```rust
  use std::io;
  use rand::Rng;
  ```

### 3. **Variables and Mutability**
- Variables in Rust are immutable by default. You can make them mutable by using the `mut` keyword:
  ```rust
  let mut guess = String::new();
  ```

### 4. **Input and Output**
- Rust provides functionalities for input and output through the `std::io` module. We use `stdin().read_line(&mut guess)` to get user input.

### 5. **Data Types and Parsing**
- The program demonstrates how to convert a `String` to another data type, such as an integer:
  ```rust
  let guess: u32 = guess.trim().parse().expect("Please type a number!");
  ```

### 6. **Error Handling**
- Rust handles potential errors using the `Result` type. The `expect` method is used to handle errors that may occur when parsing input.

### 7. **Control Flow**
The `match` statement is used to handle different outcomes of the `Result` type:
  ```rust
  match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
          println!("You win!");
          break;
      }
  }
  ```

### 8. **Random Number Generation**
- The `rand::Rng` trait provides methods for generating random numbers. The secret number in this game is generated as follows:
  ```rust
  let secret_number = rand::thread_rng().gen_range(1..=100);
  ```

## Running the Project

To run the project, ensure you have Rust installed. Then, follow these steps:

1. Clone the repository.
2. Navigate to the project directory.
3. Run the following command:
   ```sh
   cargo run
   ```

You will be prompted to guess a number between 1 and 100. The game will provide feedback until you guess correctly.

## Conclusion

This simple guessing game introduces essential Rust concepts that form the foundation for more complex programs. By understanding these concepts, you can start building more advanced applications in Rust.
