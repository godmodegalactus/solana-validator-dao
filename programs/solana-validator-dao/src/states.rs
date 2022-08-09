use anchor_lang::prelude::*;

const PROVIDES_BUYING_AND_SETUP_OF_VALIDATOR: u64 = 1;
const PROVIDES_ADMIN_SERVICES_FOR_VALIDATOR: u64 = 1 << 2;
const PROVIDES_RENTING_OF_VALIDATORS: u64 = 1 << 3;
const PROVIDES_CONFIGURING_A_SERVICE_FOR_VALIDATOR: u64 = 1 << 4;
const PROVIDES_SOLVING_ISSUES_FOR_VALIDATOR: u64 = 1 << 5;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Datatype {
    ValidatorProvider,
    GovernaceProvider,
    Contract,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PaymentPeriodicity {
    Yearly,
    Monthly,
    Weekly,
    Daily,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub datatype: Datatype,
    pub is_initialized: bool,
    pub reserved: [u8; 8],
}

// This struct represents validator provider and its details
#[account()]
pub struct ValidatorProvider {
    pub meta_data: Metadata,
    pub owner: Pubkey,
    pub payment_mint: Pubkey,
    pub services: u64,
    pub rating: f32,
    pub review_count: u32,
    pub serving_governance_count: u32, // how many governances provider is serving
    pub name: [u8; 128],
    pub description: [u8; 1024],
    pub reserved: [u8; 256],
}

#[account()]
pub struct GovernanceProvider {
    pub meta_data: Metadata,
    pub governance_id: Pubkey,
    pub validator_provider: Pubkey,
    pub validator_provider_owner: Pubkey,
    pub added_timestamp: u64,
    pub contract_count: u32,
}

#[account()]
pub struct GovernanceContract {
    pub meta_data: Metadata,
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
    pub periodicity: PaymentPeriodicity,
    pub payment_mint: Pubkey,
    pub dao_payment_account: Pubkey,
    pub has_signed_by_provider: bool,
    pub executed: bool,
    pub reserved: [u8; 256],
}

impl ValidatorProvider {
    pub fn is_valid(&self) -> bool {
        self.meta_data.datatype == Datatype::ValidatorProvider
            && self.meta_data.is_initialized == true
    }
}

impl GovernanceProvider {
    pub fn is_valid(&self) -> bool {
        self.meta_data.datatype == Datatype::GovernaceProvider
            && self.meta_data.is_initialized == true
    }
}

impl GovernanceContract {
    pub fn is_valid(&self) -> bool {
        self.meta_data.datatype == Datatype::Contract && self.meta_data.is_initialized == true
    }
}
