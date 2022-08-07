use anchor_lang::prelude::*;

#[error_code]
pub enum ValidatorDaoErrors {
    #[msg("Name too large")]
    NameTooLarge,
    #[msg("Description too large")]
    DescriptionTooLarge,
    #[msg("default")]
    Default,
}
