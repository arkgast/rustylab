//! Gate for S1-T1. Do not edit.
mod common;

use tasks::s01::t01_basics::*;

const SRC: &str = include_str!("../src/s01/t01_basics.rs");

#[test]
fn fizzbuzz_cases() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(98), "98");
    assert_eq!(fizzbuzz(0), "FizzBuzz", "0 is a multiple of everything");
}

#[test]
fn grade_boundaries() {
    let cases = [
        (0, 'F'),
        (59, 'F'),
        (60, 'D'),
        (69, 'D'),
        (70, 'C'),
        (79, 'C'),
        (80, 'B'),
        (89, 'B'),
        (90, 'A'),
        (100, 'A'),
        (101, '?'),
        (255, '?'),
    ];
    for (score, want) in cases {
        assert_eq!(grade(score), want, "grade({score})");
    }
}

#[test]
fn digits() {
    assert_eq!(sum_of_digits(0), 0);
    assert_eq!(sum_of_digits(7), 7);
    assert_eq!(sum_of_digits(1234), 10);
    assert_eq!(sum_of_digits(u64::MAX), 87);
}

#[test]
fn primes() {
    let primes: Vec<u32> = (0..50).filter(|&n| is_prime(n)).collect();
    assert_eq!(
        primes,
        [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    );
    assert!(is_prime(4_294_967_291), "largest u32 prime");
    assert!(!is_prime(u32::MAX));
}

#[test]
fn powers_of_two() {
    assert_eq!(next_power_of_two(0), 1);
    assert_eq!(next_power_of_two(1), 1);
    assert_eq!(next_power_of_two(5), 8);
    assert_eq!(next_power_of_two(64), 64);
    assert_eq!(next_power_of_two(65), 128);
    assert_eq!(next_power_of_two(u32::MAX), 4_294_967_296);
}

#[test]
fn overflow() {
    assert_eq!(overflow_modes(250, 10), (None, 4, 255, (4, true)));
    assert_eq!(overflow_modes(1, 2), (Some(3), 3, 3, (3, false)));
    assert_eq!(overflow_modes(255, 255), (None, 254, 255, (254, true)));
}

#[test]
fn min_max() {
    assert_eq!(min_max3(1, 2, 3), (1, 3));
    assert_eq!(min_max3(3, 1, 2), (1, 3));
    assert_eq!(min_max3(2, 3, 1), (1, 3));
    assert_eq!(min_max3(-5, -5, -5), (-5, -5));
    assert_eq!(min_max3(i32::MIN, 0, i32::MAX), (i32::MIN, i32::MAX));
}

#[test]
fn rule_no_as_casts() {
    common::assert_absent(
        SRC,
        " as ",
        "widen with u64::from(x) or pick the right type",
    );
}

#[test]
fn rule_power_of_two_uses_loop_with_break_value() {
    let body = common::fn_body(SRC, "next_power_of_two");
    assert!(body.contains("loop"), "use a `loop`");
    let breaks_with_value = body
        .match_indices("break")
        .any(|(i, _)| !body[i + 5..].trim_start().starts_with(';'));
    assert!(
        breaks_with_value,
        "use `break <value>` to return from the loop"
    );
    assert!(!body.contains(".next_power_of_two("), "write it yourself");
}

#[test]
fn rule_grade_uses_range_patterns() {
    let body = common::fn_body(SRC, "grade");
    assert!(body.contains("match"), "use one `match`");
    assert!(body.contains("..="), "use range patterns like `90..=100`");
    assert!(
        !body.contains("if "),
        "no `if` chains: let the `match` do it"
    );
}
