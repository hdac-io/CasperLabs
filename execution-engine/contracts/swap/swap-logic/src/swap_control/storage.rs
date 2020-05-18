use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use core::{
    fmt::Write,
    convert::TryInto
};
use num_traits::Num;

use contract::{
    contract_api::{runtime, storage as contract_storage},
    unwrap_or_revert::UnwrapOrRevert
};

use types::{
    account::PublicKey, ApiError, URef, U512,
    system_contract_errors::pos::Error,
};

use crate::constants::{keys, users};

// struct UnitSwapData {
//     new_mainnet_addr: PublicKey - 32-byted address. This will be bech32fied in string representation
//     amount: U512 - balance at snapshot in BIGSUN unit
//     kyc_level: U512 - Categorized by holding amount
//     is_sent_token_for_swap: U512 - flag for tiny token transfer from company to holder
//     kyc_step: U512 - KYC done or not
//     swapped_amount: U512 - how much swapped already
// }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitSwapData {
    pub new_mainnet_addr: PublicKey,
    pub prev_balance: U512,
    pub kyc_level: U512,
    pub is_sent_token_for_swap: U512,
    pub kyc_step: U512,
    pub swapped_amount: U512,
}

impl UnitSwapData {
    pub fn restore(unit_tree: BTreeMap<String, String>) -> Self {
        let to_publickey = |hex_str: &str| -> Result<PublicKey, Error> {
            if hex_str.len() != 64 {
                return Err(Error::CommissionKeyDeserializationFailed);
            }
            let mut key_bytes = [0u8; 32];
            let _bytes_written = base16::decode_slice(hex_str, &mut key_bytes)
                .map_err(|_| Error::CommissionKeyDeserializationFailed)?;
            debug_assert!(_bytes_written == key_bytes.len());
            Ok(PublicKey::ed25519_from(key_bytes))
        };

        let new_mainnet_addr = to_publickey(unit_tree.get(keys::KEY_NEW_MAINNET_ADDR_KEY).unwrap_or_revert()).unwrap_or_revert();

        let prev_balance = U512::from_str_radix(unit_tree.get(keys::KEY_PREV_BALANCE_KEY).unwrap_or_revert(), 10).unwrap_or_default();
        let kyc_level = U512::from_str_radix(unit_tree.get(keys::KEY_KYC_LEVEL).unwrap_or_revert(), 10).unwrap_or_default();
        let is_sent_token_for_swap = U512::from_str_radix(unit_tree.get(keys::KEY_IS_SENT_TOKEN_FOR_SWAP).unwrap_or_revert(), 10).unwrap_or_default();
        let kyc_step = U512::from_str_radix(unit_tree.get(keys::KEY_KYC_STEP).unwrap_or_revert(), 10).unwrap_or_default();
        let swapped_amount = U512::from_str_radix(unit_tree.get(keys::KEY_SWAPPED_AMOUNT).unwrap_or_revert(), 10).unwrap_or_default();

        UnitSwapData {
            new_mainnet_addr: new_mainnet_addr,
            prev_balance: prev_balance,
            kyc_level: kyc_level,
            is_sent_token_for_swap: is_sent_token_for_swap,
            kyc_step: kyc_step,
            swapped_amount: swapped_amount,
        }
    }

    pub fn organize(&self) -> BTreeMap<String, String> {
        let to_hex_string = |address: PublicKey| -> String {
            let bytes = address.value();
            let mut ret = String::with_capacity(64);
            for byte in &bytes[..32] {
                write!(ret, "{:02x}", byte).expect("Writing to a string cannot fail");
            }
            ret
        };

        let new_mainnet_addr_string = to_hex_string(self.new_mainnet_addr);

        let mut prev_balance = String::new();
        prev_balance.write_fmt(format_args!("{}", self.prev_balance)).unwrap_or_default();

        let mut kyc_level = String::new();
        kyc_level.write_fmt(format_args!("{}", self.kyc_level)).unwrap_or_default();

        let mut is_sent_token_for_swap = String::new();
        is_sent_token_for_swap.write_fmt(format_args!("{}", self.is_sent_token_for_swap)).unwrap_or_default();

        let mut kyc_step = String::new();
        kyc_step.write_fmt(format_args!("{}", self.kyc_step)).unwrap_or_default();

        let mut swapped_amount = String::new();
        swapped_amount.write_fmt(format_args!("{}", self.swapped_amount)).unwrap_or_default();

        let mut res: BTreeMap<String, String> = BTreeMap::new();
        res.insert(keys::KEY_NEW_MAINNET_ADDR_KEY.to_string(), new_mainnet_addr_string);
        res.insert(keys::KEY_PREV_BALANCE_KEY.to_string(), prev_balance);
        res.insert(keys::KEY_KYC_LEVEL.to_string(), kyc_level);
        res.insert(keys::KEY_IS_SENT_TOKEN_FOR_SWAP.to_string(), is_sent_token_for_swap);
        res.insert(keys::KEY_KYC_STEP.to_string(), kyc_step);
        res.insert(keys::KEY_SWAPPED_AMOUNT.to_string(), swapped_amount);

        res
    }
}

pub fn load_data(ver1_pubkey: String) -> UnitSwapData {
    let data_key: URef = runtime::get_key(&ver1_pubkey)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into()
        .unwrap_or_revert();

    let data = contract_storage::read(data_key.clone())
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    UnitSwapData::restore(data)
}

pub fn save_data(ver1_pubkey: String, unit_data: UnitSwapData) {
    if runtime::has_key(&ver1_pubkey) {
        runtime::remove_key(&ver1_pubkey);
    }

    let new_data_uref = contract_storage::new_uref(unit_data.organize());
    runtime::put_key(&ver1_pubkey, new_data_uref.into());
}

pub fn load_admin() -> PublicKey {
    let admin_pubkey_uref: URef = runtime::get_key(users::KEY_ADMIN)
        .unwrap_or_revert_with(ApiError::GetKey)
        .try_into()
        .unwrap_or_revert();

    contract_storage::read(admin_pubkey_uref.clone())
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound)
}
