use crate::*;

use anchor_spl::token;
use instructions::ExecuteContract;
use spl_governance::state::governance;

pub fn process(ctx: Context<ExecuteContract>) -> Result<()> {
    governance::assert_is_valid_governance(&GOVERNANCE_PROGRAM_ID, &ctx.accounts.governance_ai)?;
    let contract = &mut ctx.accounts.governance_contract;
    let clock = &ctx.accounts.clock;
    if contract.contract_start_timestamp < clock.unix_timestamp as u64 {
        return Err(errors::ValidatorDaoErrors::ContractNotYetStarted.into());
    }
    if !contract.has_signed_by_provider {
        return Err(errors::ValidatorDaoErrors::ContractNotSignedByProvider.into());
    }
    contract.executed = true;

    let transfer = token::Transfer {
        from: ctx.accounts.token_account.to_account_info().clone(),
        to: ctx
            .accounts
            .providers_token_account
            .to_account_info()
            .clone(),
        authority: ctx.accounts.token_authority.to_account_info().clone(),
    };
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        transfer,
    );
    token::transfer(transfer_ctx, contract.initial_amount_paid)?;
    Ok(())
}
