use std::fmt;

#[derive(Debug)]
struct Calculator {
    a: i32,
    b: i32,
}

trait AdditiveOperations {
    fn add(&self) -> i32;
    fn subtract(&self) -> i32;
}

trait MultiplacativeOperations {
    fn multiply(&self) -> i32;
    fn divide(&self) -> Option<i32>;
}

trait BinaryOperations {
    fn and(&self) -> i32;
    fn or(&self) -> i32;
    fn xor(&self) -> i32;
}

impl Calculator {
    fn new(a: i32, b: i32) -> Self {
        Calculator { a, b }
    }
}

impl AdditiveOperations for Calculator {
    fn add(&self) -> i32 {
        self.a + self.b
    }
    fn subtract(&self) -> i32 {
        self.a - self.b
    }
}

impl MultiplacativeOperations for Calculator {
    fn multiply(&self) -> i32 {
        self.a * self.b
    }
    fn divide(&self) -> Option<i32> {
        if self.b == 0 {
            return None;
        }

        Some(self.a / self.b)
    }
}

impl BinaryOperations for Calculator {
    fn and(&self) -> i32 {
        self.a & self.b
    }

    fn or(&self) -> i32 {
        self.a | self.b
    }

    fn xor(&self) -> i32 {
        self.a ^ self.b
    }
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Addition: {}", self.add())?;
        writeln!(f, "Subtraction: {}", self.subtract())?;
        writeln!(f, "Multiplication: {}", self.multiply())?;

        match self.divide() {
            Some(result) => writeln!(f, "Division: {}", result)?,
            None => writeln!(f, "Division: undefined (division by zero)")?,
        }

        writeln!(f, "AND: {}", self.and())?;
        writeln!(f, "OR: {}", self.or())?;
        writeln!(f, "XOR: {}", self.xor())
    }
}

fn main() {
    let a = 400;
    let b = 200;

    let calculator = Calculator::new(a, b);

    println!("{:?}", calculator);
    println!("Operations: \n{}", calculator);
}
