use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut is_previous_err = false;
    loop {
        if !is_previous_err {
            println!("Please input your guess.");
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                is_previous_err = true;
                continue;
            }
        };

        is_previous_err = false;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
