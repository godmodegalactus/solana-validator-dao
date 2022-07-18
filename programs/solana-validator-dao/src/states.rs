use anchor_lang::prelude::*;
use anchor_spl::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    ValidatorData = 0,
    RewardsData = 1,
    GovernanceData = 2,
    UserData = 3,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
#[repr(C)]
/// Stores meta information about the `Account` on chain
pub struct MetaData {
    pub data_type: DataType,
    pub version: u8,
    pub is_initialized: bool,
    pub extra_info: [u8; 5],
}

/// Status of the stake account in the validator list, for accounting
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum StakeStatus {
    /// Stake account is active, there may be a transient stake as well
    Active,
    /// Only transient stake account exists, when a transient stake is
    /// deactivating during validator removal
    DeactivatingTransient,
    /// No more validator stake accounts exist, entry ready for removal during
    /// `UpdateStakePoolBalance`
    ReadyForRemoval,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct ValidatorData {
    // meta data
    meta_data : MetaData,
    // public identity of the validator
    validator_identity : Pubkey,
    // vote key of the validator
    validator_vote_key : Pubkey,

    // when were the validator details updated
    last_update_epoch : u64,
    // lamports staked
    staked_lamports : u64,
    // lamports to be staked at next epoch
    transient_staked_lamports : u64,
    // lamports unstaked
    unstaked_lamports : u64,
    // validator status
    stake_status : StakeStatus,
    
}

#[account()]
pub struct RewardsData {
    // meta data
    meta_data : MetaData,
    // index that will increase continuouly each time we get a reward after an epoch
    reward_lamport_index : u128,
    // rewards from last epoch
    rewards_from_last_epoch_lamports : u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
struct ValidatorList {
    max_validator_count : u8,
    validator_list : Vec<ValidatorData>,
}

#[account(zero_copy)]
pub struct GovernanceData {
    // metadata
    meta_data : MetaData,
    // list of accepted validators by dao
    validator_list : ValidatorList,

}



impl Default for StakeStatus {
    fn default() -> Self {
        Self::Active
    }
}
