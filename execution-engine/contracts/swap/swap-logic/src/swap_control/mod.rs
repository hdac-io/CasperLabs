mod signiture_verification;
mod storage;

use alloc::{string::String, vec::Vec};

use contract::contract_api::runtime;
use num_traits::cast::AsPrimitive;
use storage::{UnitKYCData, UnitSnapshotData};
use types::{
    account::PublicKey, system_contract_errors::mint::Error as MintError, ApiError, Key, U512,
};

use crate::constants;
use signiture_verification::signature_verification;

// Admin features

pub fn set_swap_hash(swap_hash: Key) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }
    runtime::put_key(constants::users::KEY_SWAP_HASH, swap_hash);
}

pub fn insert_kyc_allowance_cap(allowance_cap: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    storage::save_kyc_border_allowance_cap(allowance_cap);
}

pub fn insert_snapshot(ver1_address: String, prev_balance: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let new_data = UnitSnapshotData { prev_balance };
    storage::save_snapshot_data(ver1_address, new_data);
}

pub fn insert_kyc_data(new_mainnet_address: PublicKey) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let new_data = UnitKYCData {
        is_sent_token_for_swap: U512::from(0),
        kyc_step: U512::from(0),
        kyc_level: U512::from(0),
        swapped_amount: U512::from(0),
    };
    storage::save_kyc_data(new_mainnet_address, new_data);
}

pub fn update_kyc_level(new_mainnet_address: PublicKey, kyc_level: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_kyc_data(new_mainnet_address);
    curr_data.kyc_level = kyc_level;
    storage::save_kyc_data(new_mainnet_address, curr_data);
}

pub fn update_status_is_sent_token_for_swap(
    new_mainnet_address: PublicKey,
    is_sent_token_for_swap: U512,
) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_kyc_data(new_mainnet_address);
    curr_data.is_sent_token_for_swap = is_sent_token_for_swap;
    storage::save_kyc_data(new_mainnet_address, curr_data);
}

pub fn update_kyc_step(new_mainnet_address: PublicKey, kyc_step: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_kyc_data(new_mainnet_address);
    curr_data.kyc_step = kyc_step;
    storage::save_kyc_data(new_mainnet_address, curr_data);
}

// user features

pub fn validate_sign_and_update_swapped_amount(
    ver1_address: Vec<String>,
    ver1_pubkey_hex: Vec<String>,
    message: Vec<String>,
    signature_hex: Vec<String>,
) {
    if !(ver1_address.len() == ver1_pubkey_hex.len()
        && ver1_address.len() == message.len()
        && ver1_address.len() == signature_hex.len())
    {
        runtime::revert(MintError::InsufficientNumOfSwapParams);
    }

    // Get stored values
    let curr_account = runtime::get_caller();
    let kyc_border_allowance_cap = storage::load_kyc_border_allowance_cap();

    let mut curr_user_kyc_data = storage::load_kyc_data(curr_account);

    // Check KYC status
    if curr_user_kyc_data.kyc_step < U512::from(1) {
        runtime::revert(MintError::NotRegisteredKYC);
    }

    // Iterate addresses and summize for total value
    let mut prev_amount_for_whole_address = U512::from(0);
    for address in &ver1_address {
        let data = storage::load_snapshot_data(address.clone());
        prev_amount_for_whole_address += data.prev_balance;
    }

    let kyc_level_in_primitive_type: u64 = curr_user_kyc_data.kyc_level.as_();
    let mut swappable_amount = match kyc_level_in_primitive_type {
        1u64 => {
            if prev_amount_for_whole_address >= kyc_border_allowance_cap {
                kyc_border_allowance_cap
            } else {
                prev_amount_for_whole_address
            }
        }
        2u64 => prev_amount_for_whole_address,
        _ => runtime::revert(MintError::InvalidKYCLevelValue),
    };

    // Check whether the user executed swap or not
    if curr_user_kyc_data.swapped_amount >= swappable_amount {
        runtime::revert(MintError::ExceededSwapRange);
    } else {
        swappable_amount -= curr_user_kyc_data.swapped_amount;
    }

    // Sign verification
    for i in 0..ver1_address.len() {
        if !signature_verification(
            ver1_pubkey_hex[i].clone(),
            message[i].clone(),
            signature_hex[i].clone(),
        ) {
            runtime::revert(ApiError::NoAccessRights);
        }
    }

    // Update data
    curr_user_kyc_data.swapped_amount += swappable_amount;
    storage::save_kyc_data(curr_account, curr_user_kyc_data);
}

#[cfg(test)]
mod tests {
    use super::signature_verification;

    #[test]
    pub fn test_should_verify_signature() {
        let pubkey =
            String::from("02c4ef70543e18889167ca67c8aa28c1d4c259e89cb34483a8ed6cfd3a03e8246b");
        let signature = String::from(
            "24899366fd3d5dfe6740df1e5f467a53f1a3aaafce26d8df1497a925c55b5c266339a95fe6\
                                507bd611b0e3b6e74e3bb7f19eeb1165615e5cebe7f40e5765bc41",
        );
        let message =
            String::from("69046d44e3d75d48436377626372a44a5066966b5d72c00b67769c1cc6a8619a");

        signature_verification(pubkey, message, signature);
    }
}
