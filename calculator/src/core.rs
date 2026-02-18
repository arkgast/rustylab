use std::{
    fmt,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Sub},
};

trait IntScalar:
    Copy
    + Eq
    + Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
{
}

pub struct Calculator<T> {
    a: T,
    b: T,
}

trait AdditiveOperations<T> {
    fn add(&self) -> T;
    fn subtract(&self) -> T;
}

trait MultiplicativeOperations<T> {
    fn multiply(&self) -> T;
    fn divide(&self) -> Option<T>;
}

trait BinaryOperations<T> {
    fn and(&self) -> T;
    fn or(&self) -> T;
    fn xor(&self) -> T;
}

impl<T> IntScalar for T where
    T: Copy
        + Eq
        + Default
        + PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
{
}

impl<T> Calculator<T> {
    pub fn new(a: T, b: T) -> Self {
        Calculator { a, b }
    }
}

impl<T: IntScalar> AdditiveOperations<T> for Calculator<T> {
    fn add(&self) -> T {
        self.a + self.b
    }
    fn subtract(&self) -> T {
        self.a - self.b
    }
}

impl<T: IntScalar> MultiplicativeOperations<T> for Calculator<T> {
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

impl<T: IntScalar> BinaryOperations<T> for Calculator<T> {
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
    T: fmt::Display + IntScalar,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Calculator:")?;
        writeln!(f, "\ta:{} b:{}", self.a, self.b)?;
        writeln!(f, "Operations:")?;
        writeln!(f, "\tAddition: {}", self.add())?;
        writeln!(f, "\tSubtraction: {}", self.subtract())?;

        writeln!(f, "\tMultiplication: {}", self.multiply())?;

        match self.divide() {
            Some(result) => writeln!(f, "\tDivision: {}", result)?,
            None => writeln!(f, "Division: undefined (division by zero)")?,
        }

        writeln!(f, "\tAND: {}", self.and())?;
        writeln!(f, "\tOR: {}", self.or())?;
        writeln!(f, "\tXOR: {}", self.xor())
    }
}
