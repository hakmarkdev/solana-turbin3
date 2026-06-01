use nft_staking::utils::error::StakeError;

#[test]
fn error_codes_are_stable() {
    assert_eq!(u32::from(StakeError::MaxStakeReached), 6000);
    assert_eq!(u32::from(StakeError::FreezePeriodNotPassed), 6001);
    assert_eq!(u32::from(StakeError::NotOwner), 6002);
    assert_eq!(u32::from(StakeError::InvalidCollection), 6003);
    assert_eq!(u32::from(StakeError::CollectionAlreadyInitialized), 6004);
    assert_eq!(u32::from(StakeError::CollectionNotInitialized), 6005);
    assert_eq!(u32::from(StakeError::AssetAlreadyInitialized), 6006);
}
