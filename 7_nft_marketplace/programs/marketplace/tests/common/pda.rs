use solana_pubkey::Pubkey;

use super::programs::{ata_program, metadata_program, token_program};

pub fn marketplace_pda(name: &str) -> Pubkey {
    Pubkey::find_program_address(&[b"marketplace", name.as_bytes()], &marketplace::ID).0
}

pub fn treasury_pda(marketplace: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"treasury", marketplace.as_ref()], &marketplace::ID).0
}

pub fn rewards_pda(marketplace: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"rewards", marketplace.as_ref()], &marketplace::ID).0
}

pub fn listing_pda(marketplace: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[marketplace.as_ref(), mint.as_ref()], &marketplace::ID).0
}

pub fn metadata_pda(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"metadata", metadata_program().as_ref(), mint.as_ref()],
        &metadata_program(),
    )
    .0
}

pub fn master_edition_pda(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"metadata",
            metadata_program().as_ref(),
            mint.as_ref(),
            b"edition",
        ],
        &metadata_program(),
    )
    .0
}

pub fn ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[owner.as_ref(), token_program().as_ref(), mint.as_ref()],
        &ata_program(),
    )
    .0
}
