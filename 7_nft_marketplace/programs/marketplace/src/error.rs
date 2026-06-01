use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid marketplace fee. Must be between 0 and 10000 basis points.")]
    InvalidFee,

    #[msg("Invalid marketplace name. Must be non-empty and at most 32 characters.")]
    InvalidName,

    #[msg("Invalid price. Must be greater than 0.")]
    InvalidPrice,

    #[msg("Insufficient tokens in account.")]
    InsufficientTokens,

    #[msg("Invalid collection. NFT must belong to the specified collection.")]
    InvalidCollection,

    #[msg("Unverified collection. NFT collection must be verified.")]
    UnverifiedCollection,

    #[msg("Unauthorized access. Only the owner can perform this action.")]
    Unauthorized,

    #[msg("Vault is empty. No NFT to transfer.")]
    EmptyVault,

    #[msg("Invalid maker. Maker address doesn't match listing.")]
    InvalidMaker,

    #[msg("Insufficient funds for withdrawal.")]
    InsufficientFunds,

    #[msg("Mathematical overflow occurred.")]
    MathOverflow,
}
