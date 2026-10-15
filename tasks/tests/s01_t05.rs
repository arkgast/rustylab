//! Gate for S1-T5. Do not edit.
use tasks::s01::t05_kvstore::*;

const SRC: &str = include_str!("../src/s01/t05_kvstore.rs");

fn run(store: &mut Store, line: &str) -> Result<Output, ExecError> {
    store.execute(Command::parse(line).expect("parse"))
}

#[test]
fn parse_commands_case_insensitive() {
    assert_eq!(
        Command::parse("set a 1"),
        Ok(Command::Set {
            key: "a".into(),
            value: "1".into()
        })
    );
    assert_eq!(
        Command::parse("GET a"),
        Ok(Command::Get { key: "a".into() })
    );
    assert_eq!(
        Command::parse("  Del   a "),
        Ok(Command::Del { key: "a".into() })
    );
    assert_eq!(
        Command::parse("incr a"),
        Ok(Command::Incr { key: "a".into() })
    );
}

#[test]
fn parse_errors() {
    assert_eq!(Command::parse(""), Err(ParseError::Empty));
    assert_eq!(Command::parse("   "), Err(ParseError::Empty));
    assert_eq!(
        Command::parse("FLY a"),
        Err(ParseError::UnknownCommand("FLY".into()))
    );
    assert_eq!(
        Command::parse("SET a"),
        Err(ParseError::WrongArity {
            command: "SET",
            expected: 2,
            got: 1
        })
    );
    assert_eq!(
        Command::parse("get"),
        Err(ParseError::WrongArity {
            command: "GET",
            expected: 1,
            got: 0
        })
    );
    assert_eq!(
        Command::parse("DEL a b"),
        Err(ParseError::WrongArity {
            command: "DEL",
            expected: 1,
            got: 2
        })
    );
}

#[test]
fn set_get_del() {
    let mut s = Store::new();
    assert!(s.is_empty());
    assert_eq!(run(&mut s, "SET a hello"), Ok(Output::Ok));
    assert_eq!(
        run(&mut s, "GET a"),
        Ok(Output::Value(Some("hello".into())))
    );
    assert_eq!(run(&mut s, "GET b"), Ok(Output::Value(None)));
    assert_eq!(s.len(), 1);
    assert_eq!(run(&mut s, "DEL a"), Ok(Output::Deleted(true)));
    assert_eq!(run(&mut s, "DEL a"), Ok(Output::Deleted(false)));
    assert!(s.is_empty());
}

#[test]
fn incr() {
    let mut s = Store::new();
    assert_eq!(run(&mut s, "INCR n"), Ok(Output::Int(1)));
    assert_eq!(run(&mut s, "INCR n"), Ok(Output::Int(2)));
    assert_eq!(run(&mut s, "SET n -5"), Ok(Output::Ok));
    assert_eq!(run(&mut s, "INCR n"), Ok(Output::Int(-4)));
    assert_eq!(run(&mut s, "GET n"), Ok(Output::Value(Some("-4".into()))));
}

#[test]
fn incr_errors_leave_store_unchanged() {
    let mut s = Store::new();
    run(&mut s, "SET word abc").unwrap();
    assert_eq!(
        run(&mut s, "INCR word"),
        Err(ExecError::NotAnInteger { key: "word".into() })
    );
    assert_eq!(
        run(&mut s, "GET word"),
        Ok(Output::Value(Some("abc".into())))
    );

    run(&mut s, &format!("SET big {}", i64::MAX)).unwrap();
    assert_eq!(
        run(&mut s, "INCR big"),
        Err(ExecError::Overflow { key: "big".into() })
    );
    assert_eq!(
        run(&mut s, "GET big"),
        Ok(Output::Value(Some(i64::MAX.to_string())))
    );
    assert_eq!(s.len(), 2);
}

#[test]
fn author_wrote_their_own_tests() {
    let n = SRC.matches("#[test]").count();
    assert!(
        n >= 5,
        "add at least 5 #[test]s in t05_kvstore.rs (found {n})"
    );
}

#[test]
fn no_panicking_shortcuts() {
    // Only inspect the code before the author's test module.
    let code = SRC.split("#[cfg(test)]").next().unwrap_or(SRC);
    for line in code.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        for banned in ["unwrap(", "expect(", "panic!", "todo!", "unimplemented!"] {
            assert!(!t.contains(banned), "`{banned}` found in: {t}");
        }
    }
}
