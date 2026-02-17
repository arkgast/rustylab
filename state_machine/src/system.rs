use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Copy + AddAssign + One + Zero,
    Nonce: Copy + One + Zero,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }

    pub fn nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = self.nonce(who);
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::*;

    type TestPallet = Pallet<AccountId, BlockNumber, Nonce>;

    #[test]
    fn new_pallet_starts_at_block_zero() {
        let pallet = TestPallet::new();
        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number_increments_by_one() {
        let mut pallet = TestPallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 2);
    }

    #[test]
    fn inc_nonce_initializes_missing_account_to_one() {
        let mut pallet = TestPallet::new();
        pallet.inc_nonce(&"alice".to_string());
        assert_eq!(pallet.nonce(&"alice".to_string()), 1);
    }

    #[test]
    fn inc_nonce_increments_existing_account() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        pallet.inc_nonce(&alice);
        pallet.inc_nonce(&alice);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.nonce(&"alice".to_string()), 3);
    }

    #[test]
    fn inc_nonce_does_not_change_block_number() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        let block_number = pallet.block_number();
        pallet.inc_nonce(&alice);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.block_number(), block_number);
    }

    #[test]
    fn inc_block_number_does_not_change_nonce() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        let nonce = pallet.nonce(&alice);

        pallet.inc_block_number();
        pallet.inc_block_number();

        assert_eq!(pallet.nonce(&alice), nonce);
    }

    #[test]
    fn inc_nonce_returns_error_on_overflow() {}

    #[test]
    fn inc_block_number_returns_error_on_overflow() {}
}
