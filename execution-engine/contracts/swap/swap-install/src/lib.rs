#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::String,
};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{URef, Key, ApiError, CLValue};

const KEY_ADMIN: &str = "admin";
const NAME_SWAP_UREF: &str = "swap_uref";
const NAME_SWAP_LOGIC_EXT: &str = "swap_logic_ext";

#[no_mangle]
pub extern "C" fn swap_logic_ext() {
    swap_logic::delegate();
}

#[no_mangle]
pub extern "C" fn call() {
    // Get caller's public key and store as admin
    let admin_uref: URef = storage::new_uref(runtime::get_caller());

    // create map of references for stored contract
    let mut swapper_urefs: BTreeMap<String, Key> = BTreeMap::new();
    swapper_urefs.insert(String::from(KEY_ADMIN), admin_uref.into());

    // Swap function storage
    let swap_function_pointer: URef =
        storage::store_function(NAME_SWAP_LOGIC_EXT, swapper_urefs)
            .into_uref()
            .unwrap_or_revert_with(ApiError::UnexpectedContractRefVariant);
    // let swap_function_pointer =
    //     storage::store_function_at_hash(NAME_SWAP_LOGIC_EXT, swapper_urefs)
    //         .into_uref()
    //         .unwrap_or_revert_with(ApiError::UnexpectedContractRefVariant);

    runtime::put_key(NAME_SWAP_UREF, swap_function_pointer.into());

    swap_proxy::deploy_swap_proxy(swap_function_pointer.clone());
}
