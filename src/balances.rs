use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    // the ket is the wallet and the value is the balance
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &str) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: String,
                    to: String, amount: u128) -> Result<(), &'static str> {
        let caller_balance = self.balance(&from);
        let to_balance = self.balance(&to);
        let new_caller_balance = caller_balance.checked_sub(amount)
            .ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(amount)
            .ok_or("Overflow")?;
        self.set_balance(&from, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[test]
fn test_balance() {
    let mut pallet = Pallet::new();
    const ALICE: &str = "Alice";
    const ANN: &str = "Ann";
    assert_eq!(pallet.balance(ALICE), 0);
    pallet.set_balance(&ANN.to_string(), 100);
    assert_eq!(pallet.balance(ANN), 100);
    assert_eq!(pallet.balance(ALICE), 0);
}

#[test]
fn transfer_balance() {
    let mut pallet = Pallet::new();
    const ALICE: &str  = "Alice";
    const BOB: &str  = "Bob";
    pallet.set_balance(&ALICE.to_string(), 100);
    pallet.set_balance(&BOB.to_string(), 100);
    pallet.transfer(ALICE.to_string(), BOB.to_string(), 50).unwrap();
    assert_eq!(pallet.balance("Alice"), 50);
    assert_eq!(pallet.balance("Bob"), 150);
}

#[test]
fn transfer_balance_insufficient() {
    let mut pallet = Pallet::new();
    const ALICE: &str = "Alice";
    const BOB: &str = "Bob";
    pallet.set_balance(&ALICE.to_string(), 100);
    pallet.set_balance(&BOB.to_string(), 100);
    assert_eq!(pallet.transfer(ALICE.to_string(), BOB.to_string(), 150),
               Err("Insufficient balance"));
}
