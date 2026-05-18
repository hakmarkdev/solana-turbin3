use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::instructions::release_vault;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker,
        has_one = mint_a,
    )]
    pub escrow: Account<'info, Escrow>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let seed_bytes = self.escrow.seed.to_le_bytes();
        let bump_bytes = [self.escrow.bump];
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"escrow", self.maker.key.as_ref(), &seed_bytes, &bump_bytes]];

        let amount = self.vault.amount;
        let decimals = self.mint_a.decimals;

        release_vault(
            self.token_program.key(),
            self.vault.to_account_info(),
            self.mint_a.to_account_info(),
            self.maker_ata_a.to_account_info(),
            self.escrow.to_account_info(),
            self.maker.to_account_info(),
            amount,
            decimals,
            signer_seeds,
        )
    }
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    ctx.accounts.withdraw_and_close_vault()
}
