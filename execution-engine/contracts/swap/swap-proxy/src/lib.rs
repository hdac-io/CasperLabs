#![no_std]

extern crate alloc;
mod client_api;

use contract::contract_api::{runtime, storage};

use client_api::Api;

const SWAP_PROXY_NAME: &str = "swap_proxy";

#[no_mangle]
pub extern "C" fn swap_proxy() {
    Api::from_args().invoke();
}

pub fn deploy_swap_proxy() {
    let contract_hash = storage::store_function_at_hash(SWAP_PROXY_NAME, Default::default());
    runtime::put_key(SWAP_PROXY_NAME, contract_hash.into());
}

#[cfg(not(feature = "lib"))]
#[no_mangle]
pub extern "C" fn call() {
    deploy_swap_proxy();
}
