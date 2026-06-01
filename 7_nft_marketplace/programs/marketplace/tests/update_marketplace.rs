mod common;

use common::*;
use marketplace::state::Marketplace;
use solana_signer::Signer;

#[test]
fn update_fee_success() {
    let mut env = TestEnv::new();
    let admin = env.payer.pubkey();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    env.send(
        &[env.update_ix("m", &admin, Some(500))],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let m: Marketplace = env.account(&marketplace_pda("m"));
    assert_eq!(m.fee, 500);
}

#[test]
fn update_fee_none_is_noop() {
    let mut env = TestEnv::new();
    let admin = env.payer.pubkey();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    env.send(
        &[env.update_ix("m", &admin, None)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let m: Marketplace = env.account(&marketplace_pda("m"));
    assert_eq!(m.fee, 250);
}

#[test]
fn update_fee_rejects_above_max() {
    let mut env = TestEnv::new();
    let admin = env.payer.pubkey();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();
    assert!(env
        .send(
            &[env.update_ix("m", &admin, Some(10_001))],
            &[&env.payer.insecure_clone()]
        )
        .is_err());
}

#[test]
fn update_fee_rejects_non_admin() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let attacker = env.funded_keypair(SOL);
    let ix = env.update_ix("m", &attacker.pubkey(), Some(0));
    assert!(env.send(&[ix], &[&attacker]).is_err());
}
