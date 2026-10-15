//! Gate for S1-T2. Do not edit.
use tasks::s01::t02_slices::*;

const SRC: &str = include_str!("../src/s01/t02_slices.rs");

#[test]
fn first_word_cases() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("   padded  text"), "padded");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("   \t "), "");
    assert_eq!(first_word("héllo wörld"), "héllo");
}

#[test]
fn longest_line_counts_chars_not_bytes() {
    assert_eq!(longest_line(""), None);
    assert_eq!(longest_line("a\nabc\nab"), Some("abc"));
    assert_eq!(longest_line("aa\nbb"), Some("aa"), "first wins ties");
    // 3 chars / 9 bytes vs 4 chars / 4 bytes
    assert_eq!(longest_line("日本語\nabcd"), Some("abcd"));
}

#[test]
fn split_header_body_cases() {
    assert_eq!(
        split_header_body("H: 1\nH: 2\n\nbody\nmore"),
        Some(("H: 1\nH: 2", "body\nmore"))
    );
    assert_eq!(split_header_body("no blank line\nhere"), None);
    assert_eq!(split_header_body("\n\nbody"), Some(("", "body")));
    assert_eq!(split_header_body("head\n\n"), Some(("head", "")));
    assert_eq!(split_header_body("a\n\nb\n\nc"), Some(("a", "b\n\nc")));
}

#[test]
fn truncate_chars_is_utf8_safe() {
    assert_eq!(truncate_chars("hello", 3), "hel");
    assert_eq!(truncate_chars("hello", 0), "");
    assert_eq!(truncate_chars("hello", 99), "hello");
    assert_eq!(truncate_chars("héllo", 2), "hé");
    assert_eq!(truncate_chars("日本語", 2), "日本");
    assert_eq!(truncate_chars("a🦀b", 2), "a🦀");
}

#[test]
fn truncate_chars_never_panics() {
    for s in ["", "a", "é", "日本語", "🦀🦀🦀", "e\u{301}x"] {
        for n in 0..8 {
            let out = truncate_chars(s, n);
            assert!(s.starts_with(out));
            assert!(out.chars().count() <= n);
        }
    }
}

#[test]
fn count_words_cases() {
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("   "), 0);
    assert_eq!(count_words("one"), 1);
    assert_eq!(count_words("  a  b\tc\nd "), 4);
}

#[test]
fn no_allocation_in_source() {
    for line in SRC.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        for banned in [
            "String",
            "Vec",
            "format!",
            "to_string",
            "to_owned",
            "collect",
            "Box",
        ] {
            assert!(!t.contains(banned), "`{banned}` found in: {t}");
        }
    }
}
