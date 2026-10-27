//! Shared helpers for the gate tests. Do not edit.
#![allow(dead_code)] // each test crate uses a different subset

/// Code lines of a task file: comments skipped, and everything from the first
/// `#[cfg(test)]` on ignored (rules apply to the implementation, not to tests).
pub fn code_lines(src: &str) -> impl Iterator<Item = (usize, &str)> {
    src.lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l))
        .take_while(|(_, l)| !l.trim_start().starts_with("#[cfg(test)]"))
        .filter(|(_, l)| !l.trim_start().starts_with("//"))
}

/// Panics with the line number if any code line contains `needle`.
pub fn assert_absent(src: &str, needle: &str, why: &str) {
    for (n, line) in code_lines(src) {
        assert!(
            !line.contains(needle),
            "line {n}: `{needle}` is not allowed here ({why}):\n    {}",
            line.trim()
        );
    }
}

/// Body text of `fn name` (from its signature to the next top-level item).
pub fn fn_body<'a>(src: &'a str, name: &str) -> &'a str {
    let sig = format!("fn {name}(");
    let start = src
        .find(&sig)
        .unwrap_or_else(|| panic!("fn {name} not found"));
    let rest = &src[start..];
    let end = rest[sig.len()..]
        .find("\npub fn ")
        .map_or(rest.len(), |i| i + sig.len());
    &rest[..end]
}

/// Counts `#[test]` attributes in the source.
pub fn count_tests(src: &str) -> usize {
    src.matches("#[test]").count()
}
