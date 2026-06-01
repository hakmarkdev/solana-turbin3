use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, mint_to, transfer_checked, CloseAccount, Mint, MintTo, TokenAccount,
        TokenInterface, TransferChecked,
    },
};

use crate::constants::{MARKETPLACE_SEED, REWARDS_SEED, REWARD_AMOUNT, TREASURY_SEED};
use crate::error::MarketplaceError;
use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    /// CHECK: validated by the constraint against `listing.maker`.
    #[account(
        mut,
        constraint = maker.key() == listing.maker @ MarketplaceError::InvalidMaker,
    )]
    pub maker: UncheckedAccount<'info>,

    #[account(
        seeds = [MARKETPLACE_SEED, marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    pub maker_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = rewards_mint,
        associated_token::authority = taker,
    )]
    pub taker_reward_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
        close = maker,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        constraint = vault.amount == 1 @ MarketplaceError::EmptyVault,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [REWARDS_SEED, marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
    )]
    pub rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Purchase<'info> {
    pub fn pay(&mut self) -> Result<()> {
        let (fee_amount, maker_amount) = self.marketplace.split_payment(self.listing.price)?;

        if fee_amount > 0 {
            let cpi_ctx = CpiContext::new(
                self.system_program.key(),
                Transfer {
                    from: self.taker.to_account_info(),
                    to: self.treasury.to_account_info(),
                },
            );
            transfer(cpi_ctx, fee_amount)?;
        }

        if maker_amount > 0 {
            let cpi_ctx = CpiContext::new(
                self.system_program.key(),
                Transfer {
                    from: self.taker.to_account_info(),
                    to: self.maker.to_account_info(),
                },
            );
            transfer(cpi_ctx, maker_amount)?;
        }

        msg!(
            "Paid {} to maker, {} fee to treasury",
            maker_amount,
            fee_amount
        );
        Ok(())
    }

    pub fn receive_nft(&mut self) -> Result<()> {
        let marketplace_key = self.marketplace.key();
        let maker_mint_key = self.maker_mint.key();
        let seeds: &[&[u8]] = &[
            marketplace_key.as_ref(),
            maker_mint_key.as_ref(),
            std::slice::from_ref(&self.listing.bump),
        ];
        let signer_seeds = &[seeds];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.key(), cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;
        Ok(())
    }

    pub fn receive_rewards(&mut self) -> Result<()> {
        let name = self.marketplace.name.clone();
        let seeds: &[&[u8]] = &[
            MARKETPLACE_SEED,
            name.as_bytes(),
            std::slice::from_ref(&self.marketplace.bump),
        ];
        let signer_seeds = &[seeds];

        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.taker_reward_ata.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };
        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.key(), cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, REWARD_AMOUNT)?;
        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let marketplace_key = self.marketplace.key();
        let maker_mint_key = self.maker_mint.key();
        let seeds: &[&[u8]] = &[
            marketplace_key.as_ref(),
            maker_mint_key.as_ref(),
            std::slice::from_ref(&self.listing.bump),
        ];
        let signer_seeds = &[seeds];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.key(), cpi_accounts, signer_seeds);
        close_account(cpi_ctx)?;
        Ok(())
    }
}
