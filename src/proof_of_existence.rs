use crate::support::DispatchResult;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("No claim found for this content")?;
        if *claim_owner != caller {
            return Err("You are not the owner of this claim");
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

    pub fn get_claim(&self, content: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(&content)
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();

        let _ = poe.create_claim("alice", "Hello World");
        assert_eq!(poe.get_claim(&"Hello World"), Some(&"alice"));
        assert_eq!(poe.create_claim("alice", "Hello World"), Err("Claim already exists"));

        let res = poe.revoke_claim("bob", "Hello World");
        assert_eq!(res, Err("You are not the owner of this claim"));

        let res = poe.create_claim("bob", "Hello World");
        assert_eq!(res, Err("Claim already exists"));

        let res = poe.revoke_claim("alice", "non-existent claim");
        assert_eq!(res, Err("No claim found for this content"));

        let res = poe.revoke_claim("alice", "Hello World");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claim(&"Hello World"), None);
    }
}
