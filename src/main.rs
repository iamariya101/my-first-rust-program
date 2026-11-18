use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");
    println!("I'm thinking of a number from 1 to 100.");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("\nPlease enter your guess (or type \"quit\" to exit):");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Goodbye!");
                break;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Could not read your input: {error}");
                break;
            }
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        let guess: u32 = match input.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a whole number from 1 to 100.");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("Your guess must be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
