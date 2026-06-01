use anchor_lang::prelude::Pubkey;
use nft_staking::utils::constants::{
    COLLECTION_INFO_SEED, CONFIG_SEED, REWARDS_SEED, STAKE_SEED, USER_SEED,
};

fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG_SEED], &nft_staking::id())
}

#[test]
fn config_seed_is_deterministic() {
    let (pda, _bump) = config_pda();
    assert_eq!(config_pda().0, pda);
}

#[test]
fn reward_mint_seed_is_derived_from_config() {
    let (config, _) = config_pda();
    let (rewards, _) =
        Pubkey::find_program_address(&[REWARDS_SEED, config.as_ref()], &nft_staking::id());
    assert_ne!(rewards, config);
}

#[test]
fn user_account_seed_is_per_user() {
    let alice = Pubkey::new_unique();
    let bob = Pubkey::new_unique();
    let (a, _) = Pubkey::find_program_address(&[USER_SEED, alice.as_ref()], &nft_staking::id());
    let (b, _) = Pubkey::find_program_address(&[USER_SEED, bob.as_ref()], &nft_staking::id());
    assert_ne!(a, b, "different users must get different user accounts");
}

#[test]
fn collection_info_seed_is_per_collection() {
    let collection = Pubkey::new_unique();
    let (info, _) = Pubkey::find_program_address(
        &[COLLECTION_INFO_SEED, collection.as_ref()],
        &nft_staking::id(),
    );
    let (other, _) = Pubkey::find_program_address(
        &[COLLECTION_INFO_SEED, Pubkey::new_unique().as_ref()],
        &nft_staking::id(),
    );
    assert_ne!(info, other);
}

#[test]
fn stake_account_seed_is_per_config_and_asset() {
    let (config, _) = config_pda();
    let asset = Pubkey::new_unique();
    let (stake, _) = Pubkey::find_program_address(
        &[STAKE_SEED, config.as_ref(), asset.as_ref()],
        &nft_staking::id(),
    );

    let other_asset = Pubkey::new_unique();
    let (other, _) = Pubkey::find_program_address(
        &[STAKE_SEED, config.as_ref(), other_asset.as_ref()],
        &nft_staking::id(),
    );
    assert_ne!(stake, other);
}

#[test]
fn distinct_seed_namespaces_do_not_collide() {
    let (config, _) = config_pda();
    let key = Pubkey::new_unique();
    let (user, _) = Pubkey::find_program_address(&[USER_SEED, key.as_ref()], &nft_staking::id());
    let (rewards, _) =
        Pubkey::find_program_address(&[REWARDS_SEED, config.as_ref()], &nft_staking::id());
    assert_ne!(config, user);
    assert_ne!(config, rewards);
    assert_ne!(user, rewards);
}
