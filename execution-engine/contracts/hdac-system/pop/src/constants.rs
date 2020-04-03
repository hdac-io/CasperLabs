pub(crate) mod local_keys {
    pub const DELEGATE_REQUEST_QUEUE: u8 = 1;
    pub const UNDELEGATE_REQUEST_QUEUE: u8 = 2;
    pub const REDELEGATE_REQUEST_QUEUE: u8 = 3;
    pub const CLAIM_REQUEST_QUEUE: u8 = 4;
}

pub(crate) mod uref_names {
    pub const POS_BONDING_PURSE: &str = "pos_bonding_purse";
}

pub(crate) mod methods {
    pub const METHOD_BOND: &str = "bond";
    pub const METHOD_UNBOND: &str = "unbond";
    pub const METHOD_STEP: &str = "step";
    pub const METHOD_GET_PAYMENT_PURSE: &str = "get_payment_purse";
    pub const METHOD_SET_REFUND_PURSE: &str = "set_refund_purse";
    pub const METHOD_GET_REFUND_PURSE: &str = "get_refund_purse";
    pub const METHOD_FINALIZE_PAYMENT: &str = "finalize_payment";

    pub const METHOD_DELEGATE: &str = "delegate";
    pub const METHOD_UNDELEGATE: &str = "undelegate";
    pub const METHOD_REDELEGATE: &str = "redelegate";
    pub const METHOD_VOTE: &str = "vote";
    pub const METHOD_UNVOTE: &str = "unvote";
    pub const METHOD_WRITE_GENESIS_TOTAL_SUPPLY: &str = "write_genesis_total_supply";
    pub const METHOD_DISTRIBUTE: &str = "distribute";
    pub const METHOD_CLAIM_COMMISSION: &str = "claim_commission";
    pub const METHOD_CLAIM_REWARD: &str = "claim_reward";
}

pub(crate) mod consts {
    pub const DAYS_OF_YEAR: i64 = 365_i64;
    pub const HOURS_OF_DAY: i64 = 24_i64;
    pub const SECONDS_OF_HOUR: i64 = 3600_i64;
    pub const BLOCK_TIME_IN_SEC: i64 = 2_i64;
    pub const VALIDATOR_COMMISSION_RATE_IN_PERCENTAGE: i64 = 30_i64;
}
