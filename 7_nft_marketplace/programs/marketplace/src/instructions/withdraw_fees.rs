use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::constants::{MARKETPLACE_SEED, TREASURY_SEED};
use crate::error::MarketplaceError;
use crate::state::Marketplace;

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        mut,
        constraint = admin.key() == marketplace.admin @ MarketplaceError::Unauthorized,
    )]
    pub admin: Signer<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFees<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(
            amount <= self.treasury.lamports(),
            MarketplaceError::InsufficientFunds
        );

        let marketplace_key = self.marketplace.key();
        let seeds: &[&[u8]] = &[
            TREASURY_SEED,
            marketplace_key.as_ref(),
            std::slice::from_ref(&self.marketplace.treasury_bump),
        ];
        let signer_seeds = &[seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.key(),
            Transfer {
                from: self.treasury.to_account_info(),
                to: self.admin.to_account_info(),
            },
            signer_seeds,
        );
        transfer(cpi_ctx, amount)?;

        msg!("Withdrew {} lamports to admin", amount);
        Ok(())
    }
}
