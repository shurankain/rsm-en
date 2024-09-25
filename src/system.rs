use num::{One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Copy + AddAssign;
    type Nonce: Zero + One + Ord + Copy;
}

// This is System Pallet
// Handles the low level state transition functions of the blockchain
// Contains block number (u32) and a map from the account to their nonce
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // block number
    block_number: T::BlockNumber,
    // map from account to their nonce (nonce is a number that is used only once, that counts the transactions of an account)
    // the key is the wallet and the value is the nonce (how many transactions have been made)
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>
{
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }


    pub fn inc_block_number(&mut self) {
        // fails on overflow as expected behavior, because block number should never overflow
        self.block_number += T::BlockNumber::one()
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let _pallet: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(_pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        let alice = "Alice".to_string();
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.get_nonce(&alice), 1);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.get_nonce(&alice), 2);
    }
}