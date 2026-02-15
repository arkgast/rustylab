use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &str, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    pub fn balance(&self, who: &str) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u128) -> Result<(), &'static str> {
        let from_current_balance = self.balance(from);
        let to_current_balance = self.balance(to);

        let from_new_balance = from_current_balance
            .checked_sub(amount)
            .ok_or("Not enough balance")?;
        let to_new_balance = to_current_balance
            .checked_add(amount)
            .ok_or("Balance overflow")?;

        self.set_balance(from, from_new_balance);
        self.set_balance(to, to_new_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_account_is_zero() {
        let mut pallet = Pallet::new();
        pallet.set_balance("alice", 10);
        assert_eq!(pallet.balance("bob"), 0);
    }

    #[test]
    fn set_balance_overwrites() {
        let mut pallet = Pallet::new();
        pallet.set_balance("alice", 100);
        pallet.set_balance("alice", 50);
        assert_eq!(pallet.balance("alice"), 50);
    }

    #[test]
    fn transfer_deducts_amount_from_sender() {
        let mut pallet = Pallet::new();

        pallet.set_balance("alice", 100);
        let transfer_result = pallet.transfer("alice", "bob", 50);

        assert_eq!(pallet.balance("alice"), 50);
        assert!(transfer_result.is_ok());
    }

    #[test]
    fn transfer_credits_amount_to_receiver() {
        let mut pallet = Pallet::new();

        pallet.set_balance("alice", 100);
        let transfer_result = pallet.transfer("alice", "bob", 50);

        assert_eq!(pallet.balance("bob"), 50);
        assert!(transfer_result.is_ok());
    }

    #[test]
    fn transfer_fails_if_sender_has_insufficient_balance() {
        let mut pallet = Pallet::new();

        pallet.set_balance("alice", 10);
        let transfer_result = pallet.transfer("alice", "bob", 100);
        assert_eq!(transfer_result, Err("Not enough balance"));
    }

    #[test]
    fn transfer_fails_if_receiver_balance_overflows() {
        let mut pallet = Pallet::new();

        pallet.set_balance("alice", 100);
        pallet.set_balance("bob", u128::MAX);

        let transfer_result = pallet.transfer("alice", "bob", 50);
        assert_eq!(transfer_result, Err("Balance overflow"));
    }
}
