use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const GOVERNANCE_PROGRAM_ID: &str = "GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw";

#[program]
pub mod solana_validator_dao {

    use solana_program::program::{invoke, invoke_signed};

    use super::*;

    pub fn initialize(
        ctx: Context<InitalizeDAOStakeAccount>,
        lamports: u64,
        lockup_epoch_height: u64,
        custodian: Option<Pubkey>,
    ) -> Result<()> {
        let governance_id = &ctx.accounts.governance_id.to_account_info();
        let governance_program = &ctx.accounts.governance_program.to_account_info();
        let program_id = ctx.program_id;
        let stake_program = &ctx.accounts.stake_program.to_account_info();

        // check stake program id
        assert_eq!(stake_program.key(), solana_program::stake::program::ID);
        // check governance program id
        assert_eq!(
            governance_program.key().to_string(),
            GOVERNANCE_PROGRAM_ID.to_string()
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
        let lockup = match custodian {
            Some(c) => solana_program::stake::state::Lockup {
                epoch: lockup_epoch_height,
                unix_timestamp: 0,
                custodian: c,
            },
            None => solana_program::stake::state::Lockup {
                epoch: lockup_epoch_height,
                unix_timestamp: 0,
                custodian: Pubkey::default(),
            },
        };

        let (dao_stake_account_pda, stake_account_bump) = Pubkey::find_program_address(
            &[
                b"validator_dao_stake_account",
                governance_id.key().as_ref(),
                native_treasury.key().as_ref(),
                governance_program.key().as_ref(),
            ],
            program_id,
        );
        let signer_seeds = &[
            b"validator_dao_stake_account",
            governance_id.key.as_ref(),
            native_treasury.key.as_ref(),
            governance_program.key.as_ref(),
            &[stake_account_bump],
        ];

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
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
