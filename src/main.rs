mod balances;
mod system;

pub struct Runtime {
    balances: balances::Pallet,
    system: system::Pallet,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {
    // genesis block
    let mut runtime = Runtime::new();

    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let charlie = "Charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);
    runtime.system.inc_nonce(&alice);

    let _ = runtime.balances
        .transfer(alice.clone(), bob.clone(), 50)
        .map_err(|e| print!("Error: {:?}", e));

    let _ = runtime.balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| print!("Error: {:?}", e));

}