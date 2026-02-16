use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn nonce(&self, who: &str) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }

    pub fn inc_nonce(&mut self, who: &str) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.to_string(), nonce + 1);
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
