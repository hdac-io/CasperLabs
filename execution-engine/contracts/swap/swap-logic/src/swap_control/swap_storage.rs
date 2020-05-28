use crate::constants::{keys, users};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use contract::{
    contract_api::{runtime, storage as contract_storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use core::{convert::TryInto, fmt::Write};
use num_traits::Num;
use types::{account::PublicKey, ApiError, URef, U512};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitSnapshotData {
    pub prev_balance: U512,
    pub is_swapped: U512,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitKYCData {
    pub kyc_level: U512,
    pub is_sent_token_for_swap: U512,
    pub kyc_step: U512,
    pub swapped_amount: U512,
}

impl UnitSnapshotData {
    pub fn restore(unit_tree: BTreeMap<String, String>) -> Self {
        let prev_balance = U512::from_str_radix(
            unit_tree.get(keys::KEY_PREV_BALANCE_KEY).unwrap_or_revert(),
            10,
        )
        .unwrap_or_default();
        let is_swapped =
            U512::from_str_radix(unit_tree.get(keys::KEY_IS_SWAPPED).unwrap_or_revert(), 10)
                .unwrap_or_default();

        UnitSnapshotData {
            prev_balance,
            is_swapped,
        }
    }

    pub fn organize(&self) -> BTreeMap<String, String> {
        let mut res: BTreeMap<String, String> = BTreeMap::new();
        let mut prev_balance = String::new();
        prev_balance
            .write_fmt(format_args!("{}", self.prev_balance))
            .unwrap_or_default();

        let mut is_swapped = String::new();
        is_swapped
            .write_fmt(format_args!("{}", self.is_swapped))
            .unwrap_or_default();

        res.insert(keys::KEY_PREV_BALANCE_KEY.to_string(), prev_balance);
        res.insert(keys::KEY_IS_SWAPPED.to_string(), is_swapped);

        res
    }
}

impl UnitKYCData {
    pub fn restore(unit_tree: BTreeMap<String, String>) -> Self {
        let kyc_level =
            U512::from_str_radix(unit_tree.get(keys::KEY_KYC_LEVEL).unwrap_or_revert(), 10)
                .unwrap_or_default();
        let is_sent_token_for_swap = U512::from_str_radix(
            unit_tree
                .get(keys::KEY_IS_SENT_TOKEN_FOR_SWAP)
                .unwrap_or_revert(),
            10,
        )
        .unwrap_or_default();
        let kyc_step =
            U512::from_str_radix(unit_tree.get(keys::KEY_KYC_STEP).unwrap_or_revert(), 10)
                .unwrap_or_default();
        let swapped_amount = U512::from_str_radix(
            unit_tree.get(keys::KEY_SWAPPED_AMOUNT).unwrap_or_revert(),
            10,
        )
        .unwrap_or_default();

        UnitKYCData {
            kyc_level,
            is_sent_token_for_swap,
            kyc_step,
            swapped_amount,
        }
    }

    pub fn organize(&self) -> BTreeMap<String, String> {
        let mut kyc_level = String::new();
        kyc_level
            .write_fmt(format_args!("{}", self.kyc_level))
            .unwrap_or_default();

        let mut is_sent_token_for_swap = String::new();
        is_sent_token_for_swap
            .write_fmt(format_args!("{}", self.is_sent_token_for_swap))
            .unwrap_or_default();

        let mut kyc_step = String::new();
        kyc_step
            .write_fmt(format_args!("{}", self.kyc_step))
            .unwrap_or_default();

        let mut swapped_amount = String::new();
        swapped_amount
            .write_fmt(format_args!("{}", self.swapped_amount))
            .unwrap_or_default();

        let mut res: BTreeMap<String, String> = BTreeMap::new();
        res.insert(keys::KEY_KYC_LEVEL.to_string(), kyc_level);
        res.insert(
            keys::KEY_IS_SENT_TOKEN_FOR_SWAP.to_string(),
            is_sent_token_for_swap,
        );
        res.insert(keys::KEY_KYC_STEP.to_string(), kyc_step);
        res.insert(keys::KEY_SWAPPED_AMOUNT.to_string(), swapped_amount);

        res
    }
}

pub fn load_snapshot_data(ver1_address: String) -> UnitSnapshotData {
    let data_key: URef = runtime::get_key(&ver1_address)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into()
        .unwrap_or_revert();

    let data = contract_storage::read(data_key)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    UnitSnapshotData::restore(data)
}

pub fn save_snapshot_data(ver1_address: String, unit_data: UnitSnapshotData) {
    if runtime::has_key(&ver1_address) {
        runtime::remove_key(&ver1_address);
    }

    let new_data_uref = contract_storage::new_uref(unit_data.organize());
    runtime::put_key(&ver1_address, new_data_uref.into());
}

pub fn load_kyc_data(new_address: PublicKey) -> UnitKYCData {
    let str_new_address = to_hex_string(new_address);
    let data_key: URef = runtime::get_key(&str_new_address)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into()
        .unwrap_or_revert();

    let data = contract_storage::read(data_key)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    UnitKYCData::restore(data)
}

pub fn save_kyc_data(new_address: PublicKey, unit_data: UnitKYCData) {
    let str_new_address = to_hex_string(new_address);

    if runtime::has_key(&str_new_address) {
        runtime::remove_key(&str_new_address);
    }

    let new_data_uref = contract_storage::new_uref(unit_data.organize());
    runtime::put_key(&str_new_address, new_data_uref.into());
}

pub fn to_hex_string(address: PublicKey) -> String {
    let bytes = address.value();
    let mut ret = String::with_capacity(64);
    for byte in &bytes[..32] {
        write!(ret, "{:02x}", byte).expect("Writing to a string cannot fail");
    }

    ret
}

pub fn load_admin() -> PublicKey {
    let admin_pubkey_uref: URef = runtime::get_key(users::KEY_ADMIN)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into()
        .unwrap_or_revert();

    contract_storage::read(admin_pubkey_uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound)
}

pub fn load_kyc_border_allowance_cap() -> U512 {
    let kyc_border_allowance_uref: URef = runtime::get_key(keys::KEY_KYC_BORDER_ALLOWANCE_CAP)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into()
        .unwrap_or_revert();

    contract_storage::read(kyc_border_allowance_uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound)
}

pub fn save_kyc_border_allowance_cap(value: U512) {
    if runtime::has_key(keys::KEY_KYC_BORDER_ALLOWANCE_CAP) {
        runtime::remove_key(keys::KEY_KYC_BORDER_ALLOWANCE_CAP);
    }
    let new_data_uref = contract_storage::new_uref(value);
    runtime::put_key(keys::KEY_KYC_BORDER_ALLOWANCE_CAP, new_data_uref.into());
}
