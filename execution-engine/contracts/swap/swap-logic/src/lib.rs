#![cfg_attr(not(test), no_std)]

mod constants;
mod swap_control;

extern crate alloc;

use alloc::{string::String, vec::Vec};

use crate::constants::methods;
use contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use types::{account::PublicKey, ApiError, Key, U512};

#[no_mangle]
pub extern "C" fn delegate() {
    let method_name: String = runtime::get_arg(0)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);

    match method_name.as_str() {
        methods::METHOD_SET_SWAP_HASH => {
            let swap_hash_address: Key = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::set_swap_hash(swap_hash_address);
        }
        methods::METHOD_INSERT_KYC_ALLOWANCE_CAP => {
            let kyc_allowance: U512 = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::insert_kyc_allowance_cap(kyc_allowance);
        }
        methods::METHOD_INSERT_SNAPSHOT_RECORD => {
            let ver1_address: String = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let prev_balance: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::insert_snapshot(ver1_address, prev_balance);
        }
        methods::METHOD_INSERT_KYC_DATA => {
            let new_mainnet_address: PublicKey = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::insert_kyc_data(new_mainnet_address);
        }
        methods::METHOD_UPDATE_KYC_LEVEL => {
            let new_mainnet_address: PublicKey = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let kyc_level: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::update_kyc_level(new_mainnet_address, kyc_level);
        }
        methods::METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT => {
            let new_mainnet_address: PublicKey = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let is_sent_token_for_swap: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::update_status_is_sent_token_for_swap(
                new_mainnet_address,
                is_sent_token_for_swap,
            );
        }
        methods::METHOD_UPDATE_KYC_STEP => {
            let new_mainnet_address: PublicKey = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let kyc_step: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::update_kyc_step(new_mainnet_address, kyc_step);
        }

        methods::METHOD_GET_TOKEN => {
            let ver1_address_arr: Vec<String> = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let ver1_pubkey_hex_arr: Vec<String> = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let message_arr: Vec<String> = runtime::get_arg(3)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let signature_hex_arr: Vec<String> = runtime::get_arg(4)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::validate_sign_and_update_swapped_amount(
                ver1_address_arr,
                ver1_pubkey_hex_arr,
                message_arr,
                signature_hex_arr,
            );
        }

        _ => {}
    }
}

#[cfg(not(feature = "lib"))]
#[no_mangle]
pub extern "C" fn call() {
    delegate();
}
