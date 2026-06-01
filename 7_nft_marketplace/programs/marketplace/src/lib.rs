use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("7QBSkFhooeYTXKcHQxa7HjRGC7vnmkX1TXhBZdbwHsoJ");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        require!(is_valid_fee(fee), MarketplaceError::InvalidFee);
        require!(is_valid_name(&name), MarketplaceError::InvalidName);

        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        require!(price > 0, MarketplaceError::InvalidPrice);

        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?;
        ctx.accounts.receive_nft()?;
        ctx.accounts.receive_rewards()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn update_marketplace(ctx: Context<UpdateMarketplace>, new_fee: Option<u16>) -> Result<()> {
        ctx.accounts.update(new_fee)?;
        Ok(())
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
}
