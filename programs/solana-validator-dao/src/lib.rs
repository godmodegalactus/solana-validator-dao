use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

mod states;
mod processors;
mod errors;

declare_id!("AwyKDr1Z5BfdvK3jX1UWopyjsJSV5cq4cuJpoYLofyEn");
const GOVERNANCE_PROGRAM_ID: Pubkey = solana_program::pubkey!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

solana_security_txt::security_txt! {
    name: "Validator DAO",
    project_url: "https://github.com/godmodegalactus/solana-validator-dao",
    contacts: "godmodegalactus@gmail.com",
    policy: "",
    preferred_languages: "en",
    auditors: "Godmode Galactus"
}

#[program]
pub mod solana_validator_dao {

    use super::*;

    pub fn stake(ctx: Context<InitalizeDAOStakeAccount>, seed: u8, lamports: u64) -> Result<()> {
        processors::process_stake::process(ctx, seed, lamports)
    }

    pub fn register_validator_provider(
        ctx: Context<RegisterValidatorServiceProvider>,
        services: u64,
        name: String,
        description: String,
    ) -> Result<()> {
        processors::register_validator_provider::process(ctx, services, name, description)
    }

    pub fn add_registered_provider_to_governance(
        ctx: Context<AddRegisteredProviderToGovernance>,
    ) -> Result<()> {
        processors::add_registered_provider_to_governance::process(ctx)
    }

    pub fn create_governance_contract(
        ctx: Context<CreateGovernanceContract>,
        _contract_seed: u64,
        services: u64,
        contract_start_unix_timestamp: u64,
        contract_end_unix_timestamp: u64,
        initial_amount: u64,
        recurring_amount: u64,
        periodicity: u8,
        number_of_periods: u32,
    ) -> Result<()> {
        processors::create_governance_contract::process(
            ctx,
            services,
            contract_start_unix_timestamp,
            contract_end_unix_timestamp,
            initial_amount,
            recurring_amount,
            periodicity.into(),
            number_of_periods,
        )
    }

    pub fn execute_governance_contract(ctx: Context<ExecuteContract>) -> Result<()> {
        processors::execute_contract::process(ctx)
    }
}
