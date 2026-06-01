use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{AddPluginV1CpiBuilder, UpdateCollectionPluginV1CpiBuilder},
    types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority},
    ID as CORE_PROGRAM_ID,
};

use crate::{
    state::{CollectionInfo, StakeAccount, StakeConfig, UserAccount},
    utils::{
        constants::{COLLECTION_INFO_SEED, CONFIG_SEED, STAKE_SEED, USER_SEED},
        error::StakeError,
    },
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = asset.owner == &CORE_PROGRAM_ID,
        constraint = !asset.data_is_empty()
    )]
    /// CHECK: checked by core
    pub asset: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID,
        constraint = !collection.data_is_empty()
    )]
    /// CHECK: checked by core
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        payer = user,
        space = StakeAccount::DISCRIMINATOR.len() + StakeAccount::INIT_SPACE,
        seeds = [STAKE_SEED, config.key().as_ref(), asset.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [COLLECTION_INFO_SEED, collection.key().as_ref()],
        bump = collection_info.bump,
    )]
    pub collection_info: Account<'info, CollectionInfo>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: verified by address constraint
    pub core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_account.amount_staked < self.config.max_stake,
            StakeError::MaxStakeReached
        );

        let now = Clock::get()?.unix_timestamp;

        AddPluginV1CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.user.to_account_info())
            .authority(None)
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: true }))
            .init_authority(PluginAuthority::Address {
                address: self.stake_account.key(),
            })
            .invoke()?;

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.asset.key(),
            staked_at: now,
            last_claimed_at: now,
            bump: bumps.stake_account,
        });

        self.user_account.amount_staked += 1;

        let new_count = self.collection_info.staked_count.checked_add(1).unwrap();
        self.collection_info.staked_count = new_count;

        let collection_signer_seeds: &[&[&[u8]]] = &[&[
            COLLECTION_INFO_SEED,
            &self.collection.key().to_bytes(),
            &[self.collection_info.bump],
        ]];

        UpdateCollectionPluginV1CpiBuilder::new(&self.core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.user.to_account_info())
            .authority(Some(&self.collection_info.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::Attributes(Attributes {
                attribute_list: vec![Attribute {
                    key: "staked_count".to_string(),
                    value: new_count.to_string(),
                }],
            }))
            .invoke_signed(collection_signer_seeds)?;

        Ok(())
    }
}
