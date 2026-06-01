//! Program-wide constants.

/// PDA seed for the singleton staking config
pub const CONFIG_SEED: &[u8] = b"config";
/// PDA seed for the reward mint
pub const REWARDS_SEED: &[u8] = b"rewards";
/// PDA seed for a per-wallet user account
pub const USER_SEED: &[u8] = b"user";
/// PDA seed for a per-collection info account
pub const COLLECTION_INFO_SEED: &[u8] = b"collection_info";
/// PDA seed for a per-asset stake account
pub const STAKE_SEED: &[u8] = b"stake";
/// Seconds in one day – staking rewards accrue per whole day.
pub const SECONDS_PER_DAY: i64 = 86_400;
