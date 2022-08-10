use anchor_lang::prelude::*;

#[error_code]
pub enum ValidatorDaoErrors {
    #[msg("Name too large")]
    NameTooLarge,
    #[msg("Description too large")]
    DescriptionTooLarge,
    #[msg("Contract has not started yet")]
    ContractNotYetStarted,
    #[msg("Contract not yet signed by the provider")]
    ContractNotSignedByProvider,
    #[msg("Unknown periodicity")]
    UnknownPeriodicity,
    #[msg("default")]
    Default,
}
