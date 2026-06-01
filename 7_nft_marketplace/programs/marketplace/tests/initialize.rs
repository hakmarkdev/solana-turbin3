mod common;

use common::*;
use marketplace::state::Marketplace;
use solana_signer::Signer;

#[test]
fn initialize_success() {
    let mut env = TestEnv::new();
    let ix = env.initialize_ix("my market", 250);
    env.send(&[ix], &[&env.payer.insecure_clone()]).unwrap();

    let m: Marketplace = env.account(&marketplace_pda("my market"));
    assert_eq!(m.admin, env.payer.pubkey());
    assert_eq!(m.fee, 250);
    assert_eq!(m.name, "my market");

    assert!(env
        .svm
        .get_account(&rewards_pda(&marketplace_pda("my market")))
        .is_some());
}

#[test]
fn initialize_rejects_fee_above_max() {
    let mut env = TestEnv::new();
    let ix = env.initialize_ix("market", 10_001);
    assert!(env.send(&[ix], &[&env.payer.insecure_clone()]).is_err());
}

#[test]
fn initialize_accepts_max_fee() {
    let mut env = TestEnv::new();
    let ix = env.initialize_ix("market", 10_000);
    assert!(env.send(&[ix], &[&env.payer.insecure_clone()]).is_ok());
}

#[test]
fn initialize_rejects_empty_name() {
    let mut env = TestEnv::new();
    let ix = env.initialize_ix("", 100);
    assert!(env.send(&[ix], &[&env.payer.insecure_clone()]).is_err());
}

#[test]
fn initialize_accepts_max_length_name() {
    let mut env = TestEnv::new();
    let name = "x".repeat(32);
    let ix = env.initialize_ix(&name, 100);
    assert!(env.send(&[ix], &[&env.payer.insecure_clone()]).is_ok());

    let m: Marketplace = env.account(&marketplace_pda(&name));
    assert_eq!(m.name, name);
}
