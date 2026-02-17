use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

#[derive(Debug, PartialEq, Eq)]
pub enum SystemError {
    BlockNumberOverflow,
    NonceOverflow,
}

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: CheckedAdd + Copy + One + Zero,
    Nonce: CheckedAdd + Copy + One + Zero,
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

    pub fn inc_block_number(&mut self) -> Result<(), SystemError> {
        self.block_number = self
            .block_number
            .checked_add(&BlockNumber::one())
            .ok_or(SystemError::BlockNumberOverflow)?;

        Ok(())
    }

    pub fn nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }

    pub fn inc_nonce(&mut self, who: &AccountId) -> Result<(), SystemError> {
        let nonce = self.nonce(who);
        let new_nonce = nonce
            .checked_add(&Nonce::one())
            .ok_or(SystemError::NonceOverflow)?;
        self.nonce.insert(who.clone(), new_nonce);
        Ok(())
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
        pallet.inc_block_number().unwrap();
        assert_eq!(pallet.block_number(), 1);
        pallet.inc_block_number().unwrap();
        assert_eq!(pallet.block_number(), 2);
    }

    #[test]
    fn inc_nonce_initializes_missing_account_to_one() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        pallet.inc_nonce(&alice).unwrap();
        assert_eq!(pallet.nonce(&alice), 1);
    }

    #[test]
    fn inc_nonce_increments_existing_account() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        pallet.inc_nonce(&alice).unwrap();
        pallet.inc_nonce(&alice).unwrap();
        pallet.inc_nonce(&alice).unwrap();
        assert_eq!(pallet.nonce(&alice), 3);
    }

    #[test]
    fn inc_nonce_does_not_change_block_number() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        let block_number = pallet.block_number();
        pallet.inc_nonce(&alice).unwrap();
        pallet.inc_nonce(&alice).unwrap();
        assert_eq!(pallet.block_number(), block_number);
    }

    #[test]
    fn inc_block_number_does_not_change_nonce() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        let nonce = pallet.nonce(&alice);

        pallet.inc_block_number().unwrap();
        pallet.inc_block_number().unwrap();

        assert_eq!(pallet.nonce(&alice), nonce);
    }

    #[test]
    fn inc_nonce_returns_error_on_overflow() {
        let mut pallet = TestPallet::new();
        let alice = "alice".to_string();
        pallet.nonce.insert(alice.clone(), u32::MAX);

        let err = pallet.inc_nonce(&alice).unwrap_err();
        assert_eq!(err, SystemError::NonceOverflow);

        assert_eq!(pallet.nonce(&alice), u32::MAX);
    }

    #[test]
    fn inc_block_number_returns_error_on_overflow() {
        let mut pallet = TestPallet::new();
        pallet.block_number = u32::MAX;

        let err = pallet.inc_block_number().unwrap_err();
        assert_eq!(err, SystemError::BlockNumberOverflow);

        assert_eq!(pallet.block_number(), u32::MAX);
    }
}
