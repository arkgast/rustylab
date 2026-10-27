//! Gate for S1-T3. Do not edit.
mod common;

use tasks::s01::t03_ownership::*;

const SRC: &str = include_str!("../src/s01/t03_ownership.rs");

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn shout_reuses_the_string() {
    let mut original = String::with_capacity(16);
    original.push_str("hey");
    let ptr = original.as_ptr();
    let out = shout(original);
    assert_eq!(out, "hey!");
    assert_eq!(
        out.as_ptr(),
        ptr,
        "same heap buffer: the String was moved, not copied"
    );
}

#[test]
fn longer_picks_by_bytes_and_a_on_tie() {
    assert_eq!(longer(s("abc"), s("de")), "abc");
    assert_eq!(longer(s("a"), s("xyz")), "xyz");
    assert_eq!(longer(s("ab"), s("cd")), "ab");
}

#[test]
fn wallet_lifecycle() {
    let mut w = Wallet::new(s("alice"));
    assert_eq!(w.owner, "alice");
    assert!(w.coins.is_empty());
    w.deposit(25);
    w.deposit(100);
    w.deposit(5);
    assert_eq!(w.total(), 130);
    assert_eq!(w.take_largest(), Some(100));
    assert_eq!(w.take_largest(), Some(25));
    assert_eq!(w.total(), 5);
    assert_eq!(w.into_owner(), "alice");
}

#[test]
fn take_largest_on_empty() {
    let mut w = Wallet::new(s("bob"));
    assert_eq!(w.take_largest(), None);
}

#[test]
fn total_saturates() {
    let mut w = Wallet::new(s("whale"));
    for _ in 0..3 {
        w.deposit(u32::MAX);
    }
    assert_eq!(w.total(), 3 * u64::from(u32::MAX));
}

#[test]
fn drain_into_moves_every_coin() {
    let mut a = Wallet::new(s("a"));
    let mut b = Wallet::new(s("b"));
    a.deposit(1);
    a.deposit(2);
    b.deposit(9);
    a.drain_into(&mut b);
    assert!(a.coins.is_empty());
    assert_eq!(b.coins, [9, 1, 2]);
    assert_eq!(a.owner, "a", "only coins move");
}

#[test]
fn swap_owners_moves_names() {
    let mut a = Wallet::new(s("alice"));
    let mut b = Wallet::new(s("bob"));
    let (pa, pb) = (a.owner.as_ptr(), b.owner.as_ptr());
    swap_owners(&mut a, &mut b);
    assert_eq!((a.owner.as_str(), b.owner.as_str()), ("bob", "alice"));
    assert_eq!((a.owner.as_ptr(), b.owner.as_ptr()), (pb, pa), "no copies");
}

#[test]
fn rule_nothing_is_copied() {
    for banned in [".clone()", ".to_owned()", ".to_string()", "String::from"] {
        common::assert_absent(SRC, banned, "move values instead of copying them");
    }
}
