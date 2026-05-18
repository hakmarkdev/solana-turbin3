use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use instructions::*;

declare_id!("E5sN9sH7AB2pqhSCMPD2jdzivQCMGazpF7rbKVVRARRT");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}
