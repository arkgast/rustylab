use crate::types::{AccountId, BlockNumber, Nonce};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<'a> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId<'a>, Nonce>,
}

impl<'a> Pallet<'a> {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn nonce(&self, who: AccountId<'a>) -> Nonce {
        *self.nonce.get(who).unwrap_or(&0)
    }

    pub fn inc_nonce(&mut self, who: AccountId<'a>) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who, nonce + 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_pallet_starts_at_block_zero() {
        let pallet = Pallet::new();
        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number_increments_by_one() {
        let mut pallet = Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 2);
    }

    #[test]
    fn inc_nonce_initializes_missing_account_to_one() {
        let mut pallet = Pallet::new();
        pallet.inc_nonce("alice");
        assert_eq!(pallet.nonce("alice"), 1);
    }
}
