use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("Choice must be 0 or 1")]
    InvalidChoice,
    #[msg("Bet amount must be greater than zero")]
    InvalidAmount,
    #[msg("resolve_bet must be preceded by a reveal instruction")]
    MissingReveal,
    #[msg("The preceding instruction is not this program")]
    BadRevealProgram,
    #[msg("The preceding instruction is not a reveal (bad discriminator or payload)")]
    BadRevealDiscriminator,
    #[msg("Reveal does not authorize this bet")]
    RevealBetMismatch,
    #[msg("Reveal was not signed by the house authority")]
    HouseNotSigner,
    #[msg("Reveal roll must be 0 or 1")]
    InvalidRoll,
    #[msg("House vault has insufficient funds for payout")]
    InsufficientVault,
}
