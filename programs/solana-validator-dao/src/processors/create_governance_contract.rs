use crate::{
    states::{Metadata, PaymentPeriodicity},
    *,
};

use instructions::CreateGovernanceContract;
use spl_governance::state::governance;

pub fn process(
    ctx: Context<CreateGovernanceContract>,
    services: u64,
    contract_start_unix_timestamp: u64,
    contract_end_unix_timestamp: u64,
    initial_amount: u64,
    recurring_amount: u64,
    periodicity: PaymentPeriodicity,
    number_of_periods: u32,
) -> Result<()> {
    if periodicity == PaymentPeriodicity::Unknown {
        return Err(errors::ValidatorDaoErrors::UnknownPeriodicity.into());
    }

    governance::assert_is_valid_governance(&GOVERNANCE_PROGRAM_ID, &ctx.accounts.governance_ai)?;

    let contract = &mut ctx.accounts.governance_contract;

    contract.meta_data = Metadata {
        datatype: states::Datatype::Contract,
        is_initialized: true,
        reserved: [0; 8],
    };
    contract.governance_id = ctx.accounts.governance_ai.key();
    contract.contract_creator = ctx.accounts.payer.key();
    contract.validator_provider = ctx.accounts.provider_data.key();
    contract.validator_provider_owner = ctx.accounts.provider_data.owner;
    contract.services_to_be_provided = services;
    contract.contract_start_timestamp = contract_start_unix_timestamp;
    contract.contract_end_timestamp = contract_end_unix_timestamp;
    contract.initial_amount_paid = initial_amount;
    contract.recurring_amount_to_be_paid = recurring_amount;
    contract.recurring_amount_already_paid = 0;
    contract.periodicity = periodicity;
    contract.number_of_periods = number_of_periods;
    contract.contract_end_timestamp = contract_start_unix_timestamp
        .checked_add(periodicity.to_secs(number_of_periods))
        .unwrap();
    contract.payment_mint = ctx.accounts.payment_mint.key();
    contract.dao_payment_account = ctx.accounts.token_account.key();
    contract.provider_token_account = ctx.accounts.providers_token_account.key();
    contract.executed = false;

    ctx.accounts.governance_provider_data.contract_count += 1;

    // if the contract creater is provider then sign the contract automatically
    contract.has_signed_by_provider = if ctx
        .accounts
        .payer
        .key()
        .eq(&contract.validator_provider_owner)
    {
        true
    } else {
        false
    };

    Ok(())
}
