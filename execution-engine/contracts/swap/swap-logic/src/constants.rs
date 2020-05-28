pub(crate) mod methods {
    pub const METHOD_SET_SWAP_HASH: &str = "set_swap_hash";
    pub const METHOD_INSERT_KYC_ALLOWANCE_CAP: &str = "insert_kyc_allowance_cap";
    pub const METHOD_INSERT_SNAPSHOT_RECORD: &str = "insert_snapshot_record";
    pub const METHOD_INSERT_KYC_DATA: &str = "insert_kyc_data";
    pub const METHOD_UPDATE_KYC_LEVEL: &str = "update_kyc_level";
    pub const METHOD_UPDATE_STATUS_SWAPABLE_TOKEN_SENT: &str = "update_status_swapable_token_sent";
    pub const METHOD_UPDATE_KYC_STEP: &str = "update_kyc_step";
    pub const METHOD_GET_TOKEN: &str = "get_token";
}

pub(crate) mod keys {
    pub const KEY_PREV_BALANCE_KEY: &str = "prev_balance";
    pub const KEY_IS_SWAPPED: &str = "is_swapped";

    pub const KEY_KYC_LEVEL: &str = "kyc_level";
    pub const KEY_IS_SENT_TOKEN_FOR_SWAP: &str = "is_sent_token_for_swap";
    pub const KEY_KYC_STEP: &str = "kyc_step";
    pub const KEY_SWAPPED_AMOUNT: &str = "swapped_amount";

    pub const KEY_KYC_BORDER_ALLOWANCE_CAP: &str = "kyc_border_allowance_cap";
}

pub(crate) mod users {
    pub const KEY_ADMIN: &str = "admin";
    pub const KEY_SWAP_HASH: &str = "swap_hash";
}
