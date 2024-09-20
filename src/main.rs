mod balances;
mod system;

fn main() {
    let mut balances = balances::Pallet::new();
    let mut pallet = system::Pallet::new();
}