extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};

use num_traits::identities::Zero;

use engine_core::engine_state::genesis::GenesisAccount;
use engine_shared::{motes::Motes, stored_value::StoredValue, account::Account};
use engine_test_support::{
    internal::{utils, ExecuteRequestBuilder, InMemoryWasmTestBuilder},
    DEFAULT_ACCOUNT_INITIAL_BALANCE,
};
use types::{account::PublicKey, bytesrepr::ToBytes, U512, Key};

const CONTRACT_POS_VOTE: &str = "swap_install.wasm";
const BIGSUN_TO_HDAC: u64 = 1_000_000_000_000_000_000_u64;

const ADMIN_PUBKEY: PublicKey = PublicKey::ed25519_from([1u8; 32]);
const ACCOUNT_1_PUBKEY: PublicKey = PublicKey::ed25519_from([2u8; 32]);

const GENESIS_VALIDATOR_STAKE: u64 = 5u64 * BIGSUN_TO_HDAC;
const ACCOUNT_1_DELEGATE_AMOUNT: u64 = BIGSUN_TO_HDAC;
const SYSTEM_ACC_SUPPORT: u64 = 5u64 * BIGSUN_TO_HDAC;

const VER1_ADDRESS: &str = "HR1VnNw3qXRN9UP5Zww8h93HYqcSot4MEd";
// TODO: the pubkey & signature are not related with the address
const VER1_PUBKEY: &str = "0223bec70d670d29a30d9bcee197910e37cf2a10f0dc3c5ac44d865aec0d7052fb";
// const VER1_MESSAGE: &str = "020000000001011333183ddf384da83ed49296136c70d206ad2b19331bf25d390e69b2221\
//                             65e370000000017160014b93f973eb2bf0b614bddc0f47286788c98c535b4feffffff0200\
//                             e1f5050000000017a914a860f76561c85551594c18eecceffaee8c4822d787f0c1a435000\
//                             0000017a914d8b6fcc85a383261df05423ddf068a8987bf028787024730440220434caf5b\
//                             b442cb6a251e8bce0ec493f9a1a9c4423bcfc029e542b0e8a89d1b3f022011090d4e98f79\
//                             c62b188245a4aa4eb77e912bfd57e0a9b9a1c5e65f2b39f3ab401210223bec70d670d29a3\
//                             0d9bcee197910e37cf2a10f0dc3c5ac44d865aec0d7052fb8c000000";
const VER1_MESSAGE: &str = "02000000011333183ddf384da83ed49296136c70d206ad2b19331bf25d390e69b222165e3\
                            70000000000feffffff0200e1f5050000000017a914a860f76561c85551594c18eecceffa\
                            ee8c4822d787F0C1A4350000000017a914d8b6fcc85a383261df05423ddf068a8987bf028\
                            7878c000000";
const VER1_SIGNATURE: &str = "30440220434caf5bb442cb6a251e8bce0ec493f9a1a9c4423bcfc029e542b0e8a89d1b3\
                              f022011090d4e98f79c62b188245a4aa4eb77e912bfd57e0a9b9a1c5e65f2b39f3ab401";
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

fn get_swap_hash(builder: &InMemoryWasmTestBuilder) -> [u8; 32] {
    // query client_api_proxy_hash from SYSTEM_ACCOUNT
    let admin_account = get_account(builder, ADMIN_PUBKEY);

    admin_account
        .named_keys()
        .get("swap")
        .expect("should get swap key")
        .into_hash()
        .expect("should be hash")
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

    let account = get_account(&builder, ADMIN_PUBKEY);
    assert!(account.named_keys().contains_key(VER1_ADDRESS));

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

    let account = get_account(&builder, ADMIN_PUBKEY);
    let address_uref = account
        .named_keys()
        .get(VER1_ADDRESS)
        .expect("should have ver1 address as a key");
    let stored_value = builder
        .query(None, *address_uref, &[])
        .expect("should have stored value");
    let cl_value = stored_value.as_cl_value().expect("should be CLValue");
    let value: BTreeMap<String, String> = cl_value
        .clone()
        .into_t()
        .expect("should cast CLValue to BTreeMap");
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

    let account = get_account(&builder, ADMIN_PUBKEY);
    let address_uref = account
        .named_keys()
        .get(VER1_ADDRESS)
        .expect("should have ver1 address as a key");
    let stored_value = builder
        .query(None, *address_uref, &[])
        .expect("should have stored value");
    let cl_value = stored_value.as_cl_value().expect("should be CLValue");
    let value: BTreeMap<String, String> = cl_value
        .clone()
        .into_t()
        .expect("should cast CLValue to BTreeMap");
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

    let account = get_account(&builder, ADMIN_PUBKEY);
    let address_uref = account
        .named_keys()
        .get(VER1_ADDRESS)
        .expect("should have ver1 address as a key");
    let stored_value = builder
        .query(None, *address_uref, &[])
        .expect("should have stored value");
    let cl_value = stored_value.as_cl_value().expect("should be CLValue");
    let value: BTreeMap<String, String> = cl_value
        .clone()
        .into_t()
        .expect("should cast CLValue to BTreeMap");
    assert_eq!(value.get("kyc_step").unwrap(), "1");

    // Update KYC step
    println!("6. Get token");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("get_token", VER1_ADDRESS, VER1_PUBKEY, VER1_MESSAGE, VER1_SIGNATURE, U512::from(SWAP_TRIAL)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(get_token_request)
        .expect_success()
        .commit()
        .finish();

    let account = get_account(&builder, ADMIN_PUBKEY);
    let address_uref = account
        .named_keys()
        .get(VER1_ADDRESS)
        .expect("should have ver1 address as a key");
    let stored_value = builder
        .query(None, *address_uref, &[])
        .expect("should have stored value");
    let cl_value = stored_value.as_cl_value().expect("should be CLValue");
    let value: BTreeMap<String, String> = cl_value
        .clone()
        .into_t()
        .expect("should cast CLValue to BTreeMap");
    assert_eq!(value.get("swapped_amount").unwrap(), &SWAP_TRIAL.to_string());
}
