use types::ApiError;

#[repr(u16)]
pub enum Error {
    /// Not admin
    NotAdmin = 1, // 65537

    /// Swap request exceeded the possible range
    ExceededSwapRange, // 65538

    /// Swap request exceeded the allowance
    ExceededSwapAllowanceByKyc, // 65539

    /// Insufficient number of swap parameters
    InsufficientNumOfSwapParams,

    /// Caller has not registered KYC
    NotRegisteredKYC,

    /// Invalid KYC level value
    InvalidKYCLevelValue,

    /// Invalid signature
    InvalidSignature,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
