use core::result;
use std::{fmt, iter};

use num_traits::Zero;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use engine_shared::{motes::Motes, newtypes::Blake2bHash, TypeMismatch};
use engine_storage::global_state::CommitResult;
use engine_wasm_prep::wasm_costs::WasmCosts;
use types::{
    account::PublicKey,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, Key, ProtocolVersion, U512,
};

use crate::engine_state::execution_effect::ExecutionEffect;

pub const POS_BONDING_PURSE: &str = "pos_bonding_purse";
pub const POS_PAYMENT_PURSE: &str = "pos_payment_purse";
pub const POS_REWARDS_PURSE: &str = "pos_rewards_purse";

pub enum GenesisResult {
    RootNotFound,
    KeyNotFound(Key),
    TypeMismatch(TypeMismatch),
    Serialization(bytesrepr::Error),
    Success {
        post_state_hash: Blake2bHash,
        effect: ExecutionEffect,
    },
}

impl fmt::Display for GenesisResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            GenesisResult::RootNotFound => write!(f, "Root not found"),
            GenesisResult::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            GenesisResult::TypeMismatch(type_mismatch) => {
                write!(f, "Type mismatch: {:?}", type_mismatch)
            }
            GenesisResult::Serialization(error) => write!(f, "Serialization error: {:?}", error),
            GenesisResult::Success {
                post_state_hash,
                effect,
            } => write!(f, "Success: {} {:?}", post_state_hash, effect),
        }
    }
}

impl GenesisResult {
    pub fn from_commit_result(commit_result: CommitResult, effect: ExecutionEffect) -> Self {
        match commit_result {
            CommitResult::RootNotFound => GenesisResult::RootNotFound,
            CommitResult::KeyNotFound(key) => GenesisResult::KeyNotFound(key),
            CommitResult::TypeMismatch(type_mismatch) => GenesisResult::TypeMismatch(type_mismatch),
            CommitResult::Serialization(error) => GenesisResult::Serialization(error),
            CommitResult::Success { state_root, .. } => GenesisResult::Success {
                post_state_hash: state_root,
                effect,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GenesisAccount {
    public_key: PublicKey,
    balance: Motes,
    bonded_amount: Motes,
}

impl GenesisAccount {
    pub fn new(public_key: PublicKey, balance: Motes, bonded_amount: Motes) -> Self {
        GenesisAccount {
            public_key,
            balance,
            bonded_amount,
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn balance(&self) -> Motes {
        self.balance
    }

    pub fn bonded_amount(&self) -> Motes {
        self.bonded_amount
    }
}

impl Distribution<GenesisAccount> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GenesisAccount {
        let public_key = PublicKey::new(rng.gen());

        let mut u512_array = [0u8; 64];
        rng.fill_bytes(u512_array.as_mut());
        let balance = Motes::new(U512::from(u512_array.as_ref()));

        rng.fill_bytes(u512_array.as_mut());
        let bonded_amount = Motes::new(U512::from(u512_array.as_ref()));

        GenesisAccount {
            public_key,
            balance,
            bonded_amount,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DelegateKey {
    delegator: PublicKey,
    validator: PublicKey,
}

impl DelegateKey {
    pub fn new(delegator: PublicKey, validator: PublicKey) -> Self {
        DelegateKey {
            delegator,
            validator,
        }
    }

    pub fn delegator(&self) -> PublicKey {
        self.delegator
    }

    pub fn validator(&self) -> PublicKey {
        self.validator
    }
}

impl Distribution<DelegateKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DelegateKey {
        let delegator = PublicKey::new(rng.gen());
        let validator = PublicKey::new(rng.gen());
        DelegateKey {
            delegator,
            validator,
        }
    }
}

impl FromBytes for DelegateKey {
    fn from_bytes(bytes: &[u8]) -> result::Result<(Self, &[u8]), bytesrepr::Error> {
        let (delegator, bytes) = PublicKey::from_bytes(bytes)?;
        let (validator, bytes) = PublicKey::from_bytes(bytes)?;
        let entry = DelegateKey {
            delegator,
            validator,
        };
        Ok((entry, bytes))
    }
}

impl ToBytes for DelegateKey {
    fn to_bytes(&self) -> result::Result<Vec<u8>, bytesrepr::Error> {
        Ok((self.delegator.to_bytes()?.into_iter())
            .chain(self.validator.to_bytes()?)
            .collect())
    }
}

impl CLTyped for DelegateKey {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Delegator {
    delegator_key: DelegateKey,
    amount: Motes,
}

impl Delegator {
    pub fn new(delegator_key: DelegateKey, amount: Motes) -> Self {
        Delegator {
            delegator_key,
            amount,
        }
    }

    pub fn delegator_key(&self) -> DelegateKey {
        self.delegator_key
    }

    pub fn amount(&self) -> Motes {
        self.amount
    }
}

impl Distribution<Delegator> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Delegator {
        let delegator = PublicKey::new(rng.gen());
        let validator = PublicKey::new(rng.gen());
        let delegator_key = DelegateKey {
            delegator,
            validator,
        };

        let mut u512_array = [0u8; 64];
        rng.fill_bytes(u512_array.as_mut());
        let amount = Motes::new(U512::from(u512_array.as_ref()));

        Delegator {
            delegator_key,
            amount,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisConfig {
    name: String,
    timestamp: u64,
    protocol_version: ProtocolVersion,
    mint_installer_bytes: Vec<u8>,
    proof_of_stake_installer_bytes: Vec<u8>,
    accounts: Vec<GenesisAccount>,
    delegators: Vec<Delegator>,
    wasm_costs: WasmCosts,
}

impl GenesisConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        timestamp: u64,
        protocol_version: ProtocolVersion,
        mint_installer_bytes: Vec<u8>,
        proof_of_stake_installer_bytes: Vec<u8>,
        accounts: Vec<GenesisAccount>,
        delegators: Vec<Delegator>,
        wasm_costs: WasmCosts,
    ) -> Self {
        GenesisConfig {
            name,
            timestamp,
            protocol_version,
            mint_installer_bytes,
            proof_of_stake_installer_bytes,
            accounts,
            delegators,
            wasm_costs,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    pub fn mint_installer_bytes(&self) -> &[u8] {
        self.mint_installer_bytes.as_slice()
    }

    pub fn proof_of_stake_installer_bytes(&self) -> &[u8] {
        self.proof_of_stake_installer_bytes.as_slice()
    }

    pub fn wasm_costs(&self) -> WasmCosts {
        self.wasm_costs
    }

    pub fn get_bonded_validators(&self) -> impl Iterator<Item = (PublicKey, Motes)> + '_ {
        let zero = Motes::zero();
        self.accounts.iter().filter_map(move |genesis_account| {
            if genesis_account.bonded_amount() > zero {
                Some((
                    genesis_account.public_key(),
                    genesis_account.bonded_amount(),
                ))
            } else {
                None
            }
        })
    }

    pub fn get_delegated_delegator(&self) -> impl Iterator<Item = (DelegateKey, Motes)> + '_ {
        let zero = Motes::zero();
        self.delegators.iter().filter_map(move |delegator| {
            if delegator.amount() > zero {
                Some((delegator.delegator_key(), delegator.amount()))
            } else {
                None
            }
        })
    }

    pub fn accounts(&self) -> &[GenesisAccount] {
        self.accounts.as_slice()
    }

    pub fn push_account(&mut self, account: GenesisAccount) {
        self.accounts.push(account);
    }

    pub fn delegators(&self) -> &[Delegator] {
        self.delegators.as_slice()
    }

    pub fn push_delegator(&mut self, delegator: Delegator) {
        self.delegators.push(delegator);
    }
}

impl Distribution<GenesisConfig> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GenesisConfig {
        let mut count = rng.gen_range(1, 1000);
        let name = iter::repeat(())
            .map(|_| rng.gen::<char>())
            .take(count)
            .collect();

        let timestamp = rng.gen();

        let protocol_version = ProtocolVersion::from_parts(rng.gen(), rng.gen(), rng.gen());

        count = rng.gen_range(1000, 10_000);
        let mint_installer_bytes = iter::repeat(()).map(|_| rng.gen()).take(count).collect();

        count = rng.gen_range(1000, 10_000);
        let proof_of_stake_installer_bytes =
            iter::repeat(()).map(|_| rng.gen()).take(count).collect();

        count = rng.gen_range(1, 10);
        let accounts = iter::repeat(()).map(|_| rng.gen()).take(count).collect();

        let delegators = iter::repeat(()).map(|_| rng.gen()).take(count).collect();

        let wasm_costs = WasmCosts {
            regular: rng.gen(),
            div: rng.gen(),
            mul: rng.gen(),
            mem: rng.gen(),
            initial_mem: rng.gen(),
            grow_mem: rng.gen(),
            memcpy: rng.gen(),
            max_stack_height: rng.gen(),
            opcodes_mul: rng.gen(),
            opcodes_div: rng.gen(),
        };

        GenesisConfig {
            name,
            timestamp,
            protocol_version,
            mint_installer_bytes,
            proof_of_stake_installer_bytes,
            accounts,
            delegators,
            wasm_costs,
        }
    }
}
