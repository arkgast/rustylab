//! S1-T1 · Ownership & borrowing lab.
//!
//! 1. Implement [`Inventory`] (no `.clone()` in the methods).
//! 2. Add at least six `compile_fail` doctests, each on a public item below
//!    (put them in the `///` docs of `Inventory` or its methods). Cover at
//!    least five distinct error codes. Open a fenced block whose info string
//!    is `compile_fail,E0382` (backticks, three of them), put the code that
//!    must not compile inside, and above it comment: which rule it breaks and
//!    how to fix it.
//!
//!    Suggested codes: E0382 (use after move), E0499 (two `&mut`), E0502
//!    (`&` while `&mut`), E0505 (move while borrowed), E0506 (assign while
//!    borrowed), E0597 or E0515 (dangling reference).
//!
//!    Stable rustc runs the block and requires a compile error; the error
//!    code is checked on nightly only, but the gate checks you wrote it.
//!
//! Gate: `scripts/gate.sh S1-T1`.

/// A named stock entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub qty: u32,
}

/// A collection of [`Item`]s keyed by name.
///
/// TODO: write your `compile_fail` doctests here.
#[derive(Debug, Default)]
pub struct Inventory {
    #[allow(dead_code)] // remove once implemented
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `qty` of `name`. If the name already exists, quantities merge.
    pub fn add(&mut self, _name: String, _qty: u32) {
        todo!()
    }

    /// Removes and returns the whole entry, if present (ownership moves out).
    pub fn take(&mut self, _name: &str) -> Option<Item> {
        todo!()
    }

    /// Looks at an entry without removing it.
    pub fn peek(&self, _name: &str) -> Option<&Item> {
        todo!()
    }

    /// Sum of all quantities.
    pub fn total(&self) -> u32 {
        todo!()
    }
}
