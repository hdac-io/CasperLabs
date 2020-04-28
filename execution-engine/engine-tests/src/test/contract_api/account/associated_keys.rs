use lazy_static::lazy_static;

use engine_test_support::{
    internal::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_GENESIS_CONFIG, DEFAULT_PAYMENT,
    },
    DEFAULT_ACCOUNT_ADDR,
};
use types::{
    account::{PublicKey, Weight},
    U512,
};

const CONTRACT_ADD_UPDATE_ASSOCIATED_KEY: &str = "add_update_associated_key.wasm";
const CONTRACT_REMOVE_ASSOCIATED_KEY: &str = "remove_associated_key.wasm";
const CONTRACT_TRANSFER_PURSE_TO_ACCOUNT: &str = "transfer_purse_to_account.wasm";
const ACCOUNT_1_ADDR: PublicKey = PublicKey::ed25519_from([1u8; 32]);

lazy_static! {
    static ref ACCOUNT_1_INITIAL_FUND: U512 = *DEFAULT_PAYMENT * 10;
}

#[ignore]
#[test]
fn should_manage_associated_key() {
    // for a given account, should be able to add a new associated key and update
    // that key
    let mut builder = InMemoryWasmTestBuilder::default();

    let exec_request_1 = ExecuteRequestBuilder::standard(
        DEFAULT_ACCOUNT_ADDR,
        CONTRACT_TRANSFER_PURSE_TO_ACCOUNT,
        (ACCOUNT_1_ADDR, *ACCOUNT_1_INITIAL_FUND),
    )
    .build();
    let exec_request_2 = ExecuteRequestBuilder::standard(
        ACCOUNT_1_ADDR,
        CONTRACT_ADD_UPDATE_ASSOCIATED_KEY,
        (DEFAULT_ACCOUNT_ADDR,),
    )
    .build();

    builder
        .run_genesis(&DEFAULT_GENESIS_CONFIG)
        .exec(exec_request_1)
        .expect_success()
        .commit();

    builder.exec(exec_request_2).expect_success().commit();

    let genesis_key = DEFAULT_ACCOUNT_ADDR;

    let account_1 = builder
        .get_account(ACCOUNT_1_ADDR)
        .expect("should have account");

    let gen_weight = account_1
        .get_associated_key_weight(genesis_key)
        .expect("weight");

    let expected_weight = Weight::new(2);
    assert_eq!(*gen_weight, expected_weight, "unexpected weight");

    let exec_request_3 = ExecuteRequestBuilder::standard(
        ACCOUNT_1_ADDR,
        CONTRACT_REMOVE_ASSOCIATED_KEY,
        (DEFAULT_ACCOUNT_ADDR,),
    )
    .build();

    builder.exec(exec_request_3).expect_success().commit();

    let account_1 = builder
        .get_account(ACCOUNT_1_ADDR)
        .expect("should have account");

    let new_weight = account_1.get_associated_key_weight(genesis_key);

    assert_eq!(new_weight, None, "key should be removed");

    let is_error = builder.is_error();
    assert!(!is_error);
}
