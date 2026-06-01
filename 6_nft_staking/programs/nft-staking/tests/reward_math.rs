use nft_staking::utils::constants::SECONDS_PER_DAY;
use nft_staking::utils::utils::{accrued_points, days_elapsed, reward_token_amount};

#[test]
fn one_day_is_86_400_seconds() {
    assert_eq!(SECONDS_PER_DAY, 86_400);
}

#[test]
fn days_elapsed_truncates_partial_days() {
    let start = 1_000_000;
    assert_eq!(days_elapsed(start, start), 0);
    assert_eq!(days_elapsed(start + SECONDS_PER_DAY - 1, start), 0);
    assert_eq!(days_elapsed(start + SECONDS_PER_DAY, start), 1);
    assert_eq!(days_elapsed(start + 5 * SECONDS_PER_DAY + 123, start), 5);
}

#[test]
fn accrued_points_scale_with_days_and_rate() {
    assert_eq!(accrued_points(0, 10), 0);
    assert_eq!(accrued_points(5, 10), 50);
    assert_eq!(accrued_points(7, 1), 7);
    assert_eq!(accrued_points(3, 0), 0);
}

#[test]
fn reward_token_amount_applies_decimals() {
    assert_eq!(reward_token_amount(0, 6), 0);
    assert_eq!(reward_token_amount(1, 6), 1_000_000);
    assert_eq!(reward_token_amount(50, 6), 50_000_000);
    assert_eq!(reward_token_amount(3, 0), 3);
}

#[test]
fn full_accrual_pipeline_matches_instruction_logic() {
    let staked_at = 1_700_000_000;
    let now = staked_at + 9 * SECONDS_PER_DAY + SECONDS_PER_DAY / 2;
    let points_per_stake = 10u8;
    let decimals = 6u8;

    let days = days_elapsed(now, staked_at);
    assert_eq!(days, 9, "half a day is dropped");

    let points = accrued_points(days, points_per_stake);
    assert_eq!(points, 90);

    let tokens = reward_token_amount(points, decimals);
    assert_eq!(tokens, 90_000_000);
}

#[test]
fn advancing_the_watermark_prevents_double_counting() {
    let staked_at = 0i64;
    let rate = 10u8;

    let first_claim = 4 * SECONDS_PER_DAY;
    let first = accrued_points(days_elapsed(first_claim, staked_at), rate);
    assert_eq!(first, 40);

    let unstake_at = first_claim + 3 * SECONDS_PER_DAY;
    let pending = accrued_points(days_elapsed(unstake_at, first_claim), rate);
    assert_eq!(pending, 30);
    assert_eq!(first + pending, accrued_points(7, rate));
}
