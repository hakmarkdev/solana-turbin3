mod common;
use common::*;

#[test]
fn initialize_house_funds_vault_and_config() {
    let w = setup_with_house(10 * SOL);
    assert_eq!(
        balance(&w.svm, &w.vault),
        10 * SOL,
        "vault funded with bankroll"
    );

    let house = w.svm.get_account(&w.house).expect("house config created");
    assert_eq!(house.owner, pid(), "house config owned by program");
}
