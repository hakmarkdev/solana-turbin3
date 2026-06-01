mod common;

use anchor_lang::Space;
use common::*;
use marketplace::state::Listing;
use solana_signer::Signer;

#[test]
fn purchase_success_full_settlement() {
    let mut env = TestEnv::new();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let collection = env.create_collection();
    let maker = env.funded_keypair(10 * SOL);
    let nft = env.mint_verified_nft(&maker, &collection);
    let price = 4 * SOL;
    env.send(
        &[env.list_ix("m", &maker.pubkey(), &nft, &collection, price)],
        &[&maker],
    )
    .unwrap();

    let taker = env.funded_keypair(10 * SOL);
    let maker_before = env.balance(&maker.pubkey());
    let listing_rent = env
        .svm
        .minimum_balance_for_rent_exemption(8 + Listing::INIT_SPACE);

    env.send(
        &[env.purchase_ix("m", &taker.pubkey(), &maker.pubkey(), &nft)],
        &[&taker],
    )
    .unwrap();

    let expected_fee = price * 250 / 10_000;
    let expected_to_maker = price - expected_fee;

    let treasury = treasury_pda(&marketplace_pda("m"));
    assert_eq!(env.balance(&treasury), expected_fee);
    assert_eq!(
        env.balance(&maker.pubkey()),
        maker_before + expected_to_maker + listing_rent
    );

    assert_eq!(env.token_amount(&ata(&taker.pubkey(), &nft)), Some(1));

    let rewards_mint = rewards_pda(&marketplace_pda("m"));
    assert_eq!(
        env.token_amount(&ata(&taker.pubkey(), &rewards_mint)),
        Some(REWARD_AMOUNT)
    );

    assert!(env
        .svm
        .get_account(&listing_pda(&marketplace_pda("m"), &nft))
        .map(|a| a.data.is_empty())
        .unwrap_or(true));
}

#[test]
fn purchase_rejects_wrong_maker() {
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

    let taker = env.funded_keypair(10 * SOL);
    let wrong_maker = env.funded_keypair(SOL);
    let ix = env.purchase_ix("m", &taker.pubkey(), &wrong_maker.pubkey(), &nft);
    assert!(env.send(&[ix], &[&taker]).is_err());
}

#[test]
fn purchase_then_admin_withdraws_fee() {
    let mut env = TestEnv::new();
    let admin = env.payer.pubkey();
    env.send(
        &[env.initialize_ix("m", 250)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    let collection = env.create_collection();
    let maker = env.funded_keypair(10 * SOL);
    let nft = env.mint_verified_nft(&maker, &collection);
    let price = 4 * SOL;
    env.send(
        &[env.list_ix("m", &maker.pubkey(), &nft, &collection, price)],
        &[&maker],
    )
    .unwrap();

    let taker = env.funded_keypair(10 * SOL);
    env.send(
        &[env.purchase_ix("m", &taker.pubkey(), &maker.pubkey(), &nft)],
        &[&taker],
    )
    .unwrap();

    let fee = price * 250 / 10_000;
    let admin_before = env.balance(&admin);

    env.send(
        &[env.withdraw_ix("m", &admin, fee)],
        &[&env.payer.insecure_clone()],
    )
    .unwrap();

    assert_eq!(env.balance(&treasury_pda(&marketplace_pda("m"))), 0);
    assert_eq!(env.balance(&admin), admin_before + fee - TX_FEE);
}
