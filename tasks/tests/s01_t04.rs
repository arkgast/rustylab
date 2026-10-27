//! Gate for S1-T4. Do not edit.
mod common;

use tasks::s01::t04_borrowing::*;

const SRC: &str = include_str!("../src/s01/t04_borrowing.rs");

#[test]
fn largest_values() {
    assert_eq!(largest(&[]), None);
    assert_eq!(largest(&[-3]), Some(-3));
    assert_eq!(largest(&[1, 9, -4, 9, 2]), Some(9));
    assert_eq!(largest(&[i32::MIN, i32::MIN]), Some(i32::MIN));
}

#[test]
fn sum_of_evens() {
    assert_eq!(sum_evens(&[]), 0);
    assert_eq!(sum_evens(&[1, 2, 3, 4, -6]), 0);
    assert_eq!(sum_evens(&[2, 4]), 6);
    let big = [i32::MAX - 1; 4];
    assert_eq!(sum_evens(&big), 4 * i64::from(i32::MAX - 1));
}

#[test]
fn rotation() {
    let mut v = [1, 2, 3, 4, 5];
    rotate_left(&mut v, 2);
    assert_eq!(v, [3, 4, 5, 1, 2]);
    rotate_left(&mut v, 5);
    assert_eq!(v, [3, 4, 5, 1, 2], "full turn is a no-op");
    rotate_left(&mut v, 13);
    assert_eq!(v, [1, 2, 3, 4, 5], "k wraps around");
    let mut empty: [i32; 0] = [];
    rotate_left(&mut empty, 3);
}

#[test]
fn distinct_in_sorted() {
    assert_eq!(dedup_sorted_len(&[]), 0);
    assert_eq!(dedup_sorted_len(&[7]), 1);
    assert_eq!(dedup_sorted_len(&[1, 1, 2, 3, 3, 3]), 3);
    assert_eq!(dedup_sorted_len(&[-1, 0, 1]), 3);
}

#[test]
fn first_words() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("  padded\tword"), "padded");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word("   "), "");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("ñandú rápido"), "ñandú");
}

#[test]
fn first_word_borrows_from_input() {
    let text = String::from("  zero copy");
    let w = first_word(&text);
    let start = w.as_ptr() as usize - text.as_ptr() as usize;
    assert_eq!(start, 2, "the result must point into the input");
}

#[test]
fn longest_lines() {
    assert_eq!(longest_line(""), "");
    assert_eq!(longest_line("a\nccc\nbb"), "ccc");
    assert_eq!(longest_line("ab\ncd"), "ab", "first wins a tie");
    assert_eq!(
        longest_line("abcd\n🦀🦀🦀🦀🦀"),
        "🦀🦀🦀🦀🦀",
        "count chars, not bytes"
    );
    assert_eq!(longest_line("ééé\nabcd"), "abcd", "é is 2 bytes but 1 char");
}

#[test]
fn header_body() {
    assert_eq!(
        split_header_body("Host: x\nA: b\n\nhello\n\nworld"),
        Some(("Host: x\nA: b", "hello\n\nworld"))
    );
    assert_eq!(split_header_body("\n\n"), Some(("", "")));
    assert_eq!(split_header_body("no blank line\nhere"), None);
}

#[test]
fn truncation_respects_chars() {
    assert_eq!(truncate_chars("hello", 3), "hel");
    assert_eq!(truncate_chars("hello", 10), "hello");
    assert_eq!(truncate_chars("hello", 0), "");
    assert_eq!(truncate_chars("", 2), "");
    assert_eq!(truncate_chars("héllo", 2), "hé");
    assert_eq!(truncate_chars("🦀rust", 1), "🦀");
    assert_eq!(truncate_chars("🦀🦀", 2), "🦀🦀");
}

#[test]
fn truncation_never_panics() {
    let samples = ["", "a", "é", "🦀", "a🦀é z", "日本語テキスト"];
    for s in samples {
        for n in 0..12 {
            let out = truncate_chars(s, n);
            assert!(s.starts_with(out));
            assert_eq!(out.chars().count(), n.min(s.chars().count()));
        }
    }
}

#[test]
fn word_counts() {
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("   "), 0);
    assert_eq!(count_words("one"), 1);
    assert_eq!(count_words(" two  words\n"), 2);
    assert_eq!(count_words("tabs\tand\nnewlines too"), 4);
}

#[test]
fn capitalization() {
    assert_eq!(capitalize_words("hello  rust world"), "Hello Rust World");
    assert_eq!(capitalize_words(""), "");
    assert_eq!(capitalize_words("  x  "), "X");
    assert_eq!(capitalize_words("éclair über"), "Éclair Über");
    assert_eq!(capitalize_words("mIxEd case"), "MIxEd Case");
}

#[test]
fn rule_no_clone() {
    common::assert_absent(SRC, ".clone()", "borrow instead");
}
