pub mod error;

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use error::VaultError;

const VAULT_SEED: &[u8] = b"vault";

declare_id!("4t1QxMJWjmVAt5XFckPXPqxxwpjcovbm6TzYVpkviMr6");

#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAccounts>, amount: u64) -> Result<()> {
        require_eq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::VaultAlreadyFunded
        );
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::AmountTooLow
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.key(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAccounts>) -> Result<()> {
        let balance = ctx.accounts.vault.lamports();
        require_neq!(balance, 0, VaultError::VaultEmpty);

        let seeds = &[
            VAULT_SEED,
            ctx.accounts.signer.key.as_ref(),
            &[ctx.bumps.vault],
        ];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.key(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&seeds[..]],
            ),
            balance,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_SEED, signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
