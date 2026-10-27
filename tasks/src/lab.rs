//! Helpers for the task runners in `tasks/examples/`, so you can *see* your
//! code work: `scripts/run.sh S1-T1` prints every kata's results.

use std::any::Any;
use std::panic::{self, AssertUnwindSafe};

/// Prints an expression next to its value: `show!(is_prime(7))` prints
/// `is_prime(7) => true`. Values use their `Debug` format, so strings keep
/// their quotes and you can spot stray whitespace.
#[macro_export]
macro_rules! show {
    ($e:expr) => {
        println!("  {} => {:?}", stringify!($e), $e)
    };
}

/// Runs one kata's demo under a heading.
///
/// A `todo!()` shows as ⏳ instead of aborting the whole run, so katas can be
/// solved in any order. Any other panic is printed with its location as 💥.
pub fn kata(name: &str, demo: impl FnOnce()) {
    println!("\n▶ {name}");
    let previous = panic::take_hook();
    panic::set_hook(Box::new(|info| {
        if !is_todo(info.payload()) {
            println!("  💥 {info}");
        }
    }));
    let result = panic::catch_unwind(AssertUnwindSafe(demo));
    panic::set_hook(previous);
    if let Err(payload) = result
        && is_todo(payload.as_ref())
    {
        println!("  ⏳ not implemented yet");
    }
}

fn is_todo(payload: &(dyn Any + Send)) -> bool {
    let msg = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str));
    msg.is_some_and(|m| m.starts_with("not yet implemented"))
}
