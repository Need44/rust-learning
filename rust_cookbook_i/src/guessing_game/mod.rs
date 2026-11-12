use std::io;


pub fn play() {
    println!("RUNNING GAME!");
    println!("########################");

    println!("Guess the number!");

    println!("Your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read a line!");

    println!("Your guess was: {}", guess);
}