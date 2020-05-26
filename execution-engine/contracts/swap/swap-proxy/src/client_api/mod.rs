mod error;

use alloc::{
    string::String,
    vec::Vec
};
use core::convert::TryInto;

use contract::{
    contract_api::runtime,
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{account::PublicKey, ApiError, ContractRef, URef, Key, U512};

use error::Error;

pub mod method_names {
    pub mod proxy {
        use super::swap;

        pub const NAME_SWAP_HASH: &str = "swap_hash";
        pub const METHOD_SET_SWAP_HASH: &str = swap::METHOD_SET_SWAP_HASH;
        pub const METHOD_INSERT_SNAPSHOT_RECORD: &str = swap::METHOD_INSERT_SNAPSHOT_RECORD;
        pub const METHOD_UPDATE_KYC_LEVEL: &str = swap::METHOD_UPDATE_KYC_LEVEL;
        pub const METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT: &str = swap::METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT;
        pub const METHOD_UPDATE_KYC_STEP: &str = swap::METHOD_UPDATE_KYC_STEP;
        pub const METHOD_GET_TOKEN: &str = swap::METHOD_GET_TOKEN;
    }
    pub mod swap {
        pub const METHOD_SET_SWAP_HASH: &str = "set_swap_hash";
        pub const METHOD_INSERT_SNAPSHOT_RECORD: &str = "insert_snapshot_record";
        pub const METHOD_UPDATE_KYC_LEVEL: &str = "update_kyc_level";
        pub const METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT: &str = "update_status_swapable_token_sent";
        pub const METHOD_UPDATE_KYC_STEP: &str = "update_kyc_step";
        pub const METHOD_GET_TOKEN: &str = "get_token";
    }
}

pub enum Api {
    SetSwapHash(Key),
    InsertSnapshotRecord(String, PublicKey, U512),
    UpdateKYCLevel(String, U512),
    UpdateStatusSwapableTokenSent(String, U512),
    UpdateKYCStep(String, U512),
    GetToken(Key, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<U512>),
}

fn get_contract_ref() -> ContractRef {
    let contract_hash = runtime::get_key(method_names::proxy::NAME_SWAP_HASH)
        .unwrap_or_revert_with(ApiError::GetKey);
    contract_hash
        .to_contract_ref()
        .unwrap_or_revert()
}

impl Api {
    pub fn from_args() -> Self {
        let method_name: String = runtime::get_arg(0)
            .unwrap_or_revert_with(ApiError::MissingArgument)
            .unwrap_or_revert_with(ApiError::InvalidArgument);

        match method_name.as_str() {
            method_names::proxy::METHOD_SET_SWAP_HASH => {
                let swap_hash: Key = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);

                Api::SetSwapHash(swap_hash)
            }
            method_names::proxy::METHOD_INSERT_SNAPSHOT_RECORD => {
                let ver1_address: String = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let new_mainnet_address: PublicKey = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let amount: U512 = runtime::get_arg(3)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::InsertSnapshotRecord(ver1_address, new_mainnet_address, amount)
            }
            method_names::proxy::METHOD_UPDATE_KYC_LEVEL => {
                let ver1_address: String = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let kyc_level: U512 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::UpdateKYCLevel(ver1_address, kyc_level)
            }
            method_names::proxy::METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT => {
                let ver1_address: String = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let is_swapable_token_sent: U512 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::UpdateStatusSwapableTokenSent(ver1_address, is_swapable_token_sent)
            }
            method_names::proxy::METHOD_UPDATE_KYC_STEP => {
                let ver1_address: String = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let kyc_step: U512 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::UpdateKYCStep(ver1_address, kyc_step)
            }
            method_names::proxy::METHOD_GET_TOKEN => {
                let contract_hash: Key = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let ver1_address: Vec<String> = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let ver1_pubkey: Vec<String> = runtime::get_arg(3)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let message: Vec<String> = runtime::get_arg(4)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let signature: Vec<String> = runtime::get_arg(5)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let swap_amount: Vec<U512> = runtime::get_arg(6)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::GetToken(contract_hash, ver1_address, ver1_pubkey, message, signature, swap_amount)
            }
            _ => runtime::revert(Error::UnknownProxyApi),
        }
    }

    pub fn invoke(&self) {
        match self {
            Self::SetSwapHash(swap_hash) => {
                let contract_ref = swap_hash
                    .to_contract_ref()
                    .unwrap_or_revert();
                runtime::call_contract(
                    contract_ref,
                    (
                        method_names::proxy::METHOD_SET_SWAP_HASH,
                        *swap_hash
                    )
                )
            }
            Self::InsertSnapshotRecord(ver1_address, new_mainnet_address, amount) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_INSERT_SNAPSHOT_RECORD,
                        ver1_address.clone(),
                        *new_mainnet_address,
                        *amount
                    )
                )
            }
            Self::UpdateKYCLevel(ver1_address, kyc_level) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_UPDATE_KYC_LEVEL,
                        ver1_address.clone(),
                        kyc_level.clone()
                    )
                )
            }
            Self::UpdateStatusSwapableTokenSent(ver1_address, is_swapable_token_sent) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT,
                        ver1_address.clone(),
                        is_swapable_token_sent.clone()
                    )
                )
            }
            Self::UpdateKYCStep(ver1_address, kyc_step) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_UPDATE_KYC_STEP,
                        ver1_address.clone(),
                        kyc_step.clone()
                    )
                )
            }
            Self::GetToken(swap_contract_hash, ver1_address_arr, ver1_pubkey_arr, message_arr, signature_arr, amount_arr) => {
                let contract_ref = swap_contract_hash
                    .to_contract_ref()
                    .unwrap_or_revert();

                runtime::call_contract(
                    contract_ref,
                    (
                        method_names::proxy::METHOD_GET_TOKEN,
                        ver1_address_arr.clone(),
                        ver1_pubkey_arr.clone(),
                        message_arr.clone(),
                        signature_arr.clone(),
                        amount_arr.clone()
                    )
                )
            }
        }
    }
}
