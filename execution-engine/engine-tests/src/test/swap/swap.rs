extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use core::convert::{TryInto, TryFrom};
use num_traits::identities::Zero;

use engine_core::engine_state::genesis::GenesisAccount;
use engine_shared::{motes::Motes, stored_value::StoredValue, account::Account, contract::Contract};
use engine_test_support::{
    internal::{utils, ExecuteRequestBuilder, InMemoryWasmTestBuilder},
    DEFAULT_ACCOUNT_INITIAL_BALANCE,
};
use types::{account::PublicKey, bytesrepr::ToBytes, U512, Key, URef, ContractRef, CLValue};
use contract::unwrap_or_revert::UnwrapOrRevert;

const CONTRACT_POS_VOTE: &str = "swap_install.wasm";
const BIGSUN_TO_HDAC: u64 = 1_000_000_000_000_000_000_u64;

const ADMIN_PUBKEY: PublicKey = PublicKey::ed25519_from([1u8; 32]);
const ACCOUNT_1_PUBKEY: PublicKey = PublicKey::ed25519_from([2u8; 32]);

const GENESIS_VALIDATOR_STAKE: u64 = 5u64 * BIGSUN_TO_HDAC;
const ACCOUNT_1_DELEGATE_AMOUNT: u64 = BIGSUN_TO_HDAC;
const SYSTEM_ACC_SUPPORT: u64 = 5u64 * BIGSUN_TO_HDAC;

const VER1_ADDRESS: &str = "HLkXSESzSaDZgU25CQrmxkjRayKfs5xBFK";
const VER1_PUBKEY: &str = "02c4ef70543e18889167ca67c8aa28c1d4c259e89cb34483a8ed6cfd3a03e8246b";
const VER1_MESSAGE_HASHED: &str = "69046d44e3d75d48436377626372a44a5066966b5d72c00b67769c1cc6a8619a";
const VER1_SIGNATURE: &str = "24899366fd3d5dfe6740df1e5f467a53f1a3aaafce26d8df1497a925c55b5c266339a95fe6\
                              507bd611b0e3b6e74e3bb7f19eeb1165615e5cebe7f40e5765bc41";
const VER1_AMOUNT: u64 = 10_000;
const SWAP_TRIAL: u64 = 6_000;


fn get_account(builder: &InMemoryWasmTestBuilder, account: PublicKey) -> Account {
    match builder
        .query(None, Key::Account(account), &[])
        .expect("should query system account")
    {
        StoredValue::Account(res_account) => res_account,
        _ => panic!("should get an account"),
    }
}

fn get_contract(builder: &InMemoryWasmTestBuilder, contract_uref: URef) -> Contract {
    builder
        .get_contract(contract_uref.remove_access_rights())
        .expect("should have contract")
}

fn get_swap_hash(builder: &InMemoryWasmTestBuilder) -> [u8; 32] {
    // query client_api_proxy_hash from SYSTEM_ACCOUNT
    let admin_account = get_account(builder, ADMIN_PUBKEY);

    admin_account
        .named_keys()
        .get("swap_proxy")
        .expect("should get swap key")
        .into_hash()
        .expect("should be hash")
}

fn get_swap_stored_hash(builder: &InMemoryWasmTestBuilder) -> Key {
    // query client_api_proxy_hash from SYSTEM_ACCOUNT
    let admin_account = get_account(builder, ADMIN_PUBKEY);

    *admin_account
        .named_keys()
        .get("swap_hash")
        .expect("should get swap key")
}

#[ignore]
#[test]
fn should_run_insert_update_info_and_swap_step() {
    // Genesis setting
    let accounts = vec![
        GenesisAccount::new(
            ADMIN_PUBKEY,
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
        GenesisAccount::new(
            ACCOUNT_1_PUBKEY,
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
    ];

    let state_infos = vec![
        format_args!(
            "d_{}_{}_{}",
            base16::encode_lower(&ADMIN_PUBKEY.as_bytes()),
            base16::encode_lower(&ADMIN_PUBKEY.as_bytes()),
            GENESIS_VALIDATOR_STAKE.to_string()
        )
        .to_string(),
        format_args!(
            "d_{}_{}_{}",
            base16::encode_lower(&ACCOUNT_1_PUBKEY.as_bytes()),
            base16::encode_lower(&ACCOUNT_1_PUBKEY.as_bytes()),
            GENESIS_VALIDATOR_STAKE.to_string()
        )
        .to_string(),
    ];

    let mut builder = InMemoryWasmTestBuilder::default();
    let result = builder
        .run_genesis(&utils::create_genesis_config(accounts, state_infos))
        .finish();
    builder.commit();


    // Swap install pahse
    println!("1. Swap install");
    let swap_install_request = ExecuteRequestBuilder::standard(
        ADMIN_PUBKEY,
        CONTRACT_POS_VOTE,
        (),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(swap_install_request)
        .expect_success()
        .commit()
        .finish();

    let swap_contract_hash = get_swap_hash(&builder);
    let contract_ref = get_swap_stored_hash(&builder);

    // Swap install pahse
    println!("1-1. Input swap hash");
    let set_swap_hash = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("set_swap_hash", contract_ref),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(set_swap_hash)
        .expect_success()
        .commit()
        .finish();

    // Input existing information
    println!("2. Ver1 Token info insert");
    let ver1_token_info_insert_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("insert_snapshot_record", VER1_ADDRESS, ACCOUNT_1_PUBKEY, U512::from(VER1_AMOUNT)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(ver1_token_info_insert_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
            builder.query(Some(builder.get_post_state_hash()), contract_ref, &[VER1_ADDRESS])
                .expect("cannot derive stored value")
        )
        .expect("should have CLValue")
        .into_t()
        .expect("should convert successfully");
        
    assert_eq!(value.is_empty(), false);

    // Update KYC level
    println!("3. Update KYC level");
    let update_kyc_level_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("update_kyc_level", VER1_ADDRESS, U512::from(1u64)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(update_kyc_level_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
            builder.query(Some(builder.get_post_state_hash()), contract_ref, &[VER1_ADDRESS])
                .expect("cannot derive stored value")
        )
        .expect("should have CLValue")
        .into_t()
        .expect("should convert successfully");
        
    assert_eq!(value.get("kyc_level").unwrap(), "1");

    // Update swapable token sent status
    println!("4. Update swapable token sent status");
    let update_swapable_token_sent_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("update_status_swapable_token_sent", VER1_ADDRESS, U512::from(1u64)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(update_swapable_token_sent_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
            builder.query(Some(builder.get_post_state_hash()), contract_ref, &[VER1_ADDRESS])
                .expect("cannot derive stored value")
        )
        .expect("should have CLValue")
        .into_t()
        .expect("should convert successfully");
        
    assert_eq!(value.get("is_sent_token_for_swap").unwrap(), "1");

    // Update KYC step
    println!("5. Update KYC step");
    let update_kyc_step_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("update_kyc_step", VER1_ADDRESS, U512::from(1u64)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(update_kyc_step_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
            builder.query(Some(builder.get_post_state_hash()), contract_ref, &[VER1_ADDRESS])
                .expect("cannot derive stored value")
        )
        .expect("should have CLValue")
        .into_t()
        .expect("should convert successfully");
        
    assert_eq!(value.get("kyc_step").unwrap(), "1");

    // Update KYC step
    let contract_ref = get_swap_stored_hash(&builder);
    println!("6. Get token");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ACCOUNT_1_PUBKEY,
        swap_contract_hash,
        (
            "get_token",
            contract_ref,
            vec![VER1_ADDRESS],
            vec![VER1_PUBKEY],
            vec![VER1_MESSAGE_HASHED],
            vec![VER1_SIGNATURE],
            vec![U512::from(SWAP_TRIAL)]
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(get_token_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
            builder.query(Some(builder.get_post_state_hash()), contract_ref, &[VER1_ADDRESS])
                .expect("cannot derive stored value")
        )
        .expect("should have CLValue")
        .into_t()
        .expect("should convert successfully");
        
    assert_eq!(value.get("swapped_amount").unwrap(), &SWAP_TRIAL.to_string());
}
