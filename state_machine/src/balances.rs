use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug, PartialEq, Eq)]
pub enum TransferError {
    NotEnoughBalance,
    BalanceOverflow,
    CannotTransferToSelf,
}

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Eq + Ord,
    Balance: CheckedAdd + CheckedSub + Zero + Clone,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        self.balances.get(who).unwrap_or(&Balance::zero()).clone()
    }

    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), TransferError> {
        if from == to {
            return Err(TransferError::CannotTransferToSelf);
        }

        let from_current_balance = self.balance(&from);
        let to_current_balance = self.balance(&to);

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     struct TransferCase<'a> {
//         from: AccountId<'a>,
//         to: AccountId<'a>,
//         amount: Balance,
//         expected_error: TransferError,
//     }
//
//     fn assert_failed_transfer_is_atomic<'a>(pallet: &mut Pallet<'a>, case: TransferCase<'a>) {
//         let from_before = pallet.balance(case.from);
//         let to_before = pallet.balance(case.to);
//
//         let err = pallet
//             .transfer(case.from, case.to, case.amount)
//             .unwrap_err();
//         assert_eq!(err, case.expected_error);
//
//         assert_eq!(pallet.balance(case.from), from_before);
//         assert_eq!(pallet.balance(case.to), to_before);
//     }
//
//     #[test]
//     fn unknown_account_is_zero() {
//         let mut pallet = Pallet::new();
//         pallet.set_balance("alice", 10);
//         assert_eq!(pallet.balance("bob"), 0);
//     }
//
//     #[test]
//     fn set_balance_overwrites() {
//         let mut pallet = Pallet::new();
//         pallet.set_balance("alice", 100);
//         pallet.set_balance("alice", 50);
//         assert_eq!(pallet.balance("alice"), 50);
//     }
//
//     #[test]
//     fn transfer_deducts_amount_from_sender() {
//         let mut pallet = Pallet::new();
//
//         pallet.set_balance("alice", 100);
//         let transfer_result = pallet.transfer("alice", "bob", 50);
//
//         assert_eq!(pallet.balance("alice"), 50);
//         assert!(transfer_result.is_ok());
//     }
//
//     #[test]
//     fn transfer_credits_amount_to_receiver() {
//         let mut pallet = Pallet::new();
//
//         pallet.set_balance("alice", 100);
//         let transfer_result = pallet.transfer("alice", "bob", 50);
//
//         assert_eq!(pallet.balance("bob"), 50);
//         assert!(transfer_result.is_ok());
//     }
//
//     #[test]
//     fn transfer_fails_if_sender_has_insufficient_balance() {
//         let mut pallet = Pallet::new();
//
//         pallet.set_balance("alice", 10);
//
//         assert_failed_transfer_is_atomic(
//             &mut pallet,
//             TransferCase {
//                 from: "alice",
//                 to: "bob",
//                 amount: 100,
//                 expected_error: TransferError::NotEnoughBalance,
//             },
//         );
//     }
//
//     #[test]
//     fn transfer_fails_if_receiver_balance_overflows() {
//         let mut pallet = Pallet::new();
//
//         pallet.set_balance("alice", 100);
//         pallet.set_balance("bob", u128::MAX);
//
//         assert_failed_transfer_is_atomic(
//             &mut pallet,
//             TransferCase {
//                 from: "alice",
//                 to: "bob",
//                 amount: 50,
//                 expected_error: TransferError::BalanceOverflow,
//             },
//         );
//     }
//
//     #[test]
//     fn transfer_to_self_returns_error() {
//         let mut pallet = Pallet::new();
//
//         assert_failed_transfer_is_atomic(
//             &mut pallet,
//             TransferCase {
//                 from: "alice",
//                 to: "alice",
//                 amount: 100,
//                 expected_error: TransferError::CannotTransferToSelf,
//             },
//         );
//     }
// }
