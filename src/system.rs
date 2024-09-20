use std::collections::BTreeMap;

// This is System Pallet
// Handles the low level state transition functions of the blockchain
// Contains block number (u32) and a map from the account to their nonce
pub struct Pallet {
    // block number
    block_number: u32,
    // map from account to their nonce (nonce is a number that is used only once, that counts the transactions of an account)
    // the key is the wallet and the value is the nonce (how many transactions have been made)
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
        // fails on overflow as expected behavior, because block number should never overflow
        self.block_number = self.block_number.checked_add(1).unwrap()
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }

    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let _pallet = super::Pallet::new();
        assert_eq!(_pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = super::Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut pallet = super::Pallet::new();
        let alice = "Alice".to_string();
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.get_nonce(&alice), 1);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.get_nonce(&alice), 2);
    }
}