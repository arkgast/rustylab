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
}
