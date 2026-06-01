use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::StakeConfig;
use crate::utils::constants::{CONFIG_SEED, REWARDS_SEED};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [CONFIG_SEED],
        bump,
        space = StakeConfig::DISCRIMINATOR.len() + StakeConfig::INIT_SPACE,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [REWARDS_SEED, config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub reward_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.reward_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}
