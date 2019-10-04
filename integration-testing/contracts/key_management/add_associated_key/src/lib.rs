#![no_std]
#![feature(cell_update)]

extern crate alloc;
extern crate contract_ffi;

use contract_ffi::contract_api::{add_associated_key, get_arg, revert, Error};
use contract_ffi::value::account::{PublicKey, Weight};

#[no_mangle]
pub extern "C" fn call() {
    let account: PublicKey = get_arg(0).unwrap().unwrap();
    let weight_val: u32 = get_arg(1).unwrap().unwrap();
    let weight = Weight::new(weight_val as u8);

    add_associated_key(account, weight).unwrap_or_else(|_| revert(Error::User(100)));
}
