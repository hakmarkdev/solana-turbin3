use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Vault already has funds deposited")]
    VaultAlreadyFunded,
    #[msg("Deposit amount must exceed rent minimum")]
    AmountTooLow,
    #[msg("Vault has no funds to withdraw")]
    VaultEmpty,
}
