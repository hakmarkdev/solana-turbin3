mod common;

use common::*;
use solana_signer::Signer;

#[test]
fn withdraw_rejects_insufficient_funds() {
    let mut env = TestEnv::new();
    let admin = env.payer.pubkey();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();
    let ix = env.withdraw_ix("m", &admin, 1);
    assert!(env.send(&[ix], &[&env.payer.insecure_clone()]).is_err());
}

#[test]
fn withdraw_rejects_non_admin() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();
    let attacker = env.funded_keypair(SOL);
    let ix = env.withdraw_ix("m", &attacker.pubkey(), 0);
    assert!(env.send(&[ix], &[&attacker]).is_err());
}
