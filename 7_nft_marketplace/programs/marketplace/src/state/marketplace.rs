use anchor_lang::prelude::*;

use crate::constants::{MAX_FEE_BPS, MAX_NAME_LEN};
use crate::error::MarketplaceError;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    #[max_len(MAX_NAME_LEN)]
    pub name: String,
}

impl Marketplace {
    pub fn fee_amount(&self, price: u64) -> Result<u64> {
        calculate_fee(price, self.fee)
    }

    pub fn split_payment(&self, price: u64) -> Result<(u64, u64)> {
        split_payment(price, self.fee)
    }
}

#[inline]
pub fn is_valid_fee(fee: u16) -> bool {
    fee <= MAX_FEE_BPS
}

#[inline]
pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= MAX_NAME_LEN
}

pub fn calculate_fee(price: u64, fee_bps: u16) -> Result<u64> {
    let fee = (price as u128)
        .checked_mul(fee_bps as u128)
        .ok_or(MarketplaceError::MathOverflow)?
        .checked_div(MAX_FEE_BPS as u128)
        .ok_or(MarketplaceError::MathOverflow)?;
    Ok(u64::try_from(fee).map_err(|_| MarketplaceError::MathOverflow)?)
}

pub fn split_payment(price: u64, fee_bps: u16) -> Result<(u64, u64)> {
    let fee = calculate_fee(price, fee_bps)?;
    let maker_amount = price
        .checked_sub(fee)
        .ok_or(MarketplaceError::MathOverflow)?;
    Ok((fee, maker_amount))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_zero_when_fee_bps_zero() {
        assert_eq!(calculate_fee(1_000_000, 0).unwrap(), 0);
    }

    #[test]
    fn fee_full_price_when_max_bps() {
        assert_eq!(calculate_fee(1_000_000, MAX_FEE_BPS).unwrap(), 1_000_000);
    }

    #[test]
    fn fee_two_and_a_half_percent() {
        assert_eq!(calculate_fee(1_000_000, 250).unwrap(), 25_000);
    }

    #[test]
    fn fee_rounds_down() {
        // 1 * 250 / 10000 = 0.025 -> floor 0
        assert_eq!(calculate_fee(1, 250).unwrap(), 0);
        // 39 * 250 / 10000 = 0.975 -> floor 0
        assert_eq!(calculate_fee(39, 250).unwrap(), 0);
        // 40 * 250 / 10000 = 1.0 -> 1
        assert_eq!(calculate_fee(40, 250).unwrap(), 1);
    }

    #[test]
    fn fee_zero_price() {
        assert_eq!(calculate_fee(0, 250).unwrap(), 0);
    }

    #[test]
    fn fee_no_overflow_at_u64_max() {
        let fee = calculate_fee(u64::MAX, MAX_FEE_BPS).unwrap();
        assert_eq!(fee, u64::MAX);
    }

    #[test]
    fn split_sums_to_price() {
        let (fee, maker) = split_payment(1_000_000, 250).unwrap();
        assert_eq!(fee, 25_000);
        assert_eq!(maker, 975_000);
        assert_eq!(fee + maker, 1_000_000);
    }

    #[test]
    fn split_full_fee_leaves_maker_zero() {
        let (fee, maker) = split_payment(500, MAX_FEE_BPS).unwrap();
        assert_eq!(fee, 500);
        assert_eq!(maker, 0);
    }

    #[test]
    fn split_zero_fee_gives_all_to_maker() {
        let (fee, maker) = split_payment(777, 0).unwrap();
        assert_eq!(fee, 0);
        assert_eq!(maker, 777);
    }

    #[test]
    fn validators() {
        assert!(is_valid_fee(0));
        assert!(is_valid_fee(MAX_FEE_BPS));
        assert!(!is_valid_fee(MAX_FEE_BPS + 1));

        assert!(is_valid_name("a"));
        assert!(is_valid_name(&"x".repeat(MAX_NAME_LEN)));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name(&"x".repeat(MAX_NAME_LEN + 1)));
    }

    #[test]
    fn marketplace_methods_match_free_fns() {
        let m = Marketplace {
            admin: Pubkey::new_unique(),
            fee: 300,
            bump: 1,
            treasury_bump: 2,
            rewards_bump: 3,
            name: "test".to_string(),
        };
        assert_eq!(m.fee_amount(1_000_000).unwrap(), 30_000);
        assert_eq!(m.split_payment(1_000_000).unwrap(), (30_000, 970_000));
    }

    #[test]
    fn init_space_is_stable() {
        assert_eq!(
            Marketplace::INIT_SPACE,
            32 + 2 + 1 + 1 + 1 + 4 + MAX_NAME_LEN
        );
    }
}
