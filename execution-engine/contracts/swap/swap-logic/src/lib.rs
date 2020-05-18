#![cfg_attr(not(test), no_std)]

mod swap_control;
mod constants;

extern crate alloc;

use alloc::{
    string::String,
};

use contract::{
    contract_api::runtime,
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    account::PublicKey, U512, ApiError,
};
use crate::constants::methods;

#[no_mangle]
pub extern "C" fn delegate() {
    let method_name: String = runtime::get_arg(0)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);

    match method_name.as_str() {
        methods::METHOD_INSERT_SNAPSHOT_RECORD => {
            let ver1_pubkey: String = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let new_mainnet_addr: PublicKey = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let prev_balance: U512 = runtime::get_arg(3)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::insert_info(ver1_pubkey, new_mainnet_addr, prev_balance);
        }

        methods::METHOD_UPDATE_KYC_LEVEL => {
            let ver1_pubkey: String = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let kyc_level: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::update_kyc_level(ver1_pubkey, kyc_level);
        }

        methods::METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT => {
            let ver1_pubkey: String = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let is_sent_token_for_swap: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::update_status_is_sent_token_for_swap(ver1_pubkey, is_sent_token_for_swap);
        }

        methods::METHOD_UPDATE_KYC_STEP => {
            let ver1_pubkey: String = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let kyc_step: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::update_kyc_step(ver1_pubkey, kyc_step);
        }

        methods::METHOD_GET_TOKEN => {
            let ver1_pubkey_hex: String = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let message: String = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let signature_hex: String = runtime::get_arg(3)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);
            let swap_request_amount: U512 = runtime::get_arg(4)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::InvalidArgument);

            swap_control::send_token_and_update_swapped_amount(
                ver1_pubkey_hex,
                message,
                signature_hex,
                swap_request_amount
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