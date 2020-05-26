mod signiture_verification;
mod storage;

use alloc::{string::String, vec::Vec};

use contract::contract_api::runtime;
use storage::UnitSwapData;
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

pub fn insert_info(
    ver1_pubkey: String,
    new_mainnet_addr: PublicKey,
    prev_balance: U512,
    /* kyc_level: U512,
     * is_sent_token_for_swap: U512,
     * kyc_step: U512,
     * swapped_amount: U512, */
) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let new_data = UnitSwapData {
        new_mainnet_addr,
        prev_balance,
        kyc_level: U512::from(0),
        is_sent_token_for_swap: U512::from(0),
        kyc_step: U512::from(0),
        swapped_amount: U512::from(0),
    };
    storage::save_data(ver1_pubkey, new_data);
}

pub fn update_kyc_level(ver1_pubkey: String, kyc_level: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_data(ver1_pubkey.clone());
    curr_data.kyc_level = kyc_level;
    storage::save_data(ver1_pubkey, curr_data);
}

pub fn update_status_is_sent_token_for_swap(ver1_pubkey: String, is_sent_token_for_swap: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_data(ver1_pubkey.clone());
    curr_data.is_sent_token_for_swap = is_sent_token_for_swap;
    storage::save_data(ver1_pubkey, curr_data);
}

pub fn update_kyc_step(ver1_pubkey: String, kyc_step: U512) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_data(ver1_pubkey.clone());
    curr_data.kyc_step = kyc_step;
    storage::save_data(ver1_pubkey, curr_data);
}

// user features

pub fn validate_sign_and_update_swapped_amount(
    ver1_address: Vec<String>,
    ver1_pubkey_hex: Vec<String>,
    message: Vec<String>,
    signature_hex: Vec<String>,
    swap_request_amount: Vec<U512>,
) {
    if !(ver1_address.len() == ver1_pubkey_hex.len()
        && ver1_address.len() == message.len()
        && ver1_address.len() == signature_hex.len()
        && ver1_address.len() == swap_request_amount.len())
    {
        runtime::revert(MintError::InsufficientNumOfSwapParams);
    }

    for i in 0..ver1_address.len() {
        // Message & Signature check of ver1 mainnet
        if !signature_verification(
            ver1_pubkey_hex[i].clone(),
            message[i].clone(),
            signature_hex[i].clone(),
        ) {
            runtime::revert(ApiError::NoAccessRights);
        }

        let mut curr_data = storage::load_data(ver1_address[i].clone());

        // Check & calculate the maximum swapable amount
        // Case 1: normal
        let mut updated_swap_request_amount = swap_request_amount[i];

        // Case 2: Swap range remains but not match to swap request
        if curr_data.swapped_amount + swap_request_amount[i] > curr_data.prev_balance
            && curr_data.swapped_amount < curr_data.prev_balance
        {
            updated_swap_request_amount = curr_data.prev_balance - curr_data.swapped_amount;
        }
        // Case 3: Swap range exceeded
        else if curr_data.swapped_amount + swap_request_amount[i] > curr_data.prev_balance
            && curr_data.swapped_amount >= curr_data.prev_balance
        {
            runtime::revert(MintError::ExceededSwapRange);
        }

        // Update data
        curr_data.swapped_amount += updated_swap_request_amount;
        storage::save_data(ver1_address[i].clone(), curr_data);
    }
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
