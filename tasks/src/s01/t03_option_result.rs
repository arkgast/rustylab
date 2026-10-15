//! S1-T3 · Option & Result without unwrap.
//!
//! Parse text like:
//!
//! ```text
//! # comment
//! host = example.org
//! port = 8080
//! debug = true      (optional, default false)
//! ```
//!
//! Rules: blank lines and `#` comments are skipped; whitespace around keys and
//! values is trimmed; a repeated key keeps the last value; line numbers are
//! 1-based. Error precedence: report line-level errors in file order first,
//! then missing keys (`host` before `port`).
//!
//! Gate: `scripts/gate.sh S1-T3`.

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// Line has no `=`.
    InvalidLine { line: usize },
    /// Key is not `host`, `port` or `debug`.
    UnknownKey { line: usize },
    /// `port` is not a valid `u16`.
    InvalidPort { line: usize },
    /// `debug` is not `true` or `false`.
    InvalidBool { line: usize },
    /// Required key never appeared.
    MissingKey { key: &'static str },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("include the line number for line-level errors")
    }
}

impl std::error::Error for ConfigError {}

pub fn parse_config(_input: &str) -> Result<Config, ConfigError> {
    todo!()
}
