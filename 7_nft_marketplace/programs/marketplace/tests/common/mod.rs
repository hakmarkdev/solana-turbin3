#![allow(dead_code)]

mod env;
mod ix;
mod pda;
mod programs;

pub use env::*;
pub use pda::*;
pub use programs::*;

pub const MINT_LEN: usize = 82;
pub const SOL: u64 = 1_000_000_000;
pub const REWARD_AMOUNT: u64 = 10_000_000;
pub const TX_FEE: u64 = 5000;
