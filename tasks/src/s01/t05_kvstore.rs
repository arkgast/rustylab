//! S1-T5 · Capstone: in-memory key-value store.
//!
//! Commands (case-insensitive name, exact arity, single-token args):
//!
//! ```text
//! SET key value   → Output::Ok
//! GET key         → Output::Value(Some(v)) or Value(None)
//! DEL key         → Output::Deleted(true|false)
//! INCR key        → Output::Int(n)   missing key counts as 0 → 1
//! ```
//!
//! Parsing and executing are separate steps with separate error types.
//! A failed command must leave the store unchanged.
//!
//! Also required: add at least five `#[test]`s of your own in this file.
//!
//! Gate: `scripts/gate.sh S1-T5`.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Incr { key: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// Blank input.
    Empty,
    /// Unknown command word, as typed.
    UnknownCommand(String),
    /// `command` is the canonical upper-case name; `got` counts arguments.
    WrongArity {
        command: &'static str,
        expected: usize,
        got: usize,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Ok,
    Value(Option<String>),
    Deleted(bool),
    Int(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecError {
    /// `INCR` on a value that is not an `i64`.
    NotAnInteger { key: String },
    /// `INCR` would exceed `i64::MAX`.
    Overflow { key: String },
}

impl Command {
    pub fn parse(_input: &str) -> Result<Command, ParseError> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Store {
    #[allow(dead_code)] // remove once implemented
    data: std::collections::BTreeMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, _cmd: Command) -> Result<Output, ExecError> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
