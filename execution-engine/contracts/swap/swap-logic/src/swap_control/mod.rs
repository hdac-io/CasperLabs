mod signiture_verification;
mod storage;

use alloc::string::String;

use signiture_verification::signature_verification;
use storage::UnitSwapData;

use contract::contract_api::{runtime, system, account};

use types::{
    account::PublicKey,
    system_contract_errors::mint::{Error as MintError},
    U512, TransferResult, ApiError,
};

// Admin features

pub fn insert_info(
    ver1_pubkey: String,
    new_mainnet_addr: PublicKey,
    prev_balance: U512,
    //kyc_level: U512,
    //is_sent_token_for_swap: U512,
    //kyc_step: U512,
    //swapped_amount: U512,
) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let new_data = UnitSwapData {
        new_mainnet_addr: new_mainnet_addr,
        prev_balance: prev_balance,
        kyc_level: U512::from(0),
        is_sent_token_for_swap: U512::from(0),
        kyc_step: U512::from(0),
        swapped_amount: U512::from(0),
    };
    storage::save_data(ver1_pubkey, new_data);
}

pub fn update_kyc_level(
    ver1_pubkey: String,
    kyc_level: U512,
) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_data(ver1_pubkey.clone());
    curr_data.kyc_level = kyc_level;
    storage::save_data(ver1_pubkey, curr_data);
}

pub fn update_status_is_sent_token_for_swap(
    ver1_pubkey: String,
    is_sent_token_for_swap: U512,
) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_data(ver1_pubkey.clone());
    curr_data.is_sent_token_for_swap = is_sent_token_for_swap;
    storage::save_data(ver1_pubkey, curr_data);
}

pub fn update_kyc_step(
    ver1_pubkey: String,
    kyc_step: U512,
) {
    if runtime::get_caller() != storage::load_admin() {
        runtime::revert(ApiError::NoAccessRights);
    }

    let mut curr_data = storage::load_data(ver1_pubkey.clone());
    curr_data.kyc_step = kyc_step;
    storage::save_data(ver1_pubkey, curr_data);
}

// user features

pub fn send_token_and_update_swapped_amount(
    ver1_address: String,
    ver1_pubkey_hex: String,
    message: String,
    signature_hex: String,
    swap_request_amount: U512,
) {
    let mut curr_data = storage::load_data(ver1_address.clone());

    // Message & Signature check of ver1 mainnet
    if !signature_verification(ver1_pubkey_hex.clone(), message, signature_hex) {
        runtime::revert(ApiError::NoAccessRights);
    }

    // Check & calculate the maximum swapable amount
    // Case 1: normal
    let mut updated_swap_request_amount = swap_request_amount.clone();

    // Case 2: Swap range remains but not match to swap request
    if curr_data.swapped_amount + swap_request_amount > curr_data.prev_balance 
        && curr_data.swapped_amount < curr_data.prev_balance {
        updated_swap_request_amount = curr_data.prev_balance - curr_data.swapped_amount;
    }
    // Case 3: Swap range exceeded
    else if curr_data.swapped_amount + swap_request_amount > curr_data.prev_balance 
        && curr_data.swapped_amount >= curr_data.prev_balance {
        runtime::revert(MintError::ExceededSwapRange);
    }

    // Update data
    curr_data.swapped_amount += updated_swap_request_amount;
    storage::save_data(ver1_address, curr_data);

    // let transfer_res: TransferResult =
    //     system::transfer_from_purse_to_account(
    //         account::get_main_purse(),
    //         curr_data.new_mainnet_addr,
    //         updated_swap_request_amount,
    //     );

    // if let Err(err) = transfer_res {
    //     runtime::revert(err);
    // }
}

#[cfg(test)]
mod tests {
    use super::signature_verification;

    #[test]
    pub fn test_should_verify_signature(){
        let pubkey = String::from("0223bec70d670d29a30d9bcee197910e37cf2a10f0dc3c5ac44d865aec0d7052fb");
        let signature = String::from("30440220434caf5bb442cb6a251e8bce0ec493f9a1a9c4423bcfc029e542b0e8a89d1b3\
                         f022011090d4e98f79c62b188245a4aa4eb77e912bfd57e0a9b9a1c5e65f2b39f3ab401");
        let message = String::from("020000000001011333183ddf384da83ed49296136c70d206ad2b19331bf25d390e69b2221\
                       65e370000000017160014b93f973eb2bf0b614bddc0f47286788c98c535b4feffffff0200\
                       e1f5050000000017a914a860f76561c85551594c18eecceffaee8c4822d787f0c1a435000\
                       0000017a914d8b6fcc85a383261df05423ddf068a8987bf028787024730440220434caf5b\
                       b442cb6a251e8bce0ec493f9a1a9c4423bcfc029e542b0e8a89d1b3f022011090d4e98f79\
                       c62b188245a4aa4eb77e912bfd57e0a9b9a1c5e65f2b39f3ab401210223bec70d670d29a3\
                       0d9bcee197910e37cf2a10f0dc3c5ac44d865aec0d7052fb8c000000");

        signature_verification(pubkey, message, signature);
    }
}
