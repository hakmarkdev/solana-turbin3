use anchor_lang::prelude::*;

// PDA seed prefix for the marketplace config account
#[constant]
pub const MARKETPLACE_SEED: &[u8] = b"marketplace";

// PDA seed prefix for the marketplace treasury
#[constant]
pub const TREASURY_SEED: &[u8] = b"treasury";

// PDA seed prefix for the rewards mint
#[constant]
pub const REWARDS_SEED: &[u8] = b"rewards";

// Maximum allowed length (in bytes) of a marketplace name
pub const MAX_NAME_LEN: usize = 32;

// Maximum marketplace fee expressed in basis points (100% = 10_000 bps)
pub const MAX_FEE_BPS: u16 = 10_000;

// Decimals used by the rewards mint
pub const REWARD_DECIMALS: u8 = 6;

// Amount of reward tokens (in base units) minted to a buyer on purchase
pub const REWARD_AMOUNT: u64 = 10u64.pow(REWARD_DECIMALS as u32) * 10;
