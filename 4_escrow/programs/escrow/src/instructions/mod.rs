pub mod make;
pub mod refund;
pub mod take;

pub use make::*;
pub use refund::*;
pub use take::*;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, transfer_checked, CloseAccount, TransferChecked};

pub(crate) fn release_vault<'info>(
    token_program_id: Pubkey,
    vault: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    recipient: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    rent_destination: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    transfer_checked(
        CpiContext::new_with_signer(
            token_program_id,
            TransferChecked {
                from: vault.clone(),
                mint,
                to: recipient,
                authority: authority.clone(),
            },
            signer_seeds,
        ),
        amount,
        decimals,
    )?;

    close_account(CpiContext::new_with_signer(
        token_program_id,
        CloseAccount {
            account: vault,
            destination: rent_destination,
            authority,
        },
        signer_seeds,
    ))
}
