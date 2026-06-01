use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};
use crate::utils::{
    constants::{CONFIG_SEED, REWARDS_SEED, STAKE_SEED, USER_SEED},
    utils::{accrued_points, days_elapsed, reward_token_amount},
};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [REWARDS_SEED, config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub reward_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let points = self.user_account.points;
        let signer_seeds: &[&[&[u8]]] = &[&[CONFIG_SEED, &[self.config.bump]]];

        let mint_amount = reward_token_amount(points, self.reward_mint.decimals);

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.key(),
                MintTo {
                    to: self.rewards_ata.to_account_info(),
                    mint: self.reward_mint.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                signer_seeds,
            ),
            mint_amount,
        )?;

        self.user_account.points = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimStakeRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [STAKE_SEED, config.key().as_ref(), stake_account.mint.as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == user.key()
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [REWARDS_SEED, config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub reward_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimStakeRewards<'info> {
    pub fn claim_stake_rewards(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Time-based points accrued since the last claim
        let days = days_elapsed(now, self.stake_account.last_claimed_at);
        let new_points = accrued_points(days, self.config.points_per_stake);

        // Combine newly accrued points with any previously accumulated points
        let total_points = self.user_account.points.checked_add(new_points).unwrap();

        // Advance the watermark so these points cannot be claimed again
        self.stake_account.last_claimed_at = now;

        let signer_seeds: &[&[&[u8]]] = &[&[CONFIG_SEED, &[self.config.bump]]];
        let mint_amount = reward_token_amount(total_points, self.reward_mint.decimals);

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.key(),
                MintTo {
                    to: self.rewards_ata.to_account_info(),
                    mint: self.reward_mint.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                signer_seeds,
            ),
            mint_amount,
        )?;

        // Reset accumulated points
        self.user_account.points = 0;

        Ok(())
    }
}
