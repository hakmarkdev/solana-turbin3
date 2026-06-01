use anchor_lang::prelude::*;

use crate::constants::MARKETPLACE_SEED;
use crate::error::MarketplaceError;
use crate::state::{is_valid_fee, Marketplace};

#[derive(Accounts)]
pub struct UpdateMarketplace<'info> {
    #[account(
        constraint = admin.key() == marketplace.admin @ MarketplaceError::Unauthorized,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE_SEED, marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
}

impl<'info> UpdateMarketplace<'info> {
    pub fn update(&mut self, new_fee: Option<u16>) -> Result<()> {
        if let Some(fee) = new_fee {
            require!(is_valid_fee(fee), MarketplaceError::InvalidFee);
            self.marketplace.fee = fee;
            msg!("Marketplace fee updated to {} bps", fee);
        }
        Ok(())
    }
}
