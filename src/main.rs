use std::io;
use env_logger::{Builder, Target};

fn execute_query(query: &str) {
    log::error!("Executing query: {}", query);
}

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    execute_query("DROP TABLE students");
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
