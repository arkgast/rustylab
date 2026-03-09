use std::{collections::BTreeMap, fmt::Debug};

use crate::{errors, system};

pub trait Config: system::Config {
    type Content: Debug + Ord + Clone;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    fn create_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content,
    ) -> Result<(), errors::ProofOfExistenceError> {
        if self.get_claim(&claim).is_some() {
            return Err(errors::ProofOfExistenceError::ClaimAlreadyExists);
        }
        self.claims.insert(claim.clone(), caller.clone());
        Ok(())
    }

    fn revoke_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content,
    ) -> Result<(), errors::ProofOfExistenceError> {
        let owner = self
            .claims
            .get(&claim)
            .ok_or(errors::ProofOfExistenceError::ClaimNotFound)?;
        if owner.clone() != caller {
            return Err(errors::ProofOfExistenceError::NotOwner);
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    fn get_claim(&self, content: &T::Content) -> Option<&T::AccountId> {
        if self.claims.contains_key(content) {
            return self.claims.get(content);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::types;

    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type Nonce = types::Nonce;
        type AccountId = types::AccountId;
        type BlockNumber = types::BlockNumber;
    }

    #[test]
    fn create_claim_works() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let content = &"content";

        pallet.create_claim(alice.clone(), "content").unwrap();
        assert_eq!(pallet.get_claim(content), Some(&alice));
    }

    #[test]
    fn create_claim_fails_when_content_already_claimed() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let content = &"content";

        pallet.create_claim(alice.clone(), content).unwrap();

        assert_eq!(
            pallet.create_claim(alice.clone(), content).unwrap_err(),
            errors::ProofOfExistenceError::ClaimAlreadyExists
        );
    }

    #[test]
    fn get_claim_returns_owner_for_existing_claim() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let content = &"content";
        pallet.create_claim(alice.clone(), content).unwrap();
        assert_eq!(pallet.get_claim(content).unwrap(), &alice);
    }

    #[test]
    fn get_claim_returns_none_for_nonexistent_claim() {
        let pallet = Pallet::<TestConfig>::new();
        assert_eq!(pallet.get_claim(&"content"), None);
    }

    #[test]
    fn revoke_claim_works_for_owner() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let content = &"content";

        pallet.create_claim(alice.clone(), content).unwrap();
        pallet.revoke_claim(alice.clone(), content).unwrap();

        assert_eq!(pallet.get_claim(content), None);
    }

    #[test]
    fn revoke_claim_fails_for_nonexistent_claim() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");

        assert_eq!(
            pallet.revoke_claim(alice.clone(), "content").unwrap_err(),
            errors::ProofOfExistenceError::ClaimNotFound
        );
    }

    #[test]
    fn revoke_claim_fails_for_non_owner() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let bob = String::from("bob");
        let content = "content";

        pallet.create_claim(alice.clone(), content).unwrap();

        assert_eq!(
            pallet.revoke_claim(bob.clone(), content).unwrap_err(),
            errors::ProofOfExistenceError::NotOwner
        );
    }

    #[test]
    fn create_claim_works_after_revoke() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let content = &"content";

        pallet.create_claim(alice.clone(), content).unwrap();
        pallet.revoke_claim(alice.clone(), content).unwrap();
        pallet.create_claim(alice.clone(), content).unwrap();

        assert_eq!(pallet.get_claim(content), Some(&alice));
    }
}
