//! Gate for S1-T4. Do not edit.
use tasks::s01::t04_patterns::*;

const SRC: &str = include_str!("../src/s01/t04_patterns.rs");

#[test]
fn tokenizes() {
    assert_eq!(
        tokenize("3 -4 + - * /"),
        Ok(vec![
            Token::Num(3),
            Token::Num(-4),
            Token::Add,
            Token::Sub,
            Token::Mul,
            Token::Div
        ])
    );
    assert_eq!(tokenize("1 x"), Err(RpnError::UnknownToken("x".into())));
    assert_eq!(tokenize("   "), Ok(vec![]));
}

#[test]
fn evaluates() {
    assert_eq!(eval_rpn("3 4 +"), Ok(7));
    assert_eq!(eval_rpn("5 1 2 + 4 * + 3 -"), Ok(14));
    assert_eq!(eval_rpn("-5 3 +"), Ok(-2));
    assert_eq!(eval_rpn("7 2 /"), Ok(3));
    assert_eq!(eval_rpn("-7 2 /"), Ok(-3));
    assert_eq!(eval_rpn("6 7 *"), Ok(42));
}

#[test]
fn errors() {
    assert_eq!(eval_rpn(""), Err(RpnError::Empty));
    assert_eq!(eval_rpn("+"), Err(RpnError::StackUnderflow));
    assert_eq!(eval_rpn("1 +"), Err(RpnError::StackUnderflow));
    assert_eq!(eval_rpn("1 0 /"), Err(RpnError::DivisionByZero));
    assert_eq!(eval_rpn("2 3 4"), Err(RpnError::LeftoverOperands(3)));
    assert_eq!(eval_rpn("1 2 x +"), Err(RpnError::UnknownToken("x".into())));
}

#[test]
fn overflow_is_an_error_not_a_panic() {
    let max = i64::MAX;
    let min = i64::MIN;
    assert_eq!(eval_rpn(&format!("{max} 1 +")), Err(RpnError::Overflow));
    assert_eq!(eval_rpn(&format!("{min} 1 -")), Err(RpnError::Overflow));
    assert_eq!(eval_rpn(&format!("{max} 2 *")), Err(RpnError::Overflow));
    assert_eq!(eval_rpn(&format!("{min} -1 /")), Err(RpnError::Overflow));
}

#[test]
fn no_wildcard_arms() {
    for line in SRC.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        assert!(!t.starts_with("_ =>"), "wildcard arm found: {t}");
    }
}
