use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::instructions::release_vault;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    fn transfer_to_maker(&mut self) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.key(),
                TransferChecked {
                    from: self.taker_ata_b.to_account_info(),
                    mint: self.mint_b.to_account_info(),
                    to: self.maker_ata_b.to_account_info(),
                    authority: self.taker.to_account_info(),
                },
            ),
            self.escrow.receive,
            self.mint_b.decimals,
        )
    }

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
            self.taker_ata_a.to_account_info(),
            self.escrow.to_account_info(),
            self.maker.to_account_info(),
            amount,
            decimals,
            signer_seeds,
        )
    }
}

pub fn handler(ctx: Context<Take>) -> Result<()> {
    ctx.accounts.transfer_to_maker()?;
    ctx.accounts.withdraw_and_close_vault()
}
