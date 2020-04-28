//! Functions for converting between CasperLabs types and their Protobuf equivalents.

mod ipc;
mod state;
mod transforms;

use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
    string::ToString,
};

use engine_core::{engine_state, DEPLOY_HASH_LENGTH};
use types::account::ED25519_LENGTH;

pub use transforms::TransformMap;

/// Try to convert a `Vec<u8>` to a 32-byte array.
pub(crate) fn vec_to_array(input: Vec<u8>, input_name: &str) -> Result<[u8; 32], ParsingError> {
    input
        .as_slice()
        .try_into()
        .map_err(|_| format!("{} must be 32 bytes.", input_name).into())
}

/// Try to convert a `Vec<u8>` to a 64-byte array.
pub(crate) fn vec_to_array64(input: Vec<u8>, input_name: &str) -> Result<[u8; 64], ParsingError> {
    if input.len() != 64 {
        return Err(format!("{} must be 64 bytes.", input_name).into());
    }
    let mut result = [0; 64];
    result.copy_from_slice(&input);
    Ok(result)
}

#[derive(Debug)]
pub enum MappingError {
    InvalidStateHashLength { expected: usize, actual: usize },
    InvalidPublicKeyLength { expected: usize, actual: usize },
    InvalidDeployHashLength { expected: usize, actual: usize },
    Parsing(ParsingError),
    InvalidStateHash(String),
    MissingPayload,
    TryFromSlice,
}

impl MappingError {
    pub fn invalid_public_key_length(actual: usize) -> Self {
        let expected = ED25519_LENGTH;
        MappingError::InvalidPublicKeyLength { expected, actual }
    }

    pub fn invalid_deploy_hash_length(actual: usize) -> Self {
        let expected = DEPLOY_HASH_LENGTH;
        MappingError::InvalidDeployHashLength { expected, actual }
    }
}

impl From<ParsingError> for MappingError {
    fn from(error: ParsingError) -> Self {
        MappingError::Parsing(error)
    }
}

// This is whackadoodle, we know
impl From<MappingError> for engine_state::Error {
    fn from(error: MappingError) -> Self {
        match error {
            MappingError::InvalidStateHashLength { expected, actual } => {
                engine_state::Error::InvalidHashLength { expected, actual }
            }
            _ => engine_state::Error::Deploy,
        }
    }
}

impl Display for MappingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            MappingError::InvalidStateHashLength { expected, actual } => write!(
                f,
                "Invalid hash length: expected {}, actual {}",
                expected, actual
            ),
            MappingError::InvalidPublicKeyLength { expected, actual } => write!(
                f,
                "Invalid public key length: expected {}, actual {}",
                expected, actual
            ),
            MappingError::InvalidDeployHashLength { expected, actual } => write!(
                f,
                "Invalid deploy hash length: expected {}, actual {}",
                expected, actual
            ),
            MappingError::Parsing(ParsingError(message)) => write!(f, "Parsing error: {}", message),
            MappingError::InvalidStateHash(message) => write!(f, "Invalid hash: {}", message),
            MappingError::MissingPayload => write!(f, "Missing payload"),
            MappingError::TryFromSlice => write!(f, "Unable to convert from slice"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParsingError(pub String);

impl<T: ToString> From<T> for ParsingError {
    fn from(error: T) -> Self {
        ParsingError(error.to_string())
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::{any, convert::TryFrom, fmt::Debug};

    /// Checks that domain object `original` can be converted into a corresponding protobuf object
    /// and back, and that the conversions yield an equal object to `original`.
    pub fn protobuf_round_trip<T, U>(original: T)
    where
        T: Clone + PartialEq + Debug + TryFrom<U>,
        <T as TryFrom<U>>::Error: Debug,
        U: From<T>,
    {
        let pb_object = U::from(original.clone());
        let parsed = T::try_from(pb_object).unwrap_or_else(|_| {
            panic!(
                "Expected transforming {} into {} to succeed.",
                any::type_name::<U>(),
                any::type_name::<T>()
            )
        });
        assert_eq!(original, parsed);
    }
}

#[cfg(test)]
mod tests {
    use super::vec_to_array;

    #[test]
    fn vec_to_array_test() {
        assert_eq!([1; 32], vec_to_array(vec![1; 32], "").unwrap());
        assert!(vec_to_array(vec![], "").is_err());
        assert!(vec_to_array(vec![1; 31], "").is_err());
        assert!(vec_to_array(vec![1; 33], "").is_err());
    }
}
