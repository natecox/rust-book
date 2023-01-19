extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // ..= is "range inclusive", `1..=100` is "between and including 1 and 100"
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // `mut` lets us change this later, immutable by default

        io::stdin()
            .read_line(&mut guess)          // `&mut` is a mutable reference, lets us modify
            .expect("Failed to read line"); // fail with an error if the input can't be read

        let guess: u32 = match guess.trim().parse() { // u32 is necessary so we can compare as a number
            Ok(num) => num,                           // arm
            Err(_) => continue,                       // arm
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {              // pattern expression
            Ordering::Less => println!("Too small!"),  // arm
            Ordering::Greater => println!("Too big!"), // arm
            Ordering::Equal => {                       // arm
                println!("You win!");
                break;
            }
        }
    }
}
