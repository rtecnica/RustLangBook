/*
 *
 *
 *
 *
 *
 */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;
    let max_tries = 7;

    while tries < max_tries {
        println!("Guess the secret number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        tries = tries + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guess it! {}", secret_number);
                break;
            }
        }

        println!("Tries left: {}", max_tries - tries);
    }
    if tries == max_tries {
        println!("Out of tries!");
    }
}
