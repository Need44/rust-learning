use std::{io,
cmp::Ordering};
use rand::prelude::*;


pub fn play() {
    println!("RUNNING GAME!");
    println!("########################");

    println!("Guess the number!");

    let secret_random_number = rand::rng().random_range(1..=100);

    println!("Your guess: ");
    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read a line!");
        
        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue;
        };
        match guess.cmp(&secret_random_number){
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
        
        println!("Your guess was: {}", guess);
    }
}