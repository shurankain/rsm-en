mod balances;

fn main() {
    println!("Hello, world!");
    let pallet = balances::Pallet::new();
}

#[test]
fn test_balance() {
    let mut pallet = balances::Pallet::new();
    assert_eq!(pallet.balance("Alice"), 0);
    pallet.set_balance(&"Ann".to_string(), 100);
    assert_eq!(pallet.balance("Ann"), 100);
    assert_eq!(pallet.balance("Alice"), 0);
}