use rand::random_range;
use std::cmp::Ordering;
use std::io;

const MAX_LIVES: u8 = 5;
const LIFE_EMPTY: char = '□';
const LIFE_LOST: char = '✘';

fn main() {
    let mut lives_used: u8 = 0;

    let secret_number: u8 = random_range(0..=100);

    println!("Secret numbert is: {}", secret_number);

    while lives_used < MAX_LIVES {
        let mut guess = String::new();

        print_lives(lives_used);

        println!("Insert your guess number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = guess.trim().parse().expect("Failed to parse guess number");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess number is lower than the expected number");
            }
            Ordering::Equal => {
                println!("You've guessed the secret number");
                break;
            }
            Ordering::Greater => {
                println!("Your guess number is higher than the expected number");
            }
        }

        lives_used += 1;
    }

    print_lives(lives_used);
    println!("You've lost");
}

fn print_lives(attempts: u8) {
    println!(
        "Available lives: {}{}",
        LIFE_LOST.to_string().repeat(attempts as usize),
        LIFE_EMPTY
            .to_string()
            .repeat((MAX_LIVES - attempts) as usize)
    );
}
