//! Gate for S1-T1. Do not edit.
use tasks::s01::t01_ownership::{Inventory, Item};

const SRC: &str = include_str!("../src/s01/t01_ownership.rs");

#[test]
fn add_merges_quantities() {
    let mut inv = Inventory::new();
    inv.add("bolt".to_string(), 3);
    inv.add("bolt".to_string(), 4);
    inv.add("nut".to_string(), 1);
    assert_eq!(
        inv.peek("bolt"),
        Some(&Item {
            name: "bolt".into(),
            qty: 7
        })
    );
    assert_eq!(inv.total(), 8);
}

#[test]
fn take_moves_the_entry_out() {
    let mut inv = Inventory::new();
    inv.add("bolt".to_string(), 3);
    let item = inv.take("bolt").expect("present");
    assert_eq!(item.qty, 3);
    assert!(inv.peek("bolt").is_none());
    assert!(inv.take("bolt").is_none());
    assert_eq!(inv.total(), 0);
}

#[test]
fn missing_entries_are_none() {
    let inv = Inventory::new();
    assert!(inv.peek("x").is_none());
    assert_eq!(inv.total(), 0);
}

#[test]
fn at_least_six_compile_fail_doctests_with_five_distinct_codes() {
    let fence = "```compile_fail,E";
    let mut codes: Vec<&str> = Vec::new();
    let mut count = 0;
    for part in SRC.split(fence).skip(1) {
        count += 1;
        if let Some(code) = part.get(..4)
            && !codes.contains(&code)
        {
            codes.push(code);
        }
    }
    assert!(
        count >= 6,
        "expected >= 6 compile_fail doctests, found {count}"
    );
    assert!(
        codes.len() >= 5,
        "expected >= 5 distinct error codes, found {codes:?}"
    );
}

#[test]
fn no_clone_in_implementation() {
    for line in SRC.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        assert!(!t.contains(".clone()"), "`.clone()` found: {t}");
    }
}
