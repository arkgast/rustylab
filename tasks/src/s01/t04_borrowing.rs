//! S1-T4 · Borrowing, slices & strings.
//!
//! Replace every `todo!()`. Run `scripts/run.sh S1-T4` to see your results,
//! `scripts/gate.sh S1-T4` to check them.
//!
//! Rules: no `.clone()`; functions that return `&str` never allocate; nothing
//! here may panic, whatever the input (watch out for slicing inside a
//! multi-byte character).

// ---- slices ---------------------------------------------------------------

/// The largest element, or `None` for an empty slice.
pub fn largest(_xs: &[i32]) -> Option<i32> {
    todo!()
}

/// Sum of the even elements, as `i64` so it cannot overflow for any `&[i32]`
/// you can fit in memory.
pub fn sum_evens(_xs: &[i32]) -> i64 {
    todo!()
}

/// Rotates the slice left by `k` in place: `[1,2,3,4,5]`, k = 2 →
/// `[3,4,5,1,2]`. `k` may be larger than the length. No new `Vec`.
pub fn rotate_left(_xs: &mut [i32], _k: usize) {
    todo!()
}

/// For a sorted slice, the number of distinct values: `[1,1,2,3,3,3]` → 3.
pub fn dedup_sorted_len(_xs: &[i32]) -> usize {
    todo!()
}

// ---- strings --------------------------------------------------------------

/// The first whitespace-separated word, or `""`. Leading whitespace is
/// skipped: `first_word("  hello world") == "hello"`.
pub fn first_word(_s: &str) -> &str {
    todo!()
}

/// The longest line (by `chars().count()`); the first one wins a tie. `""` for
/// empty input.
pub fn longest_line(_text: &str) -> &str {
    todo!()
}

/// Splits an HTTP-like message at the first blank line (`"\n\n"`) into
/// `(header, body)`. `None` if there is no blank line.
pub fn split_header_body(_msg: &str) -> Option<(&str, &str)> {
    todo!()
}

/// The first `max_chars` characters of `s` (characters, not bytes). Never
/// panics and never splits a multi-byte character.
pub fn truncate_chars(_s: &str, _max_chars: usize) -> &str {
    todo!()
}

/// Number of whitespace-separated words.
pub fn count_words(_s: &str) -> usize {
    todo!()
}

/// Uppercases the first character of every word and keeps the rest as is;
/// words are re-joined with a single space:
/// `capitalize_words("hello  rust world") == "Hello Rust World"`.
pub fn capitalize_words(_s: &str) -> String {
    todo!()
}
