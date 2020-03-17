use contract::contract_api::system;
use proof_of_stake::MintProvider;
use types::{
    account::{PublicKey, PurseId},
    TransferResult, U512,
};

pub struct ContractMint;

impl MintProvider for ContractMint {
    fn transfer_from_purse_to_account(
        source: PurseId,
        target: PublicKey,
        amount: U512,
    ) -> TransferResult {
        system::transfer_from_purse_to_account(source, target, amount)
    }

    fn transfer_from_purse_to_purse(
        source: PurseId,
        target: PurseId,
        amount: U512,
    ) -> Result<(), ()> {
        system::transfer_from_purse_to_purse(source, target, amount).map_err(|_| ())
    }

    fn get_balance(purse: PurseId) -> Option<U512> {
        system::get_balance(purse)
    }
}
