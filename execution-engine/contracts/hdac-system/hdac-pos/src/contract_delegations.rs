use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};
use core::{fmt::Write, result};

use contract::contract_api::runtime;
use types::{
    account::PublicKey,
    bytesrepr::{self, FromBytes, ToBytes},
    system_contract_errors::pos::{Error, Result},
    CLType, CLTyped, Key, U512,
};

pub struct ContractDelegations;
pub struct Delegations(BTreeMap<DelegationKey, U512>);

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct DelegationKey {
    delegator: PublicKey,
    validator: PublicKey,
}

impl ContractDelegations {
    pub fn read() -> Result<Delegations> {
        let mut delegations = BTreeMap::new();
        for (name, _) in runtime::list_named_keys() {
            let mut split_name = name.split('_');
            if Some("d") != split_name.next() {
                continue;
            }
            let hex_key = split_name
                .next()
                .ok_or(Error::StakesKeyDeserializationFailed)?;
            if hex_key.len() != 64 {
                return Err(Error::StakesKeyDeserializationFailed);
            }
            let delegator = Self::to_publickey(hex_key)?;

            let hex_key = split_name
                .next()
                .ok_or(Error::StakesKeyDeserializationFailed)?;
            if hex_key.len() != 64 {
                return Err(Error::StakesKeyDeserializationFailed);
            }
            let validator = Self::to_publickey(hex_key)?;

            let balance = split_name
                .next()
                .and_then(|b| U512::from_dec_str(b).ok())
                .ok_or(Error::StakesDeserializationFailed)?;

            delegations.insert(
                DelegationKey {
                    delegator,
                    validator,
                },
                balance,
            );
        }
        if delegations.is_empty() {
            return Err(Error::StakesNotFound);
        }
        Ok(Delegations(delegations))
    }

    /// Writes the current stakes to the contract's known urefs.
    pub fn write(delegations: &Delegations) {
        // Encode the stakes as a set of uref names.
        let mut new_urefs: BTreeSet<String> = delegations
            .0
            .iter()
            .map(|(delegation_key, balance)| {
                let delegator = Self::to_hex_string(delegation_key.delegator);
                let validator = Self::to_hex_string(delegation_key.validator);
                let mut uref = String::new();
                uref.write_fmt(format_args!("d_{}_{}_{}", delegator, validator, balance))
                    .expect("Writing to a string cannot fail");
                uref
            })
            .collect();
        // Remove and add urefs to update the contract's known urefs accordingly.
        for (name, _) in runtime::list_named_keys() {
            if name.starts_with("d_") && !new_urefs.remove(&name) {
                runtime::remove_key(&name);
            }
        }
        for name in new_urefs {
            runtime::put_key(&name, Key::Hash([0; 32]));
        }
    }

    fn to_hex_string(address: PublicKey) -> String {
        let bytes = address.value();
        let mut ret = String::with_capacity(64);
        for byte in &bytes[..32] {
            write!(ret, "{:02x}", byte).expect("Writing to a string cannot fail");
        }
        ret
    }

    fn to_publickey(hex_str: &str) -> Result<PublicKey> {
        let mut key_bytes = [0u8; 32];
        let _bytes_written = base16::decode_slice(hex_str, &mut key_bytes)
            .map_err(|_| Error::StakesKeyDeserializationFailed)?;
        debug_assert!(_bytes_written == key_bytes.len());
        Ok(PublicKey::from(key_bytes))
    }
}

impl Delegations {
    pub fn delegate(&mut self, delegator: PublicKey, validator: PublicKey, amount: U512) {
        let key = DelegationKey {
            delegator,
            validator,
        };
        self.0
            .entry(key)
            .and_modify(|x| *x += amount)
            .or_insert(amount);
    }
}
