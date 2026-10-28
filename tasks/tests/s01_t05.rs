//! Gate for S1-T5. Do not edit.
mod common;

use tasks::s01::t05_ledger::*;

const SRC: &str = include_str!("../src/s01/t05_ledger.rs");

fn s(x: &str) -> String {
    x.to_string()
}

fn run(ledger: &mut Ledger, line: &str) -> Result<Outcome, LedgerError> {
    let cmd = Command::parse(line).unwrap_or_else(|e| panic!("{line:?} should parse: {e:?}"));
    ledger.execute(cmd)
}

fn funded() -> Ledger {
    let mut l = Ledger::new();
    for line in ["open alice", "open bob", "deposit alice 100"] {
        run(&mut l, line).expect("setup");
    }
    l
}

#[test]
fn parses_every_command() {
    assert_eq!(Command::parse("open alice"), Ok(Command::Open(s("alice"))));
    assert_eq!(
        Command::parse("  deposit   alice  100 "),
        Ok(Command::Deposit {
            account: s("alice"),
            amount: 100
        })
    );
    assert_eq!(
        Command::parse("withdraw bob 7"),
        Ok(Command::Withdraw {
            account: s("bob"),
            amount: 7
        })
    );
    assert_eq!(
        Command::parse("transfer alice bob 18446744073709551615"),
        Ok(Command::Transfer {
            from: s("alice"),
            to: s("bob"),
            amount: u64::MAX
        })
    );
    assert_eq!(
        Command::parse("balance\tbob"),
        Ok(Command::Balance(s("bob")))
    );
}

#[test]
fn parse_errors() {
    assert_eq!(Command::parse(""), Err(ParseError::Empty));
    assert_eq!(Command::parse("  \t "), Err(ParseError::Empty));
    assert_eq!(
        Command::parse("steal alice"),
        Err(ParseError::UnknownCommand(s("steal")))
    );
    assert_eq!(
        Command::parse("OPEN alice"),
        Err(ParseError::UnknownCommand(s("OPEN")))
    );
    assert_eq!(
        Command::parse("open"),
        Err(ParseError::WrongArgCount {
            expected: 1,
            found: 0
        })
    );
    assert_eq!(
        Command::parse("transfer alice bob"),
        Err(ParseError::WrongArgCount {
            expected: 3,
            found: 2
        })
    );
    assert_eq!(
        Command::parse("balance a b"),
        Err(ParseError::WrongArgCount {
            expected: 1,
            found: 2
        })
    );
    for bad in ["ten", "-5", "0", "1.5", "18446744073709551616"] {
        assert_eq!(
            Command::parse(&format!("deposit alice {bad}")),
            Err(ParseError::BadAmount(s(bad))),
            "amount {bad:?}"
        );
    }
}

#[test]
fn happy_path() {
    let mut l = Ledger::new();
    assert_eq!(run(&mut l, "open alice"), Ok(Outcome::Opened));
    assert_eq!(run(&mut l, "open bob"), Ok(Outcome::Opened));
    assert_eq!(run(&mut l, "deposit alice 100"), Ok(Outcome::Balance(100)));
    assert_eq!(run(&mut l, "withdraw alice 30"), Ok(Outcome::Balance(70)));
    assert_eq!(
        run(&mut l, "transfer alice bob 50"),
        Ok(Outcome::Transferred { from: 20, to: 50 })
    );
    assert_eq!(run(&mut l, "balance bob"), Ok(Outcome::Balance(50)));
    assert_eq!(l.balance_of("alice"), Some(20));
    assert_eq!(l.balance_of("carol"), None);
}

#[test]
fn failures_are_typed() {
    let mut l = funded();
    assert_eq!(
        run(&mut l, "open alice"),
        Err(LedgerError::AccountExists(s("alice")))
    );
    assert_eq!(
        run(&mut l, "deposit carol 1"),
        Err(LedgerError::UnknownAccount(s("carol")))
    );
    assert_eq!(
        run(&mut l, "balance carol"),
        Err(LedgerError::UnknownAccount(s("carol")))
    );
    assert_eq!(
        run(&mut l, "withdraw alice 101"),
        Err(LedgerError::InsufficientFunds {
            needed: 101,
            available: 100
        })
    );
    assert_eq!(
        run(&mut l, "transfer alice alice 1"),
        Err(LedgerError::SameAccount)
    );
    assert_eq!(
        run(&mut l, "transfer alice carol 1"),
        Err(LedgerError::UnknownAccount(s("carol")))
    );
    assert_eq!(
        run(&mut l, "transfer carol alice 1"),
        Err(LedgerError::UnknownAccount(s("carol")))
    );
}

#[test]
fn overflow_is_an_error() {
    let mut l = funded();
    assert_eq!(
        run(&mut l, "deposit alice 18446744073709551615"),
        Err(LedgerError::Overflow)
    );
    run(&mut l, "deposit bob 18446744073709551615").expect("bob can hold u64::MAX");
    assert_eq!(
        run(&mut l, "transfer alice bob 1"),
        Err(LedgerError::Overflow)
    );
}

#[test]
fn failed_commands_leave_state_unchanged() {
    let mut l = funded();
    run(&mut l, "deposit bob 18446744073709551600").expect("setup");
    let before = l.clone();
    let failing = [
        "open bob",
        "deposit nobody 5",
        "withdraw alice 1000",
        "transfer alice bob 50",
        "transfer alice alice 1",
        "transfer alice nobody 1",
        "transfer nobody alice 1",
        "transfer alice bob 101",
    ];
    for line in failing {
        assert!(run(&mut l, line).is_err(), "{line:?} should fail");
        assert_eq!(l, before, "{line:?} changed the ledger");
    }
}

#[test]
fn rule_no_panicking_shortcuts() {
    for banned in [".unwrap()", ".expect(", "panic!", "unreachable!", "todo!"] {
        common::assert_absent(SRC, banned, "return a typed error instead");
    }
}

#[test]
fn rule_no_indexing() {
    for (n, line) in common::code_lines(SRC) {
        let b = line.as_bytes();
        for i in 1..b.len() {
            let before = b[i - 1];
            let is_index = b[i] == b'['
                && (before.is_ascii_alphanumeric() || matches!(before, b'_' | b')' | b']'));
            assert!(
                !is_index,
                "line {n}: indexing can panic, use a slice pattern or a for loop:\n    {}",
                line.trim()
            );
        }
    }
}

#[test]
fn rule_own_tests() {
    let n = common::count_tests(SRC);
    assert!(
        n >= 5,
        "write at least 5 #[test]s in the source file, found {n}"
    );
}
