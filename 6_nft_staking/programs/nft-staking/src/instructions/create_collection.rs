use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder,
    types::{Attribute, Attributes, Plugin, PluginAuthorityPair},
    ID as CORE_PROGRAM_ID,
};

use crate::state::CollectionInfo;
use crate::utils::{constants::COLLECTION_INFO_SEED, error::StakeError};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub uri: String,
    pub nft_name: String,
    pub nft_uri: String,
}

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = collection.data_is_empty() @ StakeError::CollectionAlreadyInitialized
    )]
    pub collection: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = CollectionInfo::DISCRIMINATOR.len() + CollectionInfo::INIT_SPACE,
        seeds = [COLLECTION_INFO_SEED, collection.key().as_ref()],
        bump
    )]
    pub collection_info: Account<'info, CollectionInfo>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: verified by address constraint
    pub core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateCollection<'info> {
    pub fn create_collection(
        &mut self,
        args: CreateCollectionArgs,
        bumps: &CreateCollectionBumps,
    ) -> Result<()> {
        self.collection_info.set_inner(CollectionInfo {
            collection: self.collection.key(),
            authority: self.authority.key(),
            name: args.name.clone(),
            uri: args.uri.clone(),
            nft_name: args.nft_name.clone(),
            nft_uri: args.nft_uri.clone(),
            staked_count: 0,
            bump: bumps.collection_info,
        });

        let signer_seeds: &[&[&[u8]]] = &[&[
            COLLECTION_INFO_SEED,
            &self.collection.key().to_bytes(),
            &[bumps.collection_info],
        ]];

        CreateCollectionV2CpiBuilder::new(&self.core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.authority.to_account_info())
            .update_authority(Some(&self.collection_info.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![Attribute {
                        key: "staked_count".to_string(),
                        value: "0".to_string(),
                    }],
                }),
                authority: None,
            }])
            .external_plugin_adapters(vec![])
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
