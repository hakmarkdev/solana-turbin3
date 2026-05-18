use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Deposit amount must be greater than zero")]
    ZeroDepositAmount,
    #[msg("Receive amount must be greater than zero")]
    ZeroReceiveAmount,
}
