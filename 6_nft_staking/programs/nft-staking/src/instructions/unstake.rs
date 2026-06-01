use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{
        RemovePluginV1CpiBuilder, UpdateCollectionPluginV1CpiBuilder, UpdatePluginV1CpiBuilder,
    },
    types::{Attribute, Attributes, FreezeDelegate, Plugin, PluginType},
    ID as CORE_PROGRAM_ID,
};

use crate::{
    state::{CollectionInfo, StakeAccount, StakeConfig, UserAccount},
    utils::{
        constants::{COLLECTION_INFO_SEED, CONFIG_SEED, STAKE_SEED, USER_SEED},
        error::StakeError,
        utils::{accrued_points, days_elapsed},
    },
};

#[derive(Accounts)]
pub struct Unstake<'info> {
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
        mut,
        close = user,
        seeds = [STAKE_SEED, config.key().as_ref(), asset.key().as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == user.key() @ StakeError::NotOwner
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

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Enforce minimum staking period
        let days_staked = days_elapsed(now, self.stake_account.staked_at);
        require!(
            days_staked >= self.config.freeze_period,
            StakeError::FreezePeriodNotPassed
        );

        // Award any remaining unclaimed time based rewards since last_claimed_at
        let days_since_last_claim = days_elapsed(now, self.stake_account.last_claimed_at);
        let pending_points = accrued_points(days_since_last_claim, self.config.points_per_stake);
        self.user_account.points = self
            .user_account
            .points
            .checked_add(pending_points)
            .unwrap();
        self.user_account.amount_staked = self.user_account.amount_staked.saturating_sub(1);

        // Capture values needed for CPIs before stake_account is borrowed by close
        let config_key = self.config.key();
        let asset_key = self.asset.key();
        let stake_bump = self.stake_account.bump;

        let stake_signer_seeds: &[&[&[u8]]] = &[&[
            STAKE_SEED,
            &config_key.to_bytes(),
            &asset_key.to_bytes(),
            &[stake_bump],
        ]];

        // Thaw the NFT
        UpdatePluginV1CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.user.to_account_info())
            .authority(Some(&self.stake_account.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .invoke_signed(stake_signer_seeds)?;

        // Remove the FreezeDelegate plugin
        RemovePluginV1CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.user.to_account_info())
            .authority(None)
            .system_program(&self.system_program.to_account_info())
            .plugin_type(PluginType::FreezeDelegate)
            .invoke()?;

        let new_count = self.collection_info.staked_count.saturating_sub(1);
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
