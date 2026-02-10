use rand::random_range;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::iter::repeat_n;

const MAX_GUESSING_NUMBER: u8 = 100;
const MIN_GUESSING_NUMBER: u8 = 0;

const MAX_LIVES: u8 = 5;
const LIFE_EMPTY: char = '□';
const LIFE_LOST: char = '✘';

fn main() -> io::Result<()> {
    let mut lives_used: u8 = 0;

    let secret_number: u8 = random_range(0..=100);

    #[cfg(debug_assertions)]
    eprintln!("Secret numbert is: {}", secret_number);

    while lives_used < MAX_LIVES {
        print_lives(lives_used);

        let line = read_line()?;

        let guess = match parse_guess(&line) {
            Ok(input) => input,
            Err(msg) => {
                println!("{msg}");
                continue;
            }
        };

        lives_used += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess number is lower than the expected number");
            }
            Ordering::Equal => {
                println!("You've guessed the secret number!");
                return Ok(());
            }
            Ordering::Greater => {
                println!("Your guess number is higher than the expected number");
            }
        }
        println!();
    }

    print_lives(lives_used);
    println!("You've lost. The number was {secret_number}");

    Ok(())
}

fn read_line() -> io::Result<String> {
    let mut line = String::new();
    print!("Insert your guess number: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn parse_guess(input: &str) -> Result<u8, &'static str> {
    let guess: u8 = input
        .trim()
        .parse()
        .map_err(|_| "Please enter a valid number")?;

    if !(MIN_GUESSING_NUMBER..=MAX_GUESSING_NUMBER).contains(&guess) {
        return Err("Guess number should be in teh range of 0 and 100");
    }

    Ok(guess)
}

fn print_lives(lives_used: u8) {
    let remaining = MAX_LIVES - lives_used;

    let lives: String = repeat_n(LIFE_LOST, lives_used as usize)
        .chain(repeat_n(LIFE_EMPTY, remaining as usize))
        .collect();

    println!("Available lives: {}", lives);
}
