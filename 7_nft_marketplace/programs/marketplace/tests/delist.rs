mod common;

use common::*;
use solana_signer::Signer;

#[test]
fn delist_success_returns_nft() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let collection = env.create_collection();
    let maker = env.funded_keypair(10 * SOL);
    let nft = env.mint_verified_nft(&maker, &collection);
    env.send(
        &[env.list_ix("m", &maker.pubkey(), &nft, &collection, SOL)],
        &[&maker],
    )
    .unwrap();

    env.send(&[env.delist_ix("m", &maker.pubkey(), &nft)], &[&maker])
        .unwrap();

    assert_eq!(env.token_amount(&ata(&maker.pubkey(), &nft)), Some(1));
    assert!(env
        .svm
        .get_account(&listing_pda(&marketplace_pda("m"), &nft))
        .map(|a| a.data.is_empty())
        .unwrap_or(true));
}

#[test]
fn delist_rejects_non_maker() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let collection = env.create_collection();
    let maker = env.funded_keypair(10 * SOL);
    let nft = env.mint_verified_nft(&maker, &collection);
    env.send(
        &[env.list_ix("m", &maker.pubkey(), &nft, &collection, SOL)],
        &[&maker],
    )
    .unwrap();

    let attacker = env.funded_keypair(SOL);
    let ix = env.delist_ix("m", &attacker.pubkey(), &nft);
    assert!(env.send(&[ix], &[&attacker]).is_err());
}
