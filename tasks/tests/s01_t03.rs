//! Gate for S1-T3. Do not edit.
use tasks::s01::t03_option_result::*;

const SRC: &str = include_str!("../src/s01/t03_option_result.rs");

#[test]
fn parses_full_config() {
    let cfg = parse_config("host = example.org\nport = 8080\ndebug = true\n").unwrap();
    assert_eq!(
        cfg,
        Config {
            host: "example.org".into(),
            port: 8080,
            debug: true
        }
    );
}

#[test]
fn debug_defaults_to_false() {
    let cfg = parse_config("host=h\nport=1").unwrap();
    assert!(!cfg.debug);
}

#[test]
fn skips_comments_blanks_and_trims() {
    let cfg = parse_config("# c\n\n  host =   h  \n\t port=  22 \n#end").unwrap();
    assert_eq!(cfg.host, "h");
    assert_eq!(cfg.port, 22);
}

#[test]
fn last_duplicate_wins() {
    let cfg = parse_config("host=a\nhost=b\nport=1").unwrap();
    assert_eq!(cfg.host, "b");
}

#[test]
fn missing_keys_in_order() {
    assert_eq!(
        parse_config("port=1"),
        Err(ConfigError::MissingKey { key: "host" })
    );
    assert_eq!(
        parse_config("host=h"),
        Err(ConfigError::MissingKey { key: "port" })
    );
    assert_eq!(
        parse_config(""),
        Err(ConfigError::MissingKey { key: "host" })
    );
}

#[test]
fn line_errors_carry_one_based_line_numbers() {
    assert_eq!(
        parse_config("host=h\nport=1\nnonsense"),
        Err(ConfigError::InvalidLine { line: 3 })
    );
    assert_eq!(
        parse_config("host=h\ncolor=red\nport=1"),
        Err(ConfigError::UnknownKey { line: 2 })
    );
    assert_eq!(
        parse_config("host=h\nport=70000"),
        Err(ConfigError::InvalidPort { line: 2 })
    );
    assert_eq!(
        parse_config("host=h\nport=abc"),
        Err(ConfigError::InvalidPort { line: 2 })
    );
    assert_eq!(
        parse_config("host=h\nport=1\ndebug=maybe"),
        Err(ConfigError::InvalidBool { line: 3 })
    );
}

#[test]
fn line_errors_win_over_missing_keys() {
    // port missing AND line 1 is invalid → report the line error
    assert_eq!(
        parse_config("???"),
        Err(ConfigError::InvalidLine { line: 1 })
    );
}

#[test]
fn errors_display_and_implement_error() {
    let e = ConfigError::InvalidPort { line: 42 };
    assert!(e.to_string().contains("42"));
    let m = ConfigError::MissingKey { key: "host" };
    assert!(m.to_string().contains("host"));
    let _: &dyn std::error::Error = &e;
}

#[test]
fn no_panicking_shortcuts() {
    for line in SRC.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        for banned in [
            "unwrap(",
            "expect(",
            "panic!",
            "unreachable!",
            "todo!",
            "unimplemented!",
        ] {
            assert!(!t.contains(banned), "`{banned}` found in: {t}");
        }
    }
    assert!(SRC.contains('?'), "expected use of the `?` operator");
}
