use rand::random_range;
use std::cmp::Ordering;
use std::io::{self};
use std::iter::repeat_n;

const MAX_GUESSING_NUMBER: u8 = 100;
const MIN_GUESSING_NUMBER: u8 = 0;

const MAX_LIVES: u8 = 5;
const LIFE_EMPTY: char = '□';
const LIFE_LOST: char = '✘';

fn main() {
    let mut lives_used: u8 = 0;

    let secret_number: u8 = random_range(0..=100);

    #[cfg(debug_assertions)]
    eprintln!("Secret numbert is: {}", secret_number);

    while lives_used < MAX_LIVES {
        let mut guess = String::new();

        print_lives(lives_used);

        println!("Insert your guess number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        lives_used += 1;

        let guess: u8 = match guess.trim().parse() {
            Ok(guess) => {
                if !(MIN_GUESSING_NUMBER..=MAX_GUESSING_NUMBER).contains(&guess) {
                    println!("Guess number should be in the range of 0 and 100");
                    continue;
                }
                guess
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess number is lower than the expected number");
            }
            Ordering::Equal => {
                println!("You've guessed the secret number");
                return;
            }
            Ordering::Greater => {
                println!("Your guess number is higher than the expected number");
            }
        }
    }

    print_lives(lives_used);
    println!("You've lost");
}

fn print_lives(lives_used: u8) {
    let remaining = MAX_LIVES - lives_used;

    let lives: String = repeat_n(LIFE_LOST, lives_used as usize)
        .chain(repeat_n(LIFE_EMPTY, remaining as usize))
        .collect();

    println!("Available lives: {}", lives);
}
