use crate::{*};

use instructions::AddRegisteredProviderToGovernance;
use spl_governance::state::governance;
use spl_governance::state::native_treasury;

pub fn process(ctx: Context<AddRegisteredProviderToGovernance>) -> Result<()> {
    governance::assert_is_valid_governance(&GOVERNANCE_PROGRAM_ID, &ctx.accounts.governance_ai)?;
    let native_treasury_address = native_treasury::get_native_treasury_address(
        &GOVERNANCE_PROGRAM_ID,
        ctx.accounts.governance_ai.key,
    );

    assert_eq!(
        native_treasury_address,
        ctx.accounts.governance_native_treasury.key()
    );

    let provider_data = &mut ctx.accounts.provider_data.load_mut()?;
    let governance_provider_data = &mut ctx.accounts.governance_provider_data;
    governance_provider_data.governance_id = ctx.accounts.governance_ai.key();
    governance_provider_data.validator_provider = ctx.accounts.provider_data.key();
    governance_provider_data.validator_provider_owner = provider_data.owner.key();
    governance_provider_data.added_timestamp = Clock::get()?.unix_timestamp as u64;
    governance_provider_data.contract_count = 0;
    provider_data.serving_governance_count += 1;

    Ok(())
}
