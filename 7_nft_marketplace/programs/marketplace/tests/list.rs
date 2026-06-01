mod common;

use common::*;
use marketplace::state::Listing;
use solana_signer::Signer;

#[test]
fn list_success_escrows_nft() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let collection = env.create_collection();
    let maker = env.funded_keypair(10 * SOL);
    let nft = env.mint_verified_nft(&maker, &collection);

    let ix = env.list_ix("m", &maker.pubkey(), &nft, &collection, 2 * SOL);
    env.send(&[ix], &[&maker]).unwrap();

    let listing: Listing = env.account(&listing_pda(&marketplace_pda("m"), &nft));
    assert_eq!(listing.maker, maker.pubkey());
    assert_eq!(listing.maker_mint, nft);
    assert_eq!(listing.price, 2 * SOL);

    let vault = ata(&listing_pda(&marketplace_pda("m"), &nft), &nft);
    assert_eq!(env.token_amount(&vault), Some(1));
    assert_eq!(env.token_amount(&ata(&maker.pubkey(), &nft)), Some(0));
}

#[test]
fn list_rejects_unverified_collection() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let collection = env.create_collection();
    let maker = env.funded_keypair(10 * SOL);
    let nft = env.mint_unverified_nft(&maker, &collection);

    let ix = env.list_ix("m", &maker.pubkey(), &nft, &collection, SOL);
    assert!(env.send(&[ix], &[&maker]).is_err());
}
