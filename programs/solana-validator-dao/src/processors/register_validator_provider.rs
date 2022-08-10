use crate::{
    errors::ValidatorDaoErrors,
    states::{Datatype, Metadata},
    *,
};
use instructions::RegisterValidatorServiceProvider;

pub fn process(
    ctx: Context<RegisterValidatorServiceProvider>,
    services: u64,
    name: String,
    description: String,
) -> Result<()> {
    msg!("registering a validator provider");
    if name.len() > 128 {
        return Err(ValidatorDaoErrors::NameTooLarge.into());
    }

    if description.len() > 1024 {
        return Err(ValidatorDaoErrors::DescriptionTooLarge.into());
    }
    let validator_provider_data = &mut ctx.accounts.provider_data;
    validator_provider_data.meta_data = Metadata {
        datatype: Datatype::ValidatorProvider,
        is_initialized: true,
        reserved: [0; 8],
    };
    validator_provider_data.owner = ctx.accounts.owner.key();
    validator_provider_data.services = services;
    validator_provider_data.rating = 0;
    validator_provider_data.review_count = 0;
    validator_provider_data.serving_governance_count = 0;
    validator_provider_data.payment_mint = ctx.accounts.payment_mint.key();
    validator_provider_data.name[..name.len()].clone_from_slice(name.as_bytes());
    validator_provider_data.description[..description.len()]
        .clone_from_slice(description.as_bytes());
    Ok(())
}
