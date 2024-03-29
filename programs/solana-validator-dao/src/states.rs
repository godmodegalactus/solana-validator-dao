use std::u8;

use anchor_lang::prelude::*;

const PROVIDES_BUYING_AND_SETUP_OF_VALIDATOR: u64 = 1;
const PROVIDES_ADMIN_SERVICES_FOR_VALIDATOR: u64 = 1 << 2;
const PROVIDES_RENTING_OF_VALIDATORS: u64 = 1 << 3;
const PROVIDES_CONFIGURING_A_SERVICE_FOR_VALIDATOR: u64 = 1 << 4;
const PROVIDES_SOLVING_ISSUES_FOR_VALIDATOR: u64 = 1 << 5;


#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PaymentPeriodicity {
    Yearly = 0,
    Monthly,
    Weekly,
    Daily,
    Unknown,
}

impl From<u8> for PaymentPeriodicity {
    fn from(v: u8) -> Self {
        match v {
            0 => return PaymentPeriodicity::Yearly,
            1 => return PaymentPeriodicity::Monthly,
            2 => return PaymentPeriodicity::Weekly,
            3 => return PaymentPeriodicity::Daily,
            4..=u8::MAX => return PaymentPeriodicity::Unknown,
        }
    }
}

impl PaymentPeriodicity {
    fn number_of_seconds_in_period(period: PaymentPeriodicity) -> u64 {
        let number_of_seconds_in_period: u64 = match period {
            PaymentPeriodicity::Daily => 60 * 60 * 24,
            PaymentPeriodicity::Weekly => {
                Self::number_of_seconds_in_period(PaymentPeriodicity::Daily)
                    .saturating_mul(7 as u64)
            }

            PaymentPeriodicity::Monthly => {
                Self::number_of_seconds_in_period(PaymentPeriodicity::Daily).saturating_mul(30)
            }
            PaymentPeriodicity::Yearly => {
                Self::number_of_seconds_in_period(PaymentPeriodicity::Daily).saturating_mul(365)
            }
            PaymentPeriodicity::Unknown => 0,
        };
        return number_of_seconds_in_period;
    }

    pub fn to_secs(&self, number_of_periods: u32) -> u64 {
        Self::number_of_seconds_in_period(*self).saturating_mul(number_of_periods as u64)
    }
}

// This struct represents validator provider and its details
#[account(zero_copy)]
pub struct ValidatorProvider {
    pub owner: Pubkey,
    pub payment_mint: Pubkey,
    pub services: u64,
    pub rating: u32,
    pub review_count: u32,
    pub serving_governance_count: u32, // how many governances provider is serving
    pub name: [u8; 128],
    pub description: [u8; 2048],
    pub reserved: [u8; 256],
}

#[account()]
pub struct GovernanceProvider {
    pub governance_id: Pubkey,
    pub validator_provider: Pubkey,
    pub validator_provider_owner: Pubkey,
    pub added_timestamp: u64,
    pub contract_count: u32,
}

#[account()]
pub struct GovernanceContract {
    pub governance_id: Pubkey,
    pub contract_creator: Pubkey,
    pub validator_provider: Pubkey,
    pub validator_provider_owner: Pubkey,
    pub provider_token_account: Pubkey,
    pub services_to_be_provided: u64,
    pub contract_start_timestamp: u64,
    pub contract_end_timestamp: u64,
    pub initial_amount_paid: u64,
    pub recurring_amount_to_be_paid: u64,
    pub recurring_amount_already_paid: u64,
    pub periodicity: PaymentPeriodicity,
    pub number_of_periods: u32,
    pub payment_mint: Pubkey,
    pub dao_payment_account: Pubkey,
    pub has_signed_by_provider: bool,
    pub executed: bool,
    pub reserved: [u8; 256],
}
