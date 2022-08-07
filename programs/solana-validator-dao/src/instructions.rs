use std::mem::size_of;

use crate::*;
use anchor_spl::token::Mint;
use states::*;
const VALIDATOR_DAO_STAKE_ACCOUNT_SEEDS : &[u8] = b"validator_dao_stake_account";
const VALIDATOR_PROVIDER_SEEDS : &[u8] = b"validator_provider";

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
        space = 8 + size_of::<ValidatorProvider>(),
        payer = owner,
    )]
    pub provider_data : Box<Account<'info, ValidatorProvider>>,

    pub payment_mint : Box<Account<'info, Mint>>,

    // system program
    pub system_program: Program<'info, System>,
}