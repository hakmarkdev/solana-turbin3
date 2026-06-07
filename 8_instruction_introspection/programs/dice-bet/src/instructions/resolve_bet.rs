use crate::error::DiceError;
use crate::state::{Bet, House, RevealArgs};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::Discriminator;
use solana_instructions_sysvar::{
    load_current_index_checked, load_instruction_at_checked, ID as INSTRUCTIONS_SYSVAR_ID,
};

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    /// CHECK: Only used as a lamport destination; pinned to `bet.player` via `has_one`
    #[account(mut)]
    pub player: SystemAccount<'info>,

    #[account(seeds = [b"house"], bump = house.bump)]
    pub house: Account<'info, House>,

    #[account(mut, seeds = [b"vault"], bump = house.vault_bump)]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close = player,
        has_one = player @ DiceError::RevealBetMismatch,
        seeds = [b"bet", bet.player.as_ref(), &bet.seed.to_le_bytes()],
        bump = bet.bump,
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK: Validated by address; read via the sysvar instructions helpers
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    pub instructions: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ResolveBet>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();
    let current = load_current_index_checked(&ixs)? as usize;
    require!(current >= 1, DiceError::MissingReveal);
    let reveal_ix = load_instruction_at_checked(current - 1, &ixs)?;

    require_keys_eq!(reveal_ix.program_id, crate::ID, DiceError::BadRevealProgram);
    let disc = crate::instruction::Reveal::DISCRIMINATOR;
    require!(
        reveal_ix.data.len() >= disc.len() && &reveal_ix.data[..disc.len()] == disc,
        DiceError::BadRevealDiscriminator
    );

    let args = RevealArgs::try_from_slice(&reveal_ix.data[disc.len()..])
        .map_err(|_| error!(DiceError::BadRevealDiscriminator))?;
    require!(args.roll <= 1, DiceError::InvalidRoll);
    require_keys_eq!(
        args.bet,
        ctx.accounts.bet.key(),
        DiceError::RevealBetMismatch
    );

    let house_meta = reveal_ix
        .accounts
        .first()
        .ok_or(error!(DiceError::HouseNotSigner))?;
    require!(house_meta.is_signer, DiceError::HouseNotSigner);
    require_keys_eq!(
        house_meta.pubkey,
        ctx.accounts.house.authority,
        DiceError::HouseNotSigner
    );

    let bet = &ctx.accounts.bet;
    if args.roll == bet.choice {
        let payout = bet.amount.checked_mul(2).unwrap();
        require!(
            ctx.accounts.vault.lamports() >= payout,
            DiceError::InsufficientVault
        );

        let vault_seeds: &[&[u8]] = &[
            b"vault",
            core::slice::from_ref(&ctx.accounts.house.vault_bump),
        ];
        let signer = &[vault_seeds];
        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.key(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.player.to_account_info(),
            },
            signer,
        );
        transfer(cpi, payout)?;
        msg!("Bet WON: paid {} lamports to {}", payout, bet.player);
    } else {
        msg!("Bet LOST: house keeps {} lamports", bet.amount);
    }
    Ok(())
}
