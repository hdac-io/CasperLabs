#![no_std]

extern crate alloc;

mod client_api;

use alloc::{
    string::String,
    collections::BTreeMap,
};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{URef, Key, ApiError};

use client_api::{Api, method_names};

const SWAP_PROXY_NAME: &str = "swap_proxy";

#[no_mangle]
pub extern "C" fn swap_proxy() {
    Api::from_args().invoke();
}

pub fn deploy_swap_proxy(swap_contract_uref: URef) {
    let mut named_key: BTreeMap<String, Key> = BTreeMap::new();
    named_key.insert(String::from(method_names::proxy::NAME_SWAP_UREF), swap_contract_uref.into());

    let contract_hash = storage::store_function_at_hash(SWAP_PROXY_NAME, named_key);
    runtime::put_key(SWAP_PROXY_NAME, contract_hash.into());
}

#[cfg(not(feature = "lib"))]
#[no_mangle]
pub extern "C" fn call() {
    let swap_logic_uref: URef = runtime::get_arg(0)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);

    deploy_swap_proxy(swap_logic_uref);
}
