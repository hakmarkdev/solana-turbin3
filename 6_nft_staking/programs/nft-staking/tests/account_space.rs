use anchor_lang::{Discriminator, Space};
use nft_staking::state::{CollectionInfo, StakeAccount, StakeConfig, UserAccount};

const DISC: usize = 8;

#[test]
fn stake_config_layout() {
    // points_per_stake(1) + max_stake(1) + freeze_period(4) + rewards_bump(1) + bump(1)
    assert_eq!(StakeConfig::INIT_SPACE, 1 + 1 + 4 + 1 + 1);
    assert_eq!(StakeConfig::DISCRIMINATOR.len(), DISC);
}

#[test]
fn user_account_layout() {
    // points(4) + amount_staked(1) + bump(1)
    assert_eq!(UserAccount::INIT_SPACE, 4 + 1 + 1);
}

#[test]
fn stake_account_layout() {
    // owner(32) + mint(32) + staked_at(8) + last_claimed_at(8) + bump(1)
    assert_eq!(StakeAccount::INIT_SPACE, 32 + 32 + 8 + 8 + 1);
}

#[test]
fn collection_info_layout() {
    // collection(32) + authority(32)
    //   + name(4+32) + uri(4+200) + nft_name(4+32) + nft_uri(4+200)
    //   + staked_count(4) + bump(1)
    let expected = 32 + 32 + (4 + 32) + (4 + 200) + (4 + 32) + (4 + 200) + 4 + 1;
    assert_eq!(CollectionInfo::INIT_SPACE, expected);
}

#[test]
fn on_chain_space_includes_discriminator() {
    assert_eq!(
        StakeConfig::DISCRIMINATOR.len() + StakeConfig::INIT_SPACE,
        DISC + 8
    );
    assert_eq!(
        UserAccount::DISCRIMINATOR.len() + UserAccount::INIT_SPACE,
        DISC + 6
    );
    assert_eq!(
        StakeAccount::DISCRIMINATOR.len() + StakeAccount::INIT_SPACE,
        DISC + 81
    );
}
