#[test]
fn program_id_matches_declared_id() {
    assert_eq!(
        nft_staking::id().to_string(),
        "8xTAMja3yDejXWub6bAHKgB5mDy175esAgiUBjzRnKHu"
    );
}
