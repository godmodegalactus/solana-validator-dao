use crate::*;

#[derive(Accounts)]
pub struct InitalizeDAOStakeAccount<'info> {
    /// CHECK: governance id
    pub governance_id: UncheckedAccount<'info>,
    /// CHECK: native treasury account
    #[account(mut)]
    pub governance_native_treasury_account: Signer<'info>,
    /// CHECK: stake account created for dao
    #[account(mut,
        seeds = [b"validator_dao_stake_account", governance_id.key().as_ref(), governance_native_treasury_account.key().as_ref(), governance_program.key().as_ref()],
        bump,
    )]
    pub dao_stake_account: UncheckedAccount<'info>,
    // payer
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: governance program
    pub governance_program: UncheckedAccount<'info>,
    /// CHECK: stake program
    pub stake_program: UncheckedAccount<'info>,
    // system program
    pub system_program: Program<'info, System>,
    // rent program
    pub rent_program: Sysvar<'info, Rent>,
}
