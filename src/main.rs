use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    placeholder_example();
}

fn placeholder_example() {
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}, and x + y = {}", y + 2, x + y);
}