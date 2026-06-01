use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    // Seller who created the listing
    pub maker: Pubkey,
    // Mint of the NFT being sold
    pub maker_mint: Pubkey,
    // Asking price in lamports
    pub price: u64,
    // Bump for the listing PDA
    pub bump: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_space_is_stable() {
        assert_eq!(Listing::INIT_SPACE, 32 + 32 + 8 + 1);
    }
}
