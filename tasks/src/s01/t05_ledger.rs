//! S1-T5 · Capstone: ledger CLI.
//!
//! A tiny bank that reads text commands:
//!
//! ```text
//! open alice
//! deposit alice 100
//! withdraw alice 30
//! transfer alice bob 50
//! balance alice
//! ```
//!
//! Two steps, two error types: [`Command::parse`] turns a line into a
//! [`Command`] (or a [`ParseError`]); [`Ledger::execute`] applies it (or
//! returns a [`LedgerError`] and leaves the ledger exactly as it was).
//!
//! Rules: no `unwrap`, `expect`, `panic!` or indexing (`v[i]`) outside
//! `#[cfg(test)]`; checked arithmetic for balances; at least 5 tests of your
//! own at the bottom of this file, at least 3 of them on failure paths.
//!
//! Run `scripts/run.sh S1-T5` (scripted session) or
//! `scripts/run.sh S1-T5 -- repl` (type commands yourself).

/// A parsed command. Names are owned: the command outlives the input line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Open(String),
    Deposit {
        account: String,
        amount: u64,
    },
    Withdraw {
        account: String,
        amount: u64,
    },
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },
    Balance(String),
}

/// Why a line is not a valid command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The line is empty or only whitespace.
    Empty,
    /// The first word is not one of the five commands (commands are lowercase).
    UnknownCommand(String),
    /// Right command, wrong number of arguments after it.
    WrongArgCount { expected: usize, found: usize },
    /// The amount is not a positive integer that fits in `u64` (zero is rejected).
    BadAmount(String),
}

impl Command {
    /// Parses one line. Words are separated by any whitespace.
    pub fn parse(_line: &str) -> Result<Command, ParseError> {
        todo!()
    }
}

/// One account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub name: String,
    pub balance: u64,
}

/// What a successful command produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Opened,
    /// The account's balance after `deposit`, `withdraw` or `balance`.
    Balance(u64),
    /// Both balances after a transfer.
    Transferred {
        from: u64,
        to: u64,
    },
}

/// Why a valid command could not be applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    AccountExists(String),
    UnknownAccount(String),
    InsufficientFunds {
        needed: u64,
        available: u64,
    },
    /// A balance would exceed `u64::MAX`.
    Overflow,
    /// `transfer` from an account to itself.
    SameAccount,
}

/// All accounts, in the order they were opened.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ledger {
    accounts: Vec<Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Balance of `name`, if the account exists.
    pub fn balance_of(&self, _name: &str) -> Option<u64> {
        todo!()
    }

    /// Applies `cmd`. On `Err`, the ledger is unchanged.
    pub fn execute(&mut self, _cmd: Command) -> Result<Outcome, LedgerError> {
        todo!()
    }
}

// Your tests go here: at least 5, at least 3 on failure paths.
#[cfg(test)]
mod tests {}
