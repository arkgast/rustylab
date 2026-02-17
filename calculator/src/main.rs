use std::{
    fmt,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Sub},
};

#[derive(Debug, Copy, Clone)]
struct Calculator<T> {
    a: T,
    b: T,
}

trait AdditiveOperations<T> {
    fn add(&self) -> T;
    fn subtract(&self) -> T;
}

trait MultiplacativeOperations<T> {
    fn multiply(&self) -> T;
    fn divide(&self) -> Option<T>;
}

trait BinaryOperations<T> {
    fn and(&self) -> T;
    fn or(&self) -> T;
    fn xor(&self) -> T;
}

impl<T> Calculator<T> {
    fn new(a: T, b: T) -> Self {
        Calculator { a, b }
    }
}

impl<T> AdditiveOperations<T> for Calculator<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    fn add(&self) -> T {
        self.a + self.b
    }
    fn subtract(&self) -> T {
        self.a - self.b
    }
}

impl<T> MultiplacativeOperations<T> for Calculator<T>
where
    T: Mul<Output = T> + Div<Output = T> + Copy + Eq + PartialEq + Default,
{
    fn multiply(&self) -> T {
        self.a * self.b
    }
    fn divide(&self) -> Option<T> {
        if self.b == T::default() {
            return None;
        }

        Some(self.a / self.b)
    }
}

impl<T> BinaryOperations<T> for Calculator<T>
where
    T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy,
{
    fn and(&self) -> T {
        self.a & self.b
    }

    fn or(&self) -> T {
        self.a | self.b
    }

    fn xor(&self) -> T {
        self.a ^ self.b
    }
}

impl<T> fmt::Display for Calculator<T>
where
    T: fmt::Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Copy
        + Default
        + Eq,
{
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
    let a = 400u32;
    let b = 200u32;

    let calculator = Calculator::new(a, b);

    println!("{:?}", calculator);
    println!("Operations: \n{}", calculator);
}
