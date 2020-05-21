#![no_std]

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{ApiError, URef};

const KEY_ADMIN: &str = "admin";
const NAME_SWAP: &str = "swap";
const NAME_SWAP_LOGIC_EXT: &str = "swap_logic_ext";

#[no_mangle]
pub extern "C" fn swap_logic_ext() {
    swap_logic::delegate();
}

#[no_mangle]
pub extern "C" fn call() {
    // Get caller's public key and store as admin
    let admin_uref: URef = storage::new_uref(runtime::get_caller());
    runtime::put_key(KEY_ADMIN, admin_uref.into());

    // Swap function storage
    let swap_function_pointer = storage::store_function_at_hash(NAME_SWAP_LOGIC_EXT, Default::default());
    runtime::put_key(NAME_SWAP, swap_function_pointer.into());
}
