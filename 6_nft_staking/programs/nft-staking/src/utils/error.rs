use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Max stake limit reached")]
    MaxStakeReached,
    #[msg("Freeze period has not passed yet")]
    FreezePeriodNotPassed,
    #[msg("You are not the owner of this staked NFT")]
    NotOwner,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Collection already initialized")]
    CollectionAlreadyInitialized,
    #[msg("Collection not initialized")]
    CollectionNotInitialized,
    #[msg("Asset already initialized")]
    AssetAlreadyInitialized,
}
