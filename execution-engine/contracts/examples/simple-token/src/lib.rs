#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String};

use contract::{
    contract_api::{runtime, storage, TURef},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{ContractRef, Key, CLValue, U512, ApiError};
use core::convert::TryInto;

const CONTRACT_NAME: &str = "prac_token";
const CONTRACT_EXT_NAME: &str = "prac_token_ext";
const CONTRACT_PROXY_NAME: &str = "prac_token_proxy";
const TOKEN_MAP_NAME: &str = "token_map";
const TOTAL_SUPPLY_NAME: &str = "total_supply";

fn init_token_db(){
    // Token value storage
    let init_token_map: BTreeMap<Key, U512> = BTreeMap::new();
    let token_map_turef: TURef<BTreeMap<Key, U512>> = storage::new_turef(init_token_map);

    // Total supply
    let total_supply: U512 = U512::from(0);
    let token_supply_turef: TURef<U512> = storage::new_turef(total_supply);

    // For stored contract
    let mut stored_contract_urefs: BTreeMap<String, Key> = BTreeMap::new();
    stored_contract_urefs.insert(String::from(TOKEN_MAP_NAME), token_map_turef.into());
    stored_contract_urefs.insert(String::from(TOTAL_SUPPLY_NAME), token_supply_turef.into());
}

fn mint(address: Key, value: U512) -> Option<bool>{
    if runtime::has_key(TOKEN_MAP_NAME) {
        let token_map_uref: TURef<BTreeMap<Key, U512>> = runtime::get_key(TOKEN_MAP_NAME)
            .unwrap_or_revert_with(ApiError::GetKey)
            .try_into().unwrap_or_revert();
        let mut token_map = storage::read(token_map_uref.clone())
            .unwrap_or_revert_with(ApiError::Read)
            .unwrap_or_revert_with(ApiError::ValueNotFound);

        // If token_map has given address, update
        // Or, insert value
        let val = token_map.entry(address).or_insert(U512::from(0));
        *val += value;

        storage::write(token_map_uref, token_map);
    }

    if runtime::has_key(TOTAL_SUPPLY_NAME) {
        let total_supply_uref: TURef<U512> = runtime::get_key(TOTAL_SUPPLY_NAME)
            .unwrap_or_revert()
            .try_into().unwrap_or_revert();
        let total_supply_opt = storage::read(total_supply_uref)
            .unwrap_or_revert();
        let total_supply_after = match total_supply_opt {
            Some(total_supply_before) => {
                total_supply_before + value
            }
            _ => panic!("Total supply calc error!"),
        };
        storage::write(total_supply_uref, total_supply_after);
    }

    Some(true)
}

fn get_total_supply() -> Option<U512>{
    let total_supply_uref: TURef<U512> = runtime::get_key(TOTAL_SUPPLY_NAME)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into().unwrap_or_revert();
    let total_supply = storage::read(total_supply_uref)
        .unwrap_or_revert();
    
    total_supply
}

#[no_mangle]
pub extern "C" fn call() {
    let method_name: String = runtime::get_arg(0)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);

    match method_name.as_str() {
        "init" => init_token_db(),
        "mint" => {
            let address: Key = runtime::get_arg(1)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::MissingArgument);
            let amount: U512 = runtime::get_arg(2)
                .unwrap_or_revert_with(ApiError::MissingArgument)
                .unwrap_or_revert_with(ApiError::MissingArgument);
            let result: Option<bool> = mint(address, amount);
            match result {
                Some(true) => {
                    let ret = CLValue::from_t(true).unwrap_or_revert();
                    runtime::ret(ret)
                }

                _ => panic!("Error in init step!"),
            }
        }
        "total_supply" => {
            match get_total_supply() {
                Some(total_supply) => {
                    let ret = CLValue::from_t(total_supply).unwrap_or_revert();
                    runtime::ret(ret)
                }
                _ => panic!("Error to get total supply."),
            }
        }
        _ => panic!("No defined case"),
    }
}
