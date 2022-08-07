use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

mod processors;

mod errors;
mod states;

declare_id!("AwyKDr1Z5BfdvK3jX1UWopyjsJSV5cq4cuJpoYLofyEn");

const GOVERNANCE_PROGRAM_ID: &str = "GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw";

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
}
