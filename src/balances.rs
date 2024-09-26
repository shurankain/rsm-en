use num::{CheckedAdd, CheckedSub, Zero};
use std::{collections::BTreeMap, fmt::Debug};

pub trait Config: crate::system::Config {
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // the ket is the wallet and the value is the balance
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(&mut self, from: T::AccountId,
                    to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let caller_balance = self.balance(&from);
        let to_balance = self.balance(&to);
        let new_caller_balance = caller_balance.checked_sub(&amount)
            .ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(&amount)
            .ok_or("Overflow")?;
        self.set_balance(&from, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::system;

    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn test_balance() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        const ALICE: &str = "Alice";
        const ANN: &str = "Ann";
        assert_eq!(pallet.balance(&ALICE.to_string()), 0);
        pallet.set_balance(&ANN.to_string(), 100);
        assert_eq!(pallet.balance(&ANN.to_string()), 100);
        assert_eq!(pallet.balance(&ALICE.to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        static ALICE: &str = "Alice";
        static BOB: &str = "Bob";
        pallet.set_balance(&ALICE.to_string(), 100);
        pallet.set_balance(&BOB.to_string(), 100);
        pallet.transfer(ALICE.to_string(), BOB.to_string(), 50).unwrap();
        assert_eq!(pallet.balance(&ALICE.to_string()), 50);
        assert_eq!(pallet.balance(&BOB.to_string()), 150);
    }

    #[test]
    fn transfer_balance_insufficient() {
        //creat new mutable super::Pallet<AccountId, Balance>
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        const ALICE: &str = "Alice";
        const BOB: &str = "Bob";
        pallet.set_balance(&ALICE.to_string(), 100);
        pallet.set_balance(&BOB.to_string(), 100);
        assert_eq!(pallet.transfer(ALICE.to_string(), BOB.to_string(), 150),
                   Err("Insufficient balance"));
    }
}
