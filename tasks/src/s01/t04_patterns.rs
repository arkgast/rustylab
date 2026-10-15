//! S1-T4 · Enums & exhaustive matching: a reverse-Polish-notation evaluator.
//!
//! `"3 4 +"` → 7. Numbers may be negative (`"-5"`), the lone `-` is subtraction.
//! Integer division truncates toward zero.
//!
//! Rules enforced by the gate: no `_ =>` wildcard arms in this file, and all
//! arithmetic is checked.
//!
//! Gate: `scripts/gate.sh S1-T4`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Num(i64),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RpnError {
    /// Token that is neither a number nor an operator. Carries the text.
    UnknownToken(String),
    /// Operator without two operands available.
    StackUnderflow,
    DivisionByZero,
    /// Any checked operation that overflowed (including `i64::MIN / -1`).
    Overflow,
    /// Input contained no tokens.
    Empty,
    /// More than one value left. Carries how many.
    LeftoverOperands(usize),
}

pub fn tokenize(_input: &str) -> Result<Vec<Token>, RpnError> {
    todo!()
}

pub fn eval_rpn(_input: &str) -> Result<i64, RpnError> {
    todo!()
}
