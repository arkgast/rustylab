//! S1-T2 · Slices, strings & UTF-8.
//!
//! Every function returns a view into the input. No allocation allowed:
//! the gate rejects this file if it builds owned strings or vectors.
//!
//! Gate: `scripts/gate.sh S1-T2`.

/// First whitespace-delimited word, skipping leading whitespace.
/// Returns `""` if there is none.
pub fn first_word(_s: &str) -> &str {
    todo!()
}

/// The line with the most characters (not bytes). First one wins ties.
/// `None` for empty input.
pub fn longest_line(_s: &str) -> Option<&str> {
    todo!()
}

/// Splits at the first blank line (`"\n\n"`). The separator is dropped.
/// `None` if there is no blank line.
pub fn split_header_body(_s: &str) -> Option<(&str, &str)> {
    todo!()
}

/// The first `n` characters (Unicode scalar values). Never panics and never
/// splits a code point.
pub fn truncate_chars(_s: &str, _n: usize) -> &str {
    todo!()
}

/// Number of whitespace-separated words.
pub fn count_words(_s: &str) -> usize {
    todo!()
}
