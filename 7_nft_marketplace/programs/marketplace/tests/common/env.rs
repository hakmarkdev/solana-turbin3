use litesvm::LiteSVM;
use mpl_token_metadata::instructions::{
    CreateMasterEditionV3Builder, CreateMetadataAccountV3Builder, VerifyCollectionV1Builder,
};
use mpl_token_metadata::types::{Collection, DataV2};
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

use super::*;

pub struct TestEnv {
    pub svm: LiteSVM,
    pub payer: Keypair,
}

impl TestEnv {
    pub fn new() -> Self {
        let mut svm = LiteSVM::new();

        let program_bytes = include_bytes!("../../../../target/deploy/marketplace.so");
        svm.add_program(marketplace::ID, program_bytes).unwrap();

        let metadata_bytes = include_bytes!("../../../../tests/fixtures/mpl_token_metadata.so");
        svm.add_program(metadata_program(), metadata_bytes).unwrap();

        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 100 * SOL).unwrap();

        Self { svm, payer }
    }

    pub fn funded_keypair(&mut self, lamports: u64) -> Keypair {
        let kp = Keypair::new();
        self.svm.airdrop(&kp.pubkey(), lamports).unwrap();
        kp
    }

    pub fn send(
        &mut self,
        ixs: &[Instruction],
        signers: &[&Keypair],
    ) -> litesvm::types::TransactionResult {
        let payer = signers[0].pubkey();
        let blockhash = self.svm.latest_blockhash();
        let msg = Message::new_with_blockhash(ixs, Some(&payer), &blockhash);

        let mut seen: Vec<Pubkey> = Vec::new();
        let mut unique: Vec<&Keypair> = Vec::new();
        for s in signers {
            let pk = s.pubkey();
            if !seen.contains(&pk) {
                seen.push(pk);
                unique.push(*s);
            }
        }

        let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), unique.as_slice())
            .unwrap();
        self.svm.send_transaction(tx)
    }

    pub fn balance(&self, key: &Pubkey) -> u64 {
        self.svm.get_balance(key).unwrap_or(0)
    }

    pub fn account<T: anchor_lang::AccountDeserialize>(&self, key: &Pubkey) -> T {
        let acc = self.svm.get_account(key).expect("account not found");
        T::try_deserialize(&mut acc.data.as_slice()).expect("failed to deserialize")
    }

    pub fn token_amount(&self, token_account: &Pubkey) -> Option<u64> {
        let acc = self.svm.get_account(token_account)?;
        if acc.data.len() < 72 {
            return None;
        }

        Some(u64::from_le_bytes(acc.data[64..72].try_into().unwrap()))
    }

    fn create_mint_and_mint_to(&mut self, owner: &Keypair, amount: u64) -> Keypair {
        let mint = Keypair::new();
        let rent = self.svm.minimum_balance_for_rent_exemption(MINT_LEN);

        let create_mint = solana_system_interface::instruction::create_account(
            &self.payer.pubkey(),
            &mint.pubkey(),
            rent,
            MINT_LEN as u64,
            &token_program(),
        );
        let init_mint = initialize_mint2_ix(&mint.pubkey(), &owner.pubkey(), 0);
        self.send(
            &[create_mint, init_mint],
            &[&self.payer.insecure_clone(), &mint],
        )
        .unwrap();

        let create_ata = create_ata_ix(&self.payer.pubkey(), &owner.pubkey(), &mint.pubkey());
        let mint_to = mint_to_ix(
            &mint.pubkey(),
            &ata(&owner.pubkey(), &mint.pubkey()),
            &owner.pubkey(),
            amount,
        );
        self.send(
            &[create_ata, mint_to],
            &[&self.payer.insecure_clone(), &owner.insecure_clone()],
        )
        .unwrap();

        mint
    }

    fn create_metadata(&mut self, mint: &Pubkey, authority: &Keypair, collection: Option<Pubkey>) {
        let ix = CreateMetadataAccountV3Builder::new()
            .metadata(metadata_pda(mint))
            .mint(*mint)
            .mint_authority(authority.pubkey())
            .payer(self.payer.pubkey())
            .update_authority(authority.pubkey(), false)
            .data(DataV2 {
                name: "Test NFT".to_string(),
                symbol: "TST".to_string(),
                uri: "https://example.com/nft.json".to_string(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: collection.map(|key| Collection {
                    verified: false,
                    key,
                }),
                uses: None,
            })
            .is_mutable(true)
            .instruction();
        self.send(
            &[ix],
            &[&self.payer.insecure_clone(), &authority.insecure_clone()],
        )
        .unwrap();
    }

    fn create_master_edition(&mut self, mint: &Pubkey, authority: &Keypair) {
        let ix = CreateMasterEditionV3Builder::new()
            .edition(master_edition_pda(mint))
            .mint(*mint)
            .update_authority(authority.pubkey())
            .mint_authority(authority.pubkey())
            .payer(self.payer.pubkey())
            .metadata(metadata_pda(mint))
            .max_supply(0)
            .instruction();
        self.send(
            &[ix],
            &[&self.payer.insecure_clone(), &authority.insecure_clone()],
        )
        .unwrap();
    }

    pub fn create_collection(&mut self) -> Pubkey {
        let authority = self.payer.insecure_clone();
        let mint = self.create_mint_and_mint_to(&authority, 1);
        self.create_metadata(&mint.pubkey(), &authority, None);
        self.create_master_edition(&mint.pubkey(), &authority);
        mint.pubkey()
    }

    pub fn mint_verified_nft(&mut self, owner: &Keypair, collection: &Pubkey) -> Pubkey {
        let mint = self.create_mint_and_mint_to(owner, 1);
        self.create_metadata(&mint.pubkey(), owner, Some(*collection));
        self.create_master_edition(&mint.pubkey(), owner);
        self.verify_collection(&mint.pubkey(), collection);
        mint.pubkey()
    }

    pub fn mint_unverified_nft(&mut self, owner: &Keypair, collection: &Pubkey) -> Pubkey {
        let mint = self.create_mint_and_mint_to(owner, 1);
        self.create_metadata(&mint.pubkey(), owner, Some(*collection));
        self.create_master_edition(&mint.pubkey(), owner);
        mint.pubkey()
    }

    fn verify_collection(&mut self, mint: &Pubkey, collection: &Pubkey) {
        let ix = VerifyCollectionV1Builder::new()
            .authority(self.payer.pubkey())
            .metadata(metadata_pda(mint))
            .collection_mint(*collection)
            .collection_metadata(Some(metadata_pda(collection)))
            .collection_master_edition(Some(master_edition_pda(collection)))
            .system_program(system_program())
            .sysvar_instructions(instructions_sysvar())
            .instruction();
        self.send(&[ix], &[&self.payer.insecure_clone()]).unwrap();
    }
}
