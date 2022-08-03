use crate::*;

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
        seeds = [b"validator_dao_stake_account", 
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
