use anchor_lang::prelude::*;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("Fee must be between 0 and 10000 basis points")]
    InvalidFee,
    #[msg("Pool is locked")]
    PoolLocked,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Slippage limit exceeded")]
    SlippageLimitExceeded,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Invalid precision")]
    InvalidPrecision,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Zero balance in pool")]
    ZeroBalance,
}

impl From<CurveError> for AmmError {
    fn from(e: CurveError) -> Self {
        match e {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFee,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
            CurveError::SlippageLimitExceeded => AmmError::SlippageLimitExceeded,
        }
    }
}
