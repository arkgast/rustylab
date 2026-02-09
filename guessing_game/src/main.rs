use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess = String::new();

    let secret_number: u8 = random_range(0..=100);

    println!("Secret numbert is: {}", secret_number);

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
        }
        Ordering::Greater => {
            println!("Your guess number is higher than the expected number");
        }
    }
}
