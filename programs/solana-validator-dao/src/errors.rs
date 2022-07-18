use anchor_lang::prelude::*;

#[error]
pub enum ValidatorDaoErrors
{
    #[msg("default")]
    Default,
}