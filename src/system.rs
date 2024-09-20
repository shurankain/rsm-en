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
}