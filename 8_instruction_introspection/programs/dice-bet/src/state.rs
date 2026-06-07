use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct House {
    pub authority: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u64,
    pub amount: u64,
    pub slot: u64,
    pub choice: u8,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RevealArgs {
    pub bet: Pubkey,
    pub roll: u8,
}
