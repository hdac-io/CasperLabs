use engine_core::engine_state::genesis::{POS_PAYMENT_PURSE, POS_REWARDS_PURSE};
use engine_test_support::{
    internal::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_GENESIS_CONFIG,
        DEFAULT_PAYMENT,
    },
    DEFAULT_ACCOUNT_ADDR,
};
use types::{account::PublicKey, Key, URef, U512};

const CONTRACT_FINALIZE_PAYMENT: &str = "pos_finalize_payment.wasm";
const CONTRACT_TRANSFER_PURSE_TO_ACCOUNT: &str = "transfer_purse_to_account.wasm";
const FINALIZE_PAYMENT: &str = "pos_finalize_payment.wasm";

const SYSTEM_ADDR: PublicKey = PublicKey::ed25519_from([0u8; 32]);
const ACCOUNT_ADDR: PublicKey = PublicKey::ed25519_from([1u8; 32]);

fn initialize() -> InMemoryWasmTestBuilder {
    let mut builder = InMemoryWasmTestBuilder::default();

    let exec_request_1 = ExecuteRequestBuilder::standard(
        DEFAULT_ACCOUNT_ADDR,
        CONTRACT_TRANSFER_PURSE_TO_ACCOUNT,
        (SYSTEM_ADDR, *DEFAULT_PAYMENT),
    )
    .build();

    let exec_request_2 = ExecuteRequestBuilder::standard(
        DEFAULT_ACCOUNT_ADDR,
        CONTRACT_TRANSFER_PURSE_TO_ACCOUNT,
        (ACCOUNT_ADDR, *DEFAULT_PAYMENT),
    )
    .build();

    builder
        .run_genesis(&DEFAULT_GENESIS_CONFIG)
        .exec(exec_request_1)
        .expect_success()
        .commit()
        .exec(exec_request_2)
        .expect_success()
        .commit();

    builder
}

#[ignore]
#[test]
fn finalize_payment_should_not_be_run_by_non_system_accounts() {
    let mut builder = initialize();
    let payment_amount = U512::from(300);
    let spent_amount = U512::from(75);
    let refund_purse: Option<URef> = None;
    let args = (
        payment_amount,
        refund_purse,
        Some(spent_amount),
        Some(ACCOUNT_ADDR),
    );

    let exec_request_1 =
        ExecuteRequestBuilder::standard(DEFAULT_ACCOUNT_ADDR, CONTRACT_FINALIZE_PAYMENT, args)
            .build();
    let exec_request_2 =
        ExecuteRequestBuilder::standard(ACCOUNT_ADDR, CONTRACT_FINALIZE_PAYMENT, args).build();

    assert!(builder.exec(exec_request_1).is_error());

    assert!(builder.exec(exec_request_2).is_error());
}

#[ignore]
#[test]
fn finalize_payment_should_reward_to_specified_purse() {
    let mut builder = InMemoryWasmTestBuilder::default();
    let payment_amount = *DEFAULT_PAYMENT;
    let refund_purse_flag: u8 = 1;
    // Don't need to run finalize_payment manually, it happens during
    // the deploy because payment code is enabled.
    let args: (U512, u8, Option<U512>, Option<PublicKey>) =
        (payment_amount, refund_purse_flag, None, None);

    builder.run_genesis(&DEFAULT_GENESIS_CONFIG);

    let payment_pre_balance = get_pos_payment_purse_balance(&builder);
    let rewards_pre_balance = get_pos_rewards_purse_balance(&builder);

    assert!(
        payment_pre_balance.is_zero(),
        "payment purse should start with zero balance"
    );

    let exec_request = {
        let genesis_public_key = DEFAULT_ACCOUNT_ADDR;

        let deploy = DeployItemBuilder::new()
            .with_address(DEFAULT_ACCOUNT_ADDR)
            .with_deploy_hash([1; 32])
            .with_session_code("do_nothing.wasm", ())
            .with_payment_code(FINALIZE_PAYMENT, args)
            .with_authorization_keys(&[genesis_public_key])
            .build();

        ExecuteRequestBuilder::new().push_deploy(deploy).build()
    };
    builder.exec(exec_request).expect_success().commit();

    let payment_post_balance = get_pos_payment_purse_balance(&builder);
    let rewards_post_balance = get_pos_rewards_purse_balance(&builder);
    let expected_amount = rewards_pre_balance + *DEFAULT_PAYMENT;
    assert_eq!(
        expected_amount, rewards_post_balance,
        "validators should get paid; expected: {}, actual: {}",
        expected_amount, rewards_post_balance
    );

    assert!(
        payment_post_balance.is_zero(),
        "payment purse should ends with zero balance"
    );
}

// ------------- utility functions -------------------- //

fn get_pos_payment_purse_balance(builder: &InMemoryWasmTestBuilder) -> U512 {
    let purse_id = get_pos_purse_id_by_name(builder, POS_PAYMENT_PURSE)
        .expect("should find PoS payment purse");
    builder.get_purse_balance(purse_id)
}

fn get_pos_rewards_purse_balance(builder: &InMemoryWasmTestBuilder) -> U512 {
    let purse_id = get_pos_purse_id_by_name(builder, POS_REWARDS_PURSE)
        .expect("should find PoS rewards purse");
    builder.get_purse_balance(purse_id)
}

fn get_pos_purse_id_by_name(builder: &InMemoryWasmTestBuilder, purse_name: &str) -> Option<URef> {
    let pos_contract = builder.get_pos_contract();

    pos_contract
        .named_keys()
        .get(purse_name)
        .and_then(Key::as_uref)
        .cloned()
}
