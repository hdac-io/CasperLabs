mod error;

use alloc::string::String;

use contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{account::PublicKey, ApiError, U512};

use error::Error;

mod proxy_method_names {
    use super::internal_method_names;

    pub const BOND: &str = internal_method_names::BOND;
    pub const UNBOND: &str = internal_method_names::UNBOND;
    pub const TRANSFER_TO_ACCOUNT: &str = "transfer_to_account";
}

mod internal_method_names {
    pub const BOND: &str = "bond";
    pub const UNBOND: &str = "unbond";
}

pub enum Api {
    Bond(u64),
    Unbond(Option<u64>),
    TransferToAccount(PublicKey, u64),
}

impl Api {
    pub fn from_args() -> Self {
        let method_name: String = runtime::get_arg(0)
            .unwrap_or_revert_with(ApiError::MissingArgument)
            .unwrap_or_revert_with(ApiError::InvalidArgument);

        match method_name.as_str() {
            proxy_method_names::BOND => {
                let amount: u64 = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::Bond(amount)
            }
            proxy_method_names::UNBOND => {
                let amount: Option<u64> = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::Unbond(amount)
            }
            proxy_method_names::TRANSFER_TO_ACCOUNT => {
                let public_key: PublicKey = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let transfer_amount: u64 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);

                Api::TransferToAccount(public_key, transfer_amount)
            }
            _ => runtime::revert(Error::UnknownProxyApi),
        }
    }

    pub fn invoke(&self) {
        match self {
            Self::Bond(amount) => {
                let pos_ref = system::get_proof_of_stake();
                let amount: U512 = (*amount).into();

                let source_purse = account::get_main_purse();
                let bonding_purse = system::create_purse();

                system::transfer_from_purse_to_purse(source_purse, bonding_purse, amount)
                    .unwrap_or_revert();

                runtime::call_contract(
                    pos_ref,
                    (internal_method_names::BOND, amount, bonding_purse),
                )
            }
            Self::Unbond(amount) => {
                let pos_ref = system::get_proof_of_stake();
                let amount: Option<U512> = amount.map(Into::into);
                runtime::call_contract(pos_ref, (internal_method_names::UNBOND, amount))
            }
            Self::TransferToAccount(public_key, amount) => {
                system::transfer_to_account(*public_key, (*amount).into()).unwrap_or_revert();
            }
        }
    }
}
