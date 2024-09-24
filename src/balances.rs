use std::collections::BTreeMap;
use std::fmt::Debug;
use num::{CheckedAdd, CheckedSub, Zero, Unsigned};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    // the ket is the wallet and the value is the balance
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where AccountId : Ord + Clone,
      Balance : CheckedAdd + CheckedSub + Zero + Copy + Unsigned {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(&mut self, from: AccountId,
                    to: AccountId, amount: Balance) -> Result<(), &'static str> {
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

    mod types {
        pub type AccountId = String;
        pub type Balance = u128;
    }
    #[test]
    fn test_balance() {
        let mut pallet = super::Pallet::<types::AccountId, types::Balance>::new();
        const ALICE: &str = "Alice";
        const ANN: &str = "Ann";
        assert_eq!(pallet.balance(&ALICE.to_string()), 0);
        pallet.set_balance(&ANN.to_string(), 100);
        assert_eq!(pallet.balance(&ANN.to_string()), 100);
        assert_eq!(pallet.balance(&ALICE.to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet = super::Pallet::<types::AccountId, types::Balance>::new();
        static  ALICE: &str  = "Alice";
        static  BOB: &str  = "Bob";
        pallet.set_balance(&ALICE.to_string(), 100);
        pallet.set_balance(&BOB.to_string(), 100);
        pallet.transfer(ALICE.to_string(), BOB.to_string(), 50).unwrap();
        assert_eq!(pallet.balance(&ALICE.to_string()), 50);
        assert_eq!(pallet.balance(&BOB.to_string()), 150);
    }

    #[test]
    fn transfer_balance_insufficient() {
        //creat new mutable super::Pallet<AccountId, Balance>
        let mut pallet = super::Pallet::<types::AccountId, types::Balance>::new();
        const ALICE: &str = "Alice";
        const BOB: &str = "Bob";
        pallet.set_balance(&ALICE.to_string(), 100);
        pallet.set_balance(&BOB.to_string(), 100);
        assert_eq!(pallet.transfer(ALICE.to_string(), BOB.to_string(), 150),
                   Err("Insufficient balance"));
    }

}
