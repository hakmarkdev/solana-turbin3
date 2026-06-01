use super::constants::SECONDS_PER_DAY;

pub fn days_elapsed(now: i64, since: i64) -> u32 {
    ((now - since) / SECONDS_PER_DAY) as u32
}

pub fn accrued_points(days: u32, points_per_stake: u8) -> u32 {
    days.checked_mul(points_per_stake as u32).unwrap()
}

pub fn reward_token_amount(points: u32, decimals: u8) -> u64 {
    points as u64 * 10u64.pow(decimals as u32)
}
