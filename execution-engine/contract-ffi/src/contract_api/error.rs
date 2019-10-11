use core::fmt::{self, Debug, Formatter};
use core::{u16, u8};

use crate::bytesrepr;
use crate::system_contracts::{mint, pos};

/// All `Error` variants defined in this library other than `Error::User` will convert to a `u32`
/// value less than or equal to `RESERVED_ERROR_MAX`.
const RESERVED_ERROR_MAX: u32 = u16::MAX as u32; // 0..=65535

/// Proof of Stake errors (defined in "contracts/system/pos/src/error.rs") will have this value
/// added to them when being converted to a `u32`.
const POS_ERROR_OFFSET: u32 = RESERVED_ERROR_MAX - u8::MAX as u32; // 65280..=65535

/// Mint errors (defined in "contracts/system/mint/src/error.rs") will have this value
/// added to them when being converted to a `u32`.
const MINT_ERROR_OFFSET: u32 = (POS_ERROR_OFFSET - 1) - u8::MAX as u32; // 65024..=65279

/// Variants to be passed to `runtime::revert()`.
///
/// Variants other than `Error::User` will represent a `u32` in the range `(0, u16::MAX]`, while
/// `Error::User` will represent a `u32` in the range `(u16::MAX, 2 * u16::MAX + 1]`.
///
/// Users can specify a C-style enum and implement `From` to ease usage of
/// `runtime::revert()`, e.g.
/// ```
/// use casperlabs_contract_ffi::contract_api::Error;
///
/// #[repr(u16)]
/// enum FailureCode {
///     Zero = 0,  // 65,536 as an Error::User
///     One,       // 65,537 as an Error::User
///     Two        // 65,538 as an Error::User
/// }
///
/// impl From<FailureCode> for Error {
///     fn from(code: FailureCode) -> Self {
///         Error::User(code as u16)
///     }
/// }
///
/// assert_eq!(Error::User(1), FailureCode::One.into());
/// assert_eq!(65_536, u32::from(Error::from(FailureCode::Zero)));
/// assert_eq!(65_538, u32::from(Error::from(FailureCode::Two)));
/// ```
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Optional data was unexpectedly `None`.
    None,
    /// Specified argument not provided.
    MissingArgument,
    /// Argument not of correct type.
    InvalidArgument,
    /// Failed to deserialize a value.
    Deserialize,
    /// `read` returned an error.
    Read,
    /// The given key returned a `None` value.
    ValueNotFound,
    /// Failed to find a specified contract.
    ContractNotFound,
    /// A call to `get_key()` returned a failure.
    GetKey,
    /// The `Key` variant was not as expected.
    UnexpectedKeyVariant,
    /// The `Value` variant was not as expected.
    UnexpectedValueVariant,
    /// The `ContractRef` variant was not as expected.
    UnexpectedContractRefVariant,
    /// Invalid purse name given.
    InvalidPurseName,
    /// Invalid purse retrieved.
    InvalidPurse,
    /// Failed to upgrade contract at URef.
    UpgradeContractAtURef,
    /// Failed to transfer motes.
    Transfer,
    /// No access rights.
    NoAccessRights,
    /// A given type could be derived from a `Value`.
    ValueConversion,
    /// Early end of stream when deserializing.
    EarlyEndOfStream,
    /// Formatting error.
    FormattingError,
    /// Leftover bytes.
    LeftOverBytes,
    /// Out of memory error.
    OutOfMemoryError,
    /// Error specific to Mint contract.
    Mint(u8),
    /// Error specific to Proof of Stake contract.
    ProofOfStake(u8),
    /// User-specified value.  The internal `u16` value is added to `u16::MAX as u32 + 1` when an
    /// `Error::User` is converted to a `u32`.
    User(u16),
}

impl From<bytesrepr::Error> for Error {
    fn from(error: bytesrepr::Error) -> Self {
        match error {
            bytesrepr::Error::EarlyEndOfStream => Error::EarlyEndOfStream,
            bytesrepr::Error::FormattingError => Error::FormattingError,
            bytesrepr::Error::LeftOverBytes => Error::LeftOverBytes,
            bytesrepr::Error::OutOfMemoryError => Error::OutOfMemoryError,
        }
    }
}

impl From<mint::Error> for Error {
    fn from(error: mint::Error) -> Self {
        Error::Mint(error as u8)
    }
}

impl From<pos::Error> for Error {
    fn from(error: pos::Error) -> Self {
        Error::ProofOfStake(error as u8)
    }
}

impl From<Error> for u32 {
    fn from(error: Error) -> Self {
        match error {
            Error::None => 1,
            Error::MissingArgument => 2,
            Error::InvalidArgument => 3,
            Error::Deserialize => 4,
            Error::Read => 5,
            Error::ValueNotFound => 6,
            Error::ContractNotFound => 7,
            Error::GetKey => 8,
            Error::UnexpectedKeyVariant => 9,
            Error::UnexpectedValueVariant => 10,
            Error::UnexpectedContractRefVariant => 11,
            Error::InvalidPurseName => 12,
            Error::InvalidPurse => 13,
            Error::UpgradeContractAtURef => 14,
            Error::Transfer => 15,
            Error::NoAccessRights => 16,
            Error::ValueConversion => 17,
            Error::EarlyEndOfStream => 18,
            Error::FormattingError => 19,
            Error::LeftOverBytes => 20,
            Error::OutOfMemoryError => 21,
            Error::Mint(value) => MINT_ERROR_OFFSET + u32::from(value),
            Error::ProofOfStake(value) => POS_ERROR_OFFSET + u32::from(value),
            Error::User(value) => RESERVED_ERROR_MAX + 1 + u32::from(value),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::None => write!(f, "Error::None")?,
            Error::MissingArgument => write!(f, "Error::MissingArgument")?,
            Error::InvalidArgument => write!(f, "Error::InvalidArgument")?,
            Error::Deserialize => write!(f, "Error::Deserialize")?,
            Error::Read => write!(f, "Error::Read")?,
            Error::ValueNotFound => write!(f, "Error::ValueNotFound")?,
            Error::ContractNotFound => write!(f, "Error::ContractNotFound")?,
            Error::GetKey => write!(f, "Error::GetURef")?,
            Error::UnexpectedKeyVariant => write!(f, "Error::UnexpectedKeyVariant")?,
            Error::UnexpectedValueVariant => write!(f, "Error::UnexpectedValueVariant")?,
            Error::UnexpectedContractRefVariant => {
                write!(f, "Error::UnexpectedContractRefVariant")?
            }
            Error::InvalidPurseName => write!(f, "Error::InvalidPurseName")?,
            Error::InvalidPurse => write!(f, "Error::InvalidPurse")?,
            Error::UpgradeContractAtURef => write!(f, "Error::UpgradeContractAtURef")?,
            Error::Transfer => write!(f, "Error::Transfer")?,
            Error::NoAccessRights => write!(f, "Error::NoAccessRights")?,
            Error::ValueConversion => write!(f, "Error::ValueConversion")?,
            Error::EarlyEndOfStream => write!(f, "Error::EarlyEndOfStream")?,
            Error::FormattingError => write!(f, "Error::FormattingError")?,
            Error::LeftOverBytes => write!(f, "Error::LeftOverBytes")?,
            Error::OutOfMemoryError => write!(f, "Error::OutOfMemoryError")?,
            Error::Mint(value) => write!(f, "Error::Mint({})", value)?,
            Error::ProofOfStake(value) => write!(f, "Error::ProofOfStake({})", value)?,
            Error::User(value) => write!(f, "Error::User({})", value)?,
        }
        write!(f, " [{}]", u32::from(*self))
    }
}

pub fn i32_from(result: Result<(), Error>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(error) => u32::from(error) as i32,
    }
}

pub fn result_from(value: i32) -> Result<(), Error> {
    match value {
        0 => Ok(()),
        1 => Err(Error::None),
        2 => Err(Error::MissingArgument),
        3 => Err(Error::InvalidArgument),
        4 => Err(Error::Deserialize),
        5 => Err(Error::Read),
        6 => Err(Error::ValueNotFound),
        7 => Err(Error::ContractNotFound),
        8 => Err(Error::GetKey),
        9 => Err(Error::UnexpectedKeyVariant),
        10 => Err(Error::UnexpectedValueVariant),
        11 => Err(Error::UnexpectedContractRefVariant),
        12 => Err(Error::InvalidPurseName),
        13 => Err(Error::InvalidPurse),
        14 => Err(Error::UpgradeContractAtURef),
        15 => Err(Error::Transfer),
        16 => Err(Error::NoAccessRights),
        17 => Err(Error::ValueConversion),
        18 => Err(Error::EarlyEndOfStream),
        19 => Err(Error::FormattingError),
        20 => Err(Error::LeftOverBytes),
        21 => Err(Error::OutOfMemoryError),
        _ => {
            if value > RESERVED_ERROR_MAX as i32 && value <= (2 * RESERVED_ERROR_MAX + 1) as i32 {
                Err(Error::User(value as u16))
            } else if value >= MINT_ERROR_OFFSET as i32 && value < POS_ERROR_OFFSET as i32 {
                Err(Error::Mint(value as u8))
            } else if value >= POS_ERROR_OFFSET as i32 {
                Err(Error::ProofOfStake(value as u8))
            } else {
                // TODO: this is not unreachable
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::{u16, u8};

    fn round_trip(result: Result<(), Error>) {
        let code = i32_from(result);
        assert_eq!(result, result_from(code));
    }

    #[test]
    fn error() {
        assert_eq!(65_024_u32, Error::Mint(0).into()); // MINT_ERROR_OFFSET == 65,024
        assert_eq!(65_279_u32, Error::Mint(u8::MAX).into());
        assert_eq!(65_280_u32, Error::ProofOfStake(0).into()); // POS_ERROR_OFFSET == 65,280
        assert_eq!(65_535_u32, Error::ProofOfStake(u8::MAX).into());
        assert_eq!(65_536_u32, Error::User(0).into()); // u16::MAX + 1
        assert_eq!(131_071_u32, Error::User(u16::MAX).into()); // 2 * u16::MAX + 1

        assert_eq!("Error::GetURef [8]", &format!("{:?}", Error::GetKey));
        assert_eq!("Error::Mint(0) [65024]", &format!("{:?}", Error::Mint(0)));
        assert_eq!(
            "Error::Mint(255) [65279]",
            &format!("{:?}", Error::Mint(u8::MAX))
        );
        assert_eq!(
            "Error::ProofOfStake(0) [65280]",
            &format!("{:?}", Error::ProofOfStake(0))
        );
        assert_eq!(
            "Error::ProofOfStake(255) [65535]",
            &format!("{:?}", Error::ProofOfStake(u8::MAX))
        );
        assert_eq!("Error::User(0) [65536]", &format!("{:?}", Error::User(0)));
        assert_eq!(
            "Error::User(65535) [131071]",
            &format!("{:?}", Error::User(u16::MAX))
        );

        round_trip(Ok(()));
        round_trip(Err(Error::None));
        round_trip(Err(Error::MissingArgument));
        round_trip(Err(Error::InvalidArgument));
        round_trip(Err(Error::Deserialize));
        round_trip(Err(Error::Read));
        round_trip(Err(Error::ValueNotFound));
        round_trip(Err(Error::ContractNotFound));
        round_trip(Err(Error::GetKey));
        round_trip(Err(Error::UnexpectedKeyVariant));
        round_trip(Err(Error::UnexpectedValueVariant));
        round_trip(Err(Error::UnexpectedContractRefVariant));
        round_trip(Err(Error::InvalidPurseName));
        round_trip(Err(Error::InvalidPurse));
        round_trip(Err(Error::UpgradeContractAtURef));
        round_trip(Err(Error::Transfer));
        round_trip(Err(Error::NoAccessRights));
        round_trip(Err(Error::ValueConversion));
        round_trip(Err(Error::EarlyEndOfStream));
        round_trip(Err(Error::FormattingError));
        round_trip(Err(Error::LeftOverBytes));
        round_trip(Err(Error::OutOfMemoryError));
        round_trip(Err(Error::ProofOfStake(0)));
        round_trip(Err(Error::ProofOfStake(u8::MAX)));
        round_trip(Err(Error::User(0)));
        round_trip(Err(Error::User(u16::MAX)));
    }
}
