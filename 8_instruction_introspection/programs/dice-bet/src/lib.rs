#![allow(clippy::diverging_sub_expression)]

pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("EFC5qQPZhzV8AnVVxMSXNzECNKZrhCCeMdDyoje7HF9K");

#[program]
pub mod dice_bet {
    use super::*;

    pub fn initialize_house(ctx: Context<InitializeHouse>, bankroll: u64) -> Result<()> {
        instructions::initialize_house::handler(ctx, bankroll)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u64, amount: u64, choice: u8) -> Result<()> {
        instructions::place_bet::handler(ctx, seed, amount, choice)
    }

    pub fn reveal(ctx: Context<Reveal>, args: RevealArgs) -> Result<()> {
        instructions::reveal::handler(ctx, args)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>) -> Result<()> {
        instructions::resolve_bet::handler(ctx)
    }
}
