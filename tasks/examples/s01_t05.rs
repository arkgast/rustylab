//! Runner for S1-T5.
//!   scripts/run.sh S1-T5            scripted session
//!   scripts/run.sh S1-T5 -- repl    type commands yourself (Ctrl-D to quit)
use std::io::{self, BufRead, Write};

use tasks::lab::kata;
use tasks::s01::t05_ledger::{Command, Ledger};

const SCRIPT: &[&str] = &[
    "open alice",
    "open bob",
    "deposit alice 100",
    "withdraw alice 30",
    "transfer alice bob 50",
    "balance bob",
    "transfer alice bob 500",
    "open alice",
    "deposit carol 5",
    "steal alice",
    "deposit alice -5",
    "transfer alice",
    "",
];

fn step(ledger: &mut Ledger, line: &str) {
    match Command::parse(line) {
        Err(e) => println!("  {line:<24} ✗ parse error: {e:?}"),
        Ok(cmd) => match ledger.execute(cmd) {
            Ok(outcome) => println!("  {line:<24} ✓ {outcome:?}"),
            Err(e) => println!("  {line:<24} ✗ rejected: {e:?}"),
        },
    }
}

fn main() {
    if std::env::args().any(|a| a == "repl") {
        repl();
        return;
    }
    kata("scripted session", || {
        let mut ledger = Ledger::new();
        for line in SCRIPT {
            step(&mut ledger, line);
        }
        println!("\n  final state: {ledger:?}");
    });
}

fn repl() {
    println!("ledger repl: open | deposit | withdraw | transfer | balance  (Ctrl-D quits)");
    let mut ledger = Ledger::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        if io::stdout().flush().is_err() {
            return;
        }
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => kata("", || step(&mut ledger, line.trim_end())),
        }
    }
}
