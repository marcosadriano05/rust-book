use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("To end this game, guess the number or enter the word \"exit\".");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess = guess.trim();
        if guess == "exit" {
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numbers are allowed. Please, try again.");
                continue;
            }
        };

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
