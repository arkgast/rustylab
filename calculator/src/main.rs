use std::io::{Write, stdin, stdout};

use calculator::Calculator;

fn main() {
    loop {
        let a = read_line();
        let a = match parse(a) {
            Ok(n) => n,
            Err(_) => continue,
        };

        let b = read_line();
        let b = match parse(b) {
            Ok(n) => n,
            Err(_) => continue,
        };

        let calculator = Calculator::new(a, b);

        println!("\n{}", calculator);
    }
}

fn read_line() -> String {
    let mut line = String::new();
    print!("Insert your number: ");
    stdout().flush().expect("Failed to flush");
    stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn parse(line: String) -> Result<i32, &'static str> {
    let num: i32 = line.trim().parse().map_err(|_| "Enter a valid number.")?;
    Ok(num)
}
