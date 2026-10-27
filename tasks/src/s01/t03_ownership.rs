//! S1-T3 · Ownership, moves & Copy.
//!
//! Replace every `todo!()`. Run `scripts/run.sh S1-T3` to see your results,
//! `scripts/gate.sh S1-T3` to check them.
//!
//! Rule: no `.clone()`, `.to_owned()`, `.to_string()` or `String::from` in
//! this file. Every value you return was moved, never copied.

/// Takes ownership of `s`, appends `"!"` and gives the same `String` back
/// (no new allocation needed if there is spare capacity).
pub fn shout(_s: String) -> String {
    todo!()
}

/// Returns whichever string is longer in bytes (`a` on a tie). The other one is
/// dropped when the function returns.
pub fn longer(_a: String, _b: String) -> String {
    todo!()
}

/// A wallet that owns its owner's name and a bag of coins (values in cents).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Wallet {
    pub owner: String,
    pub coins: Vec<u32>,
}

impl Wallet {
    pub fn new(_owner: String) -> Self {
        todo!()
    }

    pub fn deposit(&mut self, _coin: u32) {
        todo!()
    }

    /// Sum of all coins, saturating at `u64::MAX`.
    pub fn total(&self) -> u64 {
        todo!()
    }

    /// Consumes the wallet and hands back only the owner's name.
    pub fn into_owner(self) -> String {
        todo!()
    }

    /// Removes and returns the most valuable coin, if any.
    pub fn take_largest(&mut self) -> Option<u32> {
        todo!()
    }

    /// Moves every coin from `self` into `other`. Afterwards `self.coins` is
    /// empty and `other` holds its old coins followed by `self`'s coins.
    pub fn drain_into(&mut self, _other: &mut Wallet) {
        todo!()
    }
}

/// Swaps the owner names of two wallets without copying either name.
pub fn swap_owners(_a: &mut Wallet, _b: &mut Wallet) {
    todo!()
}
