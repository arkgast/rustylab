use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

pub trait Config {
    type AccountId: Clone + Eq + Ord;
    type BlockNumber: CheckedAdd + Copy + One + Zero;
    type Nonce: CheckedAdd + Copy + One + Zero;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SystemError {
    BlockNumberOverflow,
    NonceOverflow,
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) -> Result<(), SystemError> {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .ok_or(SystemError::BlockNumberOverflow)?;

        Ok(())
    }

    pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
        self.nonce.get(who).copied().unwrap_or_else(T::Nonce::zero)
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) -> Result<(), SystemError> {
        let nonce = self.nonce(who);
        let new_nonce = nonce
            .checked_add(&T::Nonce::one())
            .ok_or(SystemError::NonceOverflow)?;
        self.nonce.insert(who.clone(), new_nonce);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl<T: Config> Pallet<T> {
        fn set_nonce(&mut self, who: &T::AccountId, nonce: T::Nonce) {
            self.nonce.insert(who.clone(), nonce);
        }

        fn set_block_number(&mut self, block_number: T::BlockNumber) {
            self.block_number = block_number;
        }
    }

    #[test]
    fn new_pallet_starts_at_block_zero() {
        let pallet = Pallet::<TestConfig>::new();
        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number_increments_by_one() {
        let mut pallet = Pallet::<TestConfig>::new();
        pallet.inc_block_number().unwrap();
        assert_eq!(pallet.block_number(), 1);
        pallet.inc_block_number().unwrap();
        assert_eq!(pallet.block_number(), 2);
    }

    #[test]
    fn inc_nonce_initializes_missing_account_to_one() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        pallet.inc_nonce(&alice).unwrap();
        assert_eq!(pallet.nonce(&alice), 1);
    }

    #[test]
    fn inc_nonce_increments_existing_account() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        pallet.inc_nonce(&alice).unwrap();
        pallet.inc_nonce(&alice).unwrap();
        pallet.inc_nonce(&alice).unwrap();
        assert_eq!(pallet.nonce(&alice), 3);
    }

    #[test]
    fn inc_nonce_does_not_change_block_number() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        let block_number = pallet.block_number();
        pallet.inc_nonce(&alice).unwrap();
        pallet.inc_nonce(&alice).unwrap();
        assert_eq!(pallet.block_number(), block_number);
    }

    #[test]
    fn inc_block_number_does_not_change_nonce() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        let nonce = pallet.nonce(&alice);

        pallet.inc_block_number().unwrap();
        pallet.inc_block_number().unwrap();

        assert_eq!(pallet.nonce(&alice), nonce);
    }

    #[test]
    fn inc_nonce_returns_error_on_overflow() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        pallet.set_nonce(&alice, u32::MAX);

        let err = pallet.inc_nonce(&alice).unwrap_err();
        assert_eq!(err, SystemError::NonceOverflow);

        assert_eq!(pallet.nonce(&alice), u32::MAX);
    }

    #[test]
    fn inc_block_number_returns_error_on_overflow() {
        let mut pallet = Pallet::<TestConfig>::new();
        pallet.set_block_number(u32::MAX);

        let err = pallet.inc_block_number().unwrap_err();
        assert_eq!(err, SystemError::BlockNumberOverflow);

        assert_eq!(pallet.block_number(), u32::MAX);
    }
}
