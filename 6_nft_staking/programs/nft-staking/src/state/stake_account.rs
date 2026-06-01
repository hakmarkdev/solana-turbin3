use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    // the MPL Core asset pubkey
    pub mint: Pubkey,
    pub staked_at: i64,
    pub last_claimed_at: i64,
    pub bump: u8,
}
