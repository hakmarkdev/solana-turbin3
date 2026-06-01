pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("8xTAMja3yDejXWub6bAHKgB5mDy175esAgiUBjzRnKHu");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user_account(&ctx.bumps)
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts.create_collection(args, &ctx.bumps)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        ctx.accounts.mint_nft()
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }

    // Claim time-based rewards while the NFT is still staked
    pub fn claim_stake_rewards(ctx: Context<ClaimStakeRewards>) -> Result<()> {
        ctx.accounts.claim_stake_rewards()
    }
}
