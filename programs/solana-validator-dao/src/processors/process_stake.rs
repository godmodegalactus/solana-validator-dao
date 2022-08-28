use crate::*;
use solana_program::program::{invoke, invoke_signed};

pub fn process(ctx: Context<InitalizeDAOStakeAccount>, seed: u8, lamports: u64) -> Result<()> {
    let governance_id = &ctx.accounts.governance_id.to_account_info();
    let governance_program = &ctx.accounts.governance_program.to_account_info();
    let program_id = ctx.program_id;
    let stake_program = &ctx.accounts.stake_program.to_account_info();

    // check stake program id
    assert_eq!(stake_program.key(), solana_program::stake::program::ID);
    assert_eq!(
        ctx.accounts.system_program.key(),
        solana_program::system_program::ID
    );
    assert_eq!(
        ctx.accounts.clock_program.key(),
        solana_program::sysvar::clock::id()
    );
    assert_eq!(
        ctx.accounts.rent_program.key(),
        solana_program::sysvar::rent::id()
    );
    assert_eq!(
        ctx.accounts.stake_history.key(),
        solana_program::sysvar::stake_history::id()
    );
    assert_eq!(
        ctx.accounts.stake_config.key(),
        solana_program::stake::config::id()
    );
    // check governance key match
    spl_governance::state::governance::assert_is_valid_governance(
        &governance_program.key(),
        governance_id,
    )?;

    let native_treasury = &ctx.accounts.governance_native_treasury_account;
    assert_eq!(
        native_treasury.key(),
        spl_governance::state::native_treasury::get_native_treasury_address(
            governance_program.key,
            governance_id.key,
        )
    );

    let autorized = solana_program::stake::state::Authorized {
        staker: native_treasury.key(),
        withdrawer: native_treasury.key(),
    };

    let lockup = solana_program::stake::state::Lockup::default();

    let program_seeds = &[
        b"validator_dao_stake_account",
        governance_id.key.as_ref(),
        native_treasury.key.as_ref(),
        governance_program.key.as_ref(),
        ctx.accounts.validator_vote_key.key.as_ref(),
        &[seed],
    ];

    let (dao_stake_account_pda, stake_account_bump) =
        Pubkey::find_program_address(program_seeds, program_id);

    assert_eq!(dao_stake_account_pda, ctx.accounts.dao_stake_account.key());
    let stake_intialize_instructions = solana_program::stake::instruction::create_account(
        native_treasury.key,
        &dao_stake_account_pda,
        &autorized,
        &lockup,
        lamports,
    );
    assert!(stake_intialize_instructions.len() == 2);
    let create_account_instruction = &stake_intialize_instructions[0];
    let initailize_stake_account_instruction = &stake_intialize_instructions[1];

    let signer_seeds = &[
        b"validator_dao_stake_account",
        governance_id.key.as_ref(),
        native_treasury.key.as_ref(),
        governance_program.key.as_ref(),
        ctx.accounts.validator_vote_key.key.as_ref(),
        &[seed],
        &[stake_account_bump],
    ];
    invoke_signed(
        create_account_instruction,
        &[
            native_treasury.to_account_info().clone(),
            ctx.accounts.dao_stake_account.to_account_info().clone(),
        ],
        &[signer_seeds],
    )?;

    invoke(
        initailize_stake_account_instruction,
        &[
            ctx.accounts.dao_stake_account.to_account_info().clone(),
            ctx.accounts.rent_program.to_account_info(),
        ],
    )?;

    let delegate_instruction = solana_program::stake::instruction::delegate_stake(
        &dao_stake_account_pda,
        native_treasury.key,
        ctx.accounts.validator_vote_key.key,
    );
    invoke(
        &delegate_instruction,
        &[
            ctx.accounts.dao_stake_account.to_account_info().clone(),
            ctx.accounts.validator_vote_key.clone(),
            ctx.accounts.clock_program.to_account_info().clone(),
            ctx.accounts.stake_history.to_account_info().clone(),
            ctx.accounts.stake_config.clone(),
            ctx.accounts
                .governance_native_treasury_account
                .to_account_info()
                .clone(),
        ],
    )?;
    Ok(())
}
