<<<<<<< HEAD
<<<<<<< HEAD
=======
use contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
>>>>>>> chore: obeyed the linter majesty
=======
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use contract::{contract_api::{runtime, system}, unwrap_or_revert::UnwrapOrRevert};
>>>>>>> feat: (!Buildable) introduce of economy contract
use proof_of_stake::{MintProvider, ProofOfStake, RuntimeProvider, Stakes, StakesProvider};
use types::{
    account::{PublicKey, PurseId},
    system_contract_errors::{
        pos::{Error, PurseLookupError, Result},
        mint,
    },
    Key, URef, U512,
};

use crate::{
    constants::{consts, methods, uref_names},
    contract_delegations::{ContractDelegations, DelegationUnitForOrder},
    contract_mint::ContractMint,
    contract_queue::ContractQueue,
    contract_runtime::ContractRuntime,
    contract_stakes::ContractStakes,
    contract_votes::{ContractVotes, VoteStat, Votes},
    contract_economy::{
        ContractClaim,
        Commissions,
        Rewards,
        TotalSupply,
        sum_of_delegation,
        pop_score_calculation,
        dapp_gas_deduction_rate_calculation,
        u512ToF64,
        f64ToU512,
    }
};

pub struct ProofOfProfessionContract;

impl ProofOfStake<ContractMint, ContractQueue, ContractRuntime, ContractStakes>
    for ProofOfProfessionContract
{
    fn bond(&self, _: PublicKey, _: U512, _: URef) -> Result<()> {
        Err(Error::NotSupportedFunc)
    }

    fn unbond(&self, _: PublicKey, _: Option<U512>) -> Result<()> {
        Err(Error::NotSupportedFunc)
    }

    fn step(&self) -> Result<()> {
        Err(Error::NotSupportedFunc)
    }
}

impl ProofOfProfessionContract {
    pub fn delegate(
        &self,
        delegator: PublicKey,
        validator: PublicKey,
        amount: U512,
        source_purse: URef,
    ) -> Result<()> {
        // transfer amount to pos_bonding_purse
        if amount.is_zero() {
            return Err(Error::BondTooSmall);
        }
        let source = PurseId::new(source_purse);
        let pos_purse = get_purse_id::<ContractRuntime>(uref_names::POS_BONDING_PURSE)
            .map_err(PurseLookupError::bonding)?;

        ContractMint::transfer_from_purse_to_purse(source, pos_purse, amount)
            .map_err(|_| Error::BondTransferFailed)?;

        // TODO: enqueue a new item and dequeue items to process

        // increase validator's staked token amount
        let mut stakes: Stakes = ContractStakes::read()?;

        // if this is not self-delegation and target validator is not bonded
        if delegator != validator && !stakes.0.contains_key(&validator) {
            return Err(Error::NotBonded);
        }

        stakes.bond(&validator, amount);
        ContractStakes::write(&stakes);

        // update delegation table.
        let mut delegations = ContractDelegations::read()?;
        delegations.delegate(&delegator, &validator, amount);
        ContractDelegations::write(&delegations);

        Ok(())
    }

    pub fn undelegate(
        &self,
        delegator: PublicKey,
        validator: PublicKey,
        maybe_amount: Option<U512>,
    ) -> Result<()> {
        // TODO: enqueue a new item and dequeue items to process
        let mut delegations = ContractDelegations::read()?;
        let amount = delegations.undelegate(&delegator, &validator, maybe_amount)?;
        ContractDelegations::write(&delegations);

        let mut stakes = ContractStakes::read()?;
        let payout = stakes.unbond(&validator, Some(amount))?;
        ContractStakes::write(&stakes);

        let pos_purse = get_purse_id::<ContractRuntime>(uref_names::POS_BONDING_PURSE)
            .map_err(PurseLookupError::bonding)?;

        ContractMint::transfer_from_purse_to_account(pos_purse, delegator, payout)
            .map_err(|_| Error::UnbondTransferFailed)?;
        Ok(())
    }

    pub fn redelegate(
        &self,
        delegator: PublicKey,
        src: PublicKey,
        dest: PublicKey,
        amount: U512,
    ) -> Result<()> {
        if src == dest {
            return Err(Error::SelfRedelegation);
        }

        let mut delegations = ContractDelegations::read()?;
        let amount = delegations.undelegate(&delegator, &src, Some(amount))?;
        delegations.delegate(&delegator, &dest, amount);
        ContractDelegations::write(&delegations);

        let mut stakes = ContractStakes::read()?;
        let payout = stakes.unbond(&src, Some(amount))?;
        stakes.bond(&dest, payout);
        ContractStakes::write(&stakes);

        Ok(())
    }

    pub fn vote(&self, user: PublicKey, dapp: Key, amount: U512) -> Result<()> {
        // staked balance check
        if amount.is_zero() {
            return Err(Error::BondTooSmall);
        }

        // check validator's staked token amount
        let delegation_user_stat = ContractDelegations::read_user_stat()?;
        // if an user has no staked amount, he cannot do anything
        let delegated_balance: U512 = match delegation_user_stat.0.get(&user) {
            Some(balance) => *balance,
            None => return Err(Error::DelegationsNotFound),
        };

        // check user's vote stat
        let vote_stat: VoteStat = ContractVotes::read_stat()?;
        let vote_stat_per_user: U512 = vote_stat
            .0
            .get(&user)
            .cloned()
            .unwrap_or_else(|| U512::from(0));

        if delegated_balance < vote_stat_per_user + amount {
            return Err(Error::VoteTooLarge);
        }

        // check vote table
        let mut votes: Votes = ContractVotes::read()?; // <- here
        votes.vote(&user, &dapp, amount);
        ContractVotes::write(&votes);

        Ok(())
    }

    pub fn unvote(&self, user: PublicKey, dapp: Key, maybe_amount: Option<U512>) -> Result<()> {
        let mut votes = ContractVotes::read()?;
        votes.unvote(&user, &dapp, maybe_amount)?;
        ContractVotes::write(&votes);

        Ok(())
    }

    pub fn write_genesis_total_supply(genesis_total_supply: &U512) {
        let total_supply = TotalSupply(*genesis_total_supply);
        ContractClaim::write_total_supply(&total_supply);
    }

    pub fn distribute() {
        // 1. Increase total supply
        // 2. Do not mint in this phase.
        let mut total_supply = ContractClaim::read_total_supply()?;

        let delegations = ContractDelegations::read()?;
        let delegation_stat = ContractDelegations::read_stat()?;
        let delegation_sorted_stat = ContractDelegations::get_sorted_stat(&delegation_stat)?;

        let mut commissions = ContractClaim::read_commission()?;
        let mut rewards = ContractClaim::read_reward()?;

        // 1. Increase total supply
        //   U512::from(5) / U512::from(100) -> total inflation 5% per year
        //   U512::from(consts::DAYS_OF_YEAR * consts::HOURS_OF_DAY * consts::SECONDS_OF_HOUR / consts::BLOCK_TIME_IN_SEC)
        //      -> divider for deriving inflation per block
        let inflation_pool_per_block = total_supply.0 * U512::from(5) * U512::from(consts::CONV_RATE) / U512::from(100 * consts::DAYS_OF_YEAR * consts::HOURS_OF_DAY * consts::SECONDS_OF_HOUR / consts::BLOCK_TIME_IN_SEC);
        total_supply.add(&inflation_pool_per_block);
        ContractClaim::write_total_supply(&total_supply);

        /////////////////////////////////
        // Update validator's commission
        /////////////////////////////////
        //
        // 1. Check total delegations
        // 2. Pick 100 validators
        // 3. Summize it to derive total PoP.
        // 4. Calculate commission & add to commission claim table
        
        // Check total delegations
        let total_delegation: U512 = delegation_stat.0.iter().map(|(_, v)| v).sum();

        // Pick 100 validators + Summize it to derive total PoP
        let mut idx = 0;
        let mut total_pop_score = 0_f64;
        let mut pop_score_table: BTreeMap<PublicKey, U512> = BTreeMap::new();
        for unit_data in delegation_sorted_stat {
            let unit_pop_score = pop_score_calculation(&total_delegation, *unit_data.amount);

            total_pop_score += unit_pop_score;
            pop_score_table.insert(unit_data.validator, U512::from(unit_pop_score as i64));

            idx += 1;
            if idx >= 100 {
                break;
            }
        }

        for (validator, unit_pop_score) in pop_score_table.iter() {
            let unit_commission = unit_pop_score * consts::VALIDATOR_COMMISSION_RATE_IN_PERCENTAGE * inflation_pool_per_block / U512::from((total_pop_score * 100_f64) as i64);
            commissions.insert_commission(validator, &unit_commission);
        }
        ContractClaim::write_commission(&commissions);

        /////////////////////////////////
        // Update user's reward
        /////////////////////////////////
        // 1. Swipe delegation table, and derive user's portion of delegation
        // 2. Lookup delegation_stat table for total delegation for each validator
        // 3. Derive each validator's reward portion and insert reward of each user

        // 1. Swipe delegation table, and derive user's portion of delegation
        for (delegation_key, user_delegation_amount) in delegations.0.iter() {
            // 2. Lookup delegation_stat table for total delegation for each validator
            let total_delegation_per_validator = delegation_stat.0.get(&delegation_key.validator).unwrap_or_revert_with(Error::DelegationsKeyDeserializationFailed);

            // 3. Derive each validator's reward portion and insert reward of each user
            let pop_score_of_validator = pop_score_table.get(&delegation_key.validator).unwrap_or_revert_with(Error::DelegationsKeyDeserializationFailed);
            let user_reward = user_delegation_amount * pop_score_of_validator * U512::from(100 - consts::VALIDATOR_COMMISSION_RATE_IN_PERCENTAGE) * inflation_pool_per_block / (U512::from((total_pop_score * 100_f64) as i64) * total_delegation_per_validator);

            rewards.insert_rewards(&delegation_key.delegator, &user_reward);
        }
        ContractClaim::write_reward(&rewards);
    }

    // For validator
    pub fn claim_commission(validator: &PublicKey) {
        let mut commissions = ContractClaim::read_commission()?;
        let validator_commission = commissions.0.get(validator).unwrap_or_revert_with(Error::RewardNotFound);

        // 1. Mint to system account
        // 2. Transfer from system account to claimer
        let mint_contract_uref = system::get_mint();

        let money_uref: Result<URef, mint::Error> = runtime::call_contract(mint_contract_uref, (methods::METHOD_MINT, *validator_commission))?;
        let temp_purse = money_uref.map(PurseId::new).unwrap_or_revert_with(Error::CommissionPurseNotFound);
        system::transfer_from_purse_to_account(temp_purse, *validator, *validator_commission);

        commissions.claim_commission(validator, validator_commission);
        ContractClaim::write_commission(&commissions);
    }

    // For user
    pub fn claim_reward(user: &PublicKey) {
        let mut rewards = ContractClaim::read_reward()?;
        let user_reward = rewards.0.get(user).unwrap_or_revert_with(Error::RewardNotFound);

        // 1. Mint to system account
        // 2. Transfer from system account to claimer
        let mint_contract_uref = system::get_mint();

        let money_uref: Result<URef, mint::Error> = runtime::call_contract(mint_contract_uref, (methods::METHOD_MINT, *user_reward));
        let temp_purse = money_uref.map(PurseId::new).unwrap_or_revert_with(Error::RewardsPurseNotFound);
        system::transfer_from_purse_to_account(temp_purse, *user, *user_reward);

        rewards.claim_rewards(user, user_reward);
        ContractClaim::write_reward(&rewards);
    }
}

fn get_purse_id<R: RuntimeProvider>(name: &str) -> core::result::Result<PurseId, PurseLookupError> {
    R::get_key(name)
        .ok_or(PurseLookupError::KeyNotFound)
        .and_then(|key| match key {
            Key::URef(uref) => Ok(PurseId::new(uref)),
            _ => Err(PurseLookupError::KeyUnexpectedType),
        })
}
