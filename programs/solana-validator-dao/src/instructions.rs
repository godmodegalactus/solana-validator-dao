use std::mem::size_of;

use crate::*;
use anchor_spl::token::{Mint, TokenAccount};
use states::*;
const VALIDATOR_DAO_STAKE_ACCOUNT_SEEDS : &[u8] = b"validator_dao_stake_account";
const VALIDATOR_PROVIDER_SEEDS : &[u8] = b"validator_provider";
const GOVERNANCE_PROVIDER_SEEDS : &[u8] = b"governance_provider";
const GOVERNANCE_CONTRACT_SEEDS : &[u8] = b"governance_contract";

#[derive(Accounts)]
#[instruction(seed : u8)]
pub struct InitalizeDAOStakeAccount<'info> {
    /// CHECK: governance id
    pub governance_id: AccountInfo<'info>,
    /// CHECK: native treasury account
    #[account(mut)]
    pub governance_native_treasury_account: Signer<'info>,
    /// CHECK: stake account created for dao
    #[account(mut,
        seeds = [VALIDATOR_DAO_STAKE_ACCOUNT_SEEDS, 
            governance_id.key().as_ref(), 
            governance_native_treasury_account.key().as_ref(), 
            governance_program.key().as_ref(),
            validator_vote_key.key().as_ref(),
            &[seed]],
        bump,
    )]
    pub dao_stake_account: AccountInfo<'info>,
    // payer
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: validator vote key
    pub validator_vote_key: AccountInfo<'info>,
    /// CHECK: stake config account
    pub stake_config : AccountInfo<'info>,
    /// CHECK: governance program
    pub governance_program: AccountInfo<'info>,
    /// CHECK: stake program
    pub stake_program: AccountInfo<'info>,
    // system program
    pub system_program: Program<'info, System>,
    // rent program
    pub rent_program: Sysvar<'info, Rent>,
    // clock program
    pub clock_program: Sysvar<'info, Clock>,
    // stake history
    pub stake_history: Sysvar<'info, StakeHistory>,
}

#[derive(Accounts)]
pub struct RegisterValidatorServiceProvider<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [VALIDATOR_PROVIDER_SEEDS, owner.key().as_ref()],
        bump,
        space = 8 +  size_of::<ValidatorProvider>(),
        payer = owner,
    )]
    pub provider_data : Box<Account<'info, ValidatorProvider>>,

    pub payment_mint : Box<Account<'info, Mint>>,

    // system program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddRegisteredProviderToGovernance<'info> {
    /// CHECK: governance ai
    #[account(
        constraint = governance_ai.owner.eq( &GOVERNANCE_PROGRAM_ID ),
    )]
    pub governance_ai : AccountInfo<'info>,
    // signer for governance / should govenernance native treasury
    #[account(mut)]
    pub governance_native_treasury : Signer<'info>,
    // provider data
    #[account(
        mut,
        constraint = provider_data.owner == *program_id,
        constraint = provider_data.is_valid()
    )]
    pub provider_data: Box<Account<'info, ValidatorProvider>>,

    #[account(
        init,
        seeds = [GOVERNANCE_PROVIDER_SEEDS, governance_ai.key().as_ref(), provider_data.key().as_ref()],
        bump,
        space = 8 + size_of::<GovernanceProvider>(),
        payer = governance_native_treasury,
    )]
    pub governance_provider_data : Box<Account<'info, GovernanceProvider>>,
    // system program
    pub system_program: Program<'info, System>,
    pub clock : Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct CreateGovernanceContract<'info> {
    /// CHECK: governance ai / will be checked for valid governance
    #[account(
        constraint = governance_ai.owner.eq( &GOVERNANCE_PROGRAM_ID)
    )]
    pub governance_ai : AccountInfo<'info>,

    #[account(
        mut,
        constraint = provider_data.owner == *program_id,
        constraint = provider_data.is_valid()
    )]
    pub provider_data: Box<Account<'info, ValidatorProvider>>,

    #[account(mut,
        seeds = [GOVERNANCE_PROVIDER_SEEDS, governance_ai.key().as_ref(), provider_data.key().as_ref()],
        bump,
        constraint = governance_provider_data.to_account_info().owner == program_id,
        constraint = governance_provider_data.is_valid()
    )]
    pub governance_provider_data : Box<Account<'info, GovernanceProvider>>,

    #[account(
        init,
        seeds = [GOVERNANCE_CONTRACT_SEEDS, governance_ai.key().as_ref(), provider_data.key().as_ref(), governance_provider_data.key().as_ref()],
        bump,
        space = 8 + size_of::<GovernanceContract>(),
        payer = payer,
    )]
    pub governance_contract : Box<Account<'info, GovernanceContract>>,

    #[account()]
    pub payment_mint : Box<Account<'info, Mint>>,

    #[account(
        constraint = token_account.mint == payment_mint.key(),
    )]
    pub token_account : Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer : Signer<'info>,
    pub system_program: Program<'info, System>,
}