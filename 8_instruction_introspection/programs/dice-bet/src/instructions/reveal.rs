use crate::error::DiceError;
use crate::state::RevealArgs;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Reveal<'info> {
    pub house_authority: Signer<'info>,
}

pub fn handler(_ctx: Context<Reveal>, args: RevealArgs) -> Result<()> {
    require!(args.roll <= 1, DiceError::InvalidRoll);
    msg!("reveal: bet={} roll={}", args.bet, args.roll);
    Ok(())
}
