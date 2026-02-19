use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config {
    type AccountId: Clone + Eq + Ord;
    type Balance: CheckedAdd + CheckedSub + Copy + Eq + Zero;
}

#[derive(Debug, PartialEq, Eq)]
pub enum TransferError {
    NotEnoughBalance,
    BalanceOverflow,
    CannotTransferToSelf,
    ZeroTransfer,
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        self.balances
            .get(who)
            .copied()
            .unwrap_or_else(T::Balance::zero)
    }

    pub fn transfer(
        &mut self,
        from: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), TransferError> {
        if from == to {
            return Err(TransferError::CannotTransferToSelf);
        }

        if amount == T::Balance::zero() {
            return Err(TransferError::ZeroTransfer);
        }

        let from_current_balance = self.balance(from);
        let to_current_balance = self.balance(to);

        let from_new_balance = from_current_balance
            .checked_sub(&amount)
            .ok_or(TransferError::NotEnoughBalance)?;
        let to_new_balance = to_current_balance
            .checked_add(&amount)
            .ok_or(TransferError::BalanceOverflow)?;

        self.set_balance(from, from_new_balance);
        self.set_balance(to, to_new_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::types;

    use super::*;

    struct TransferCase {
        from: types::AccountId,
        to: types::AccountId,
        amount: types::Balance,
        expected_error: TransferError,
    }

    struct TestPallet;

    impl super::Config for TestPallet {
        type AccountId = String;
        type Balance = u128;
    }

    fn assert_failed_transfer_is_atomic(pallet: &mut Pallet<TestPallet>, case: TransferCase) {
        let from_before = pallet.balance(&case.from);
        let to_before = pallet.balance(&case.to);

        let err = pallet
            .transfer(&case.from, &case.to, case.amount)
            .unwrap_err();
        assert_eq!(err, case.expected_error);

        assert_eq!(pallet.balance(&case.from), from_before);
        assert_eq!(pallet.balance(&case.to), to_before);
    }

    #[test]
    fn balance_for_unknown_account_is_zero() {
        let mut pallet = Pallet::<TestPallet>::new();
        pallet.set_balance(&"alice".to_string(), 10);
        assert_eq!(pallet.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn set_balance_overwrites() {
        let mut pallet = Pallet::<TestPallet>::new();
        let alice = "alice".to_string();
        pallet.set_balance(&alice, 100);
        pallet.set_balance(&alice, 50);
        assert_eq!(pallet.balance(&alice), 50);
    }

    #[test]
    fn transfer_deducts_amount_from_sender() {
        let mut pallet = Pallet::<TestPallet>::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);
        let transfer_result = pallet.transfer(&alice, &bob, 50);

        assert_eq!(pallet.balance(&alice), 50);
        assert!(transfer_result.is_ok());
    }

    #[test]
    fn transfer_credits_amount_to_receiver() {
        let mut pallet = Pallet::<TestPallet>::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);
        let transfer_result = pallet.transfer(&alice, &bob, 50);

        assert_eq!(pallet.balance(&bob), 50);
        assert!(transfer_result.is_ok());
    }

    #[test]
    fn transfer_fails_if_sender_has_insufficient_balance() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 10);

        assert_failed_transfer_is_atomic(
            &mut pallet,
            TransferCase {
                from: alice.clone(),
                to: bob.clone(),
                amount: 100,
                expected_error: TransferError::NotEnoughBalance,
            },
        );
    }

    #[test]
    fn transfer_fails_if_receiver_balance_overflows() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);
        pallet.set_balance(&bob, u128::MAX);

        assert_failed_transfer_is_atomic(
            &mut pallet,
            TransferCase {
                from: alice.clone(),
                to: bob.clone(),
                amount: 50,
                expected_error: TransferError::BalanceOverflow,
            },
        );
    }

    #[test]
    fn transfer_to_self_returns_error() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();

        assert_failed_transfer_is_atomic(
            &mut pallet,
            TransferCase {
                from: alice.clone(),
                to: alice.clone(),
                amount: 100,
                expected_error: TransferError::CannotTransferToSelf,
            },
        );
    }

    #[test]
    fn transfer_with_amount_zero_returns_error() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        assert_failed_transfer_is_atomic(
            &mut pallet,
            TransferCase {
                from: alice.clone(),
                to: bob.clone(),
                amount: 0,
                expected_error: TransferError::ZeroTransfer,
            },
        );
    }
}
