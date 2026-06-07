use crate::state::House;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct InitializeHouse<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + House::INIT_SPACE,
        seeds = [b"house"],
        bump
    )]
    pub house: Account<'info, House>,

    // PDA that holds the bankroll and locked wagers
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeHouse>, bankroll: u64) -> Result<()> {
    let house = &mut ctx.accounts.house;
    house.authority = ctx.accounts.authority.key();
    house.bump = ctx.bumps.house;
    house.vault_bump = ctx.bumps.vault;

    let cpi = CpiContext::new(
        ctx.accounts.system_program.key(),
        Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    transfer(cpi, bankroll)?;
    Ok(())
}
