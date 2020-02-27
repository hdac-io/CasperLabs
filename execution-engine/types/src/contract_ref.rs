use crate::{Key, URef};

/// A reference to a smart contract stored on the network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractRef {
    /// The pseudo-hash under which the contract is stored.
    Hash([u8; 32]),
    /// The [`URef`] under which the contract is stored.
    URef(URef),
}

impl ContractRef {
    /// Tries to convert `self` into a [`URef`].
    pub fn into_uref(self) -> Option<URef> {
        match self {
            ContractRef::URef(ret) => Some(ret),
            _ => None,
        }
    }
}

impl From<ContractRef> for Key {
    fn from(contract_ref: ContractRef) -> Self {
        match contract_ref {
            ContractRef::Hash(h) => Key::Hash(h),
            ContractRef::URef(uref) => uref.into(),
        }
    }
}
