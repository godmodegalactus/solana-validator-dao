use std::str::FromStr;

use crate::{states::Metadata, *};

use instructions::AddRegisteredProviderToGovernance;
use spl_governance::state::governance;
use spl_governance::state::native_treasury;

pub fn process(ctx: Context<AddRegisteredProviderToGovernance>) -> Result<()> {
    let governace_pid_res = solana_program::pubkey::Pubkey::from_str(GOVERNANCE_PROGRAM_ID);
    let governance_pid = match governace_pid_res {
        Ok(x) => x,
        Err(_) => Pubkey::default(),
    };
    if governance_pid.eq(&Pubkey::default()) {
        return Err(errors::ValidatorDaoErrors::GovernancePidProblem.into());
    }

    governance::assert_is_valid_governance(&governance_pid, &ctx.accounts.governance_ai)?;
    let native_treasury_address = native_treasury::get_native_treasury_address(
        &governance_pid,
        ctx.accounts.governance_ai.key,
    );

    assert_eq!(
        native_treasury_address,
        ctx.accounts.governance_native_treasury.key()
    );

    let governance_provider_data = &mut ctx.accounts.governance_provider_data;
    governance_provider_data.meta_data = Metadata {
        datatype: states::Datatype::GovernaceProvider,
        is_initialized: true,
        reserved: [0; 8],
    };
    governance_provider_data.governance_id = ctx.accounts.governance_ai.key();
    governance_provider_data.validator_provider = ctx.accounts.provider_data.key();
    governance_provider_data.validator_provider_owner = ctx.accounts.provider_data.owner.key();
    governance_provider_data.added_timestamp = Clock::get()?.unix_timestamp as u64;
    governance_provider_data.contract_count = 0;

    Ok(())
}
