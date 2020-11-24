use std::io;

fn main() {
    println!("Can you guess the number?");

    println!("Okay, show me what you got...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed:  {}", guess);
}
