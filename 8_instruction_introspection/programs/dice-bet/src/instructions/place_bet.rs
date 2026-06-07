use crate::error::DiceError;
use crate::state::{Bet, House};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(seeds = [b"house"], bump = house.bump)]
    pub house: Account<'info, House>,

    #[account(mut, seeds = [b"vault"], bump = house.vault_bump)]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet", player.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlaceBet>, seed: u64, amount: u64, choice: u8) -> Result<()> {
    require!(amount > 0, DiceError::InvalidAmount);
    require!(choice <= 1, DiceError::InvalidChoice);

    let cpi = CpiContext::new(
        ctx.accounts.system_program.key(),
        Transfer {
            from: ctx.accounts.player.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    transfer(cpi, amount)?;

    ctx.accounts.bet.set_inner(Bet {
        player: ctx.accounts.player.key(),
        seed,
        amount,
        slot: Clock::get()?.slot,
        choice,
        bump: ctx.bumps.bet,
    });
    Ok(())
}
