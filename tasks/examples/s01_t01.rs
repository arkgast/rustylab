//! Runner for S1-T1: `scripts/run.sh S1-T1`. Add your own `show!` lines freely.
use tasks::s01::t01_basics::*;
use tasks::{lab::kata, show};

fn main() {
    kata("fizzbuzz", || {
        let mut line = String::from("  1..=15 =>");
        for n in 1..=15 {
            line = format!("{line} {}", fizzbuzz(n));
        }
        println!("{line}");
    });
    kata("grade", || {
        for score in [100, 85, 72, 64, 12, 150] {
            println!("  grade({score}) => {:?}", grade(score));
        }
    });
    kata("sum_of_digits", || {
        show!(sum_of_digits(1234));
        show!(sum_of_digits(u64::MAX));
    });
    kata("is_prime", || {
        let mut primes = String::from("  primes below 40 =>");
        for n in 0..40 {
            if is_prime(n) {
                primes = format!("{primes} {n}");
            }
        }
        println!("{primes}");
        show!(is_prime(4_294_967_291));
    });
    kata("next_power_of_two", || {
        for n in [0, 1, 5, 64, 65, u32::MAX] {
            println!("  next_power_of_two({n}) => {}", next_power_of_two(n));
        }
    });
    kata(
        "overflow_modes  (checked, wrapping, saturating, overflowing)",
        || {
            show!(overflow_modes(250, 10));
            show!(overflow_modes(1, 2));
        },
    );
    kata("min_max3", || {
        show!(min_max3(3, -1, 2));
    });
}
