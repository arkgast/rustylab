//! S1-T1 · Cargo, bindings & control flow.
//!
//! Replace every `todo!()`. Run `scripts/run.sh S1-T1` to see your results,
//! `scripts/gate.sh S1-T1` to check them.
//!
//! Rules: no `as` casts in this file; `next_power_of_two` uses `loop` with
//! `break <value>`; `grade` is one `match` with range patterns.

/// `"Fizz"` for multiples of 3, `"Buzz"` for multiples of 5, `"FizzBuzz"` for
/// both, otherwise the number itself (`fizzbuzz(7) == "7"`).
pub fn fizzbuzz(_n: u32) -> String {
    todo!()
}

/// Letter grade: 90..=100 → 'A', 80..=89 → 'B', 70..=79 → 'C', 60..=69 → 'D',
/// 0..=59 → 'F'. Anything above 100 is invalid → '?'.
pub fn grade(_score: u8) -> char {
    todo!()
}

/// Sum of the decimal digits: `sum_of_digits(1234) == 10`, `sum_of_digits(0) == 0`.
pub fn sum_of_digits(_n: u64) -> u64 {
    todo!()
}

/// `true` if `n` is prime. 0 and 1 are not prime. Stop at the square root.
pub fn is_prime(_n: u32) -> bool {
    todo!()
}

/// Smallest power of two `>= n`. `next_power_of_two(0) == 1`,
/// `next_power_of_two(5) == 8`, `next_power_of_two(u32::MAX) == 4294967296`.
///
/// Must use `loop` + `break <value>` (no `u32::next_power_of_two`).
pub fn next_power_of_two(_n: u32) -> u64 {
    todo!()
}

/// The four ways Rust lets you add integers, side by side, for `a + b`:
/// `(checked, wrapping, saturating, overflowing)`.
///
/// `overflow_modes(250, 10) == (None, 4, 255, (4, true))`
///
/// Preview: `Option` is taught in S1-T2. `checked_add` returns `Some(sum)`,
/// or `None` ("no value") when the sum does not fit; pass it through as is.
pub fn overflow_modes(_a: u8, _b: u8) -> (Option<u8>, u8, u8, (u8, bool)) {
    todo!()
}

/// `(min, max)` of three numbers, without sorting or a `Vec`.
pub fn min_max3(_a: i32, _b: i32, _c: i32) -> (i32, i32) {
    todo!()
}
