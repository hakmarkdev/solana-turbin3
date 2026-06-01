use anchor_lang::{InstructionData, ToAccountMetas};
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_signer::Signer;

use super::*;

impl TestEnv {
    pub fn initialize_ix(&self, name: &str, fee: u16) -> Instruction {
        let marketplace = marketplace_pda(name);
        Instruction {
            program_id: marketplace::ID,
            accounts: marketplace::accounts::Initialize {
                admin: self.payer.pubkey(),
                marketplace,
                treasury: treasury_pda(&marketplace),
                rewards_mint: rewards_pda(&marketplace),
                system_program: system_program(),
                token_program: token_program(),
            }
            .to_account_metas(None),
            data: marketplace::instruction::Initialize {
                name: name.to_string(),
                fee,
            }
            .data(),
        }
    }

    pub fn list_ix(
        &self,
        name: &str,
        maker: &Pubkey,
        maker_mint: &Pubkey,
        collection_mint: &Pubkey,
        price: u64,
    ) -> Instruction {
        let marketplace = marketplace_pda(name);
        let listing = listing_pda(&marketplace, maker_mint);
        Instruction {
            program_id: marketplace::ID,
            accounts: marketplace::accounts::List {
                maker: *maker,
                marketplace,
                maker_mint: *maker_mint,
                maker_ata: ata(maker, maker_mint),
                vault: ata(&listing, maker_mint),
                listing,
                collection_mint: *collection_mint,
                metadata: metadata_pda(maker_mint),
                master_edition: master_edition_pda(maker_mint),
                metadata_program: metadata_program(),
                associated_token_program: ata_program(),
                system_program: system_program(),
                token_program: token_program(),
            }
            .to_account_metas(None),
            data: marketplace::instruction::List { price }.data(),
        }
    }

    pub fn delist_ix(&self, name: &str, maker: &Pubkey, maker_mint: &Pubkey) -> Instruction {
        let marketplace = marketplace_pda(name);
        let listing = listing_pda(&marketplace, maker_mint);
        Instruction {
            program_id: marketplace::ID,
            accounts: marketplace::accounts::Delist {
                maker: *maker,
                marketplace,
                maker_mint: *maker_mint,
                maker_ata: ata(maker, maker_mint),
                vault: ata(&listing, maker_mint),
                listing,
                associated_token_program: ata_program(),
                system_program: system_program(),
                token_program: token_program(),
            }
            .to_account_metas(None),
            data: marketplace::instruction::Delist {}.data(),
        }
    }

    pub fn purchase_ix(
        &self,
        name: &str,
        taker: &Pubkey,
        maker: &Pubkey,
        maker_mint: &Pubkey,
    ) -> Instruction {
        let marketplace = marketplace_pda(name);
        let listing = listing_pda(&marketplace, maker_mint);
        let rewards_mint = rewards_pda(&marketplace);
        Instruction {
            program_id: marketplace::ID,
            accounts: marketplace::accounts::Purchase {
                taker: *taker,
                maker: *maker,
                marketplace,
                maker_mint: *maker_mint,
                taker_ata: ata(taker, maker_mint),
                taker_reward_ata: ata(taker, &rewards_mint),
                listing,
                vault: ata(&listing, maker_mint),
                treasury: treasury_pda(&marketplace),
                rewards_mint,
                associated_token_program: ata_program(),
                system_program: system_program(),
                token_program: token_program(),
            }
            .to_account_metas(None),
            data: marketplace::instruction::Purchase {}.data(),
        }
    }

    pub fn update_ix(&self, name: &str, admin: &Pubkey, new_fee: Option<u16>) -> Instruction {
        let marketplace = marketplace_pda(name);
        Instruction {
            program_id: marketplace::ID,
            accounts: marketplace::accounts::UpdateMarketplace {
                admin: *admin,
                marketplace,
            }
            .to_account_metas(None),
            data: marketplace::instruction::UpdateMarketplace { new_fee }.data(),
        }
    }

    pub fn withdraw_ix(&self, name: &str, admin: &Pubkey, amount: u64) -> Instruction {
        let marketplace = marketplace_pda(name);
        Instruction {
            program_id: marketplace::ID,
            accounts: marketplace::accounts::WithdrawFees {
                admin: *admin,
                marketplace,
                treasury: treasury_pda(&marketplace),
                system_program: system_program(),
            }
            .to_account_metas(None),
            data: marketplace::instruction::WithdrawFees { amount }.data(),
        }
    }
}
