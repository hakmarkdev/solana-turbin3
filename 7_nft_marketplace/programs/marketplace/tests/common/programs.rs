use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use super::pda::ata;

pub fn token_program() -> Pubkey {
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
}

pub fn ata_program() -> Pubkey {
    Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
}

pub fn metadata_program() -> Pubkey {
    mpl_token_metadata::ID
}

pub fn system_program() -> Pubkey {
    anchor_lang::solana_program::system_program::ID
}

pub fn instructions_sysvar() -> Pubkey {
    Pubkey::from_str_const("Sysvar1nstructions1111111111111111111111111")
}

pub fn initialize_mint2_ix(mint: &Pubkey, authority: &Pubkey, decimals: u8) -> Instruction {
    let mut data = Vec::with_capacity(67);
    data.push(20);
    data.push(decimals);
    data.extend_from_slice(authority.as_ref());

    data.push(1);
    data.extend_from_slice(authority.as_ref());
    Instruction {
        program_id: token_program(),
        accounts: vec![AccountMeta::new(*mint, false)],
        data,
    }
}

pub fn mint_to_ix(mint: &Pubkey, account: &Pubkey, authority: &Pubkey, amount: u64) -> Instruction {
    let mut data = Vec::with_capacity(9);
    data.push(7);
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: token_program(),
        accounts: vec![
            AccountMeta::new(*mint, false),
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

pub fn create_ata_ix(funder: &Pubkey, wallet: &Pubkey, mint: &Pubkey) -> Instruction {
    Instruction {
        program_id: ata_program(),
        accounts: vec![
            AccountMeta::new(*funder, true),
            AccountMeta::new(ata(wallet, mint), false),
            AccountMeta::new_readonly(*wallet, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(system_program(), false),
            AccountMeta::new_readonly(token_program(), false),
        ],
        data: vec![1],
    }
}
