use alloc::vec::Vec;
use core::u32;

use crate::{
    bytesrepr::{self, FromBytes, ToBytes, U32_SERIALIZED_LENGTH},
    CLType, CLTyped,
};

/// Error while converting a [`CLValue`] into a given type.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CLTypeMismatch {
    /// The [`CLType`] into which the `CLValue` was being converted.
    pub expected: CLType,
    /// The actual underlying [`CLType`] of this `CLValue`, i.e. the type from which it was
    /// constructed.
    pub found: CLType,
}

/// Error relating to [`CLValue`] operations.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CLValueError {
    /// An error while serializing or deserializing the underlying data.
    Serialization(bytesrepr::Error),
    /// A type mismatch while trying to convert a [`CLValue`] into a given type.
    Type(CLTypeMismatch),
}

/// A CasperLabs value, i.e. a value which can be stored and manipulated by smart contracts.
///
/// It holds the underlying data as a type-erased, serialized `Vec<u8>` and also holds the
/// [`CLType`] of the underlying data as a separate member.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CLValue {
    cl_type: CLType,
    bytes: Vec<u8>,
}

impl CLValue {
    /// Constructs a `CLValue` from `t`.
    pub fn from_t<T: CLTyped + ToBytes>(t: T) -> Result<CLValue, CLValueError> {
        let bytes = t.into_bytes().map_err(CLValueError::Serialization)?;

        Ok(CLValue {
            cl_type: T::cl_type(),
            bytes,
        })
    }

    /// Consumes and converts `self` back into its underlying type.
    pub fn into_t<T: CLTyped + FromBytes>(self) -> Result<T, CLValueError> {
        let expected = T::cl_type();

        if self.cl_type == expected {
            bytesrepr::deserialize(self.bytes).map_err(CLValueError::Serialization)
        } else {
            Err(CLValueError::Type(CLTypeMismatch {
                expected,
                found: self.cl_type,
            }))
        }
    }

    // This is only required in order to implement `TryFrom<state::CLValue> for CLValue` (i.e. the
    // conversion from the Protobuf `CLValue`) in a separate module to this one.
    #[doc(hidden)]
    pub fn from_components(cl_type: CLType, bytes: Vec<u8>) -> Self {
        Self { cl_type, bytes }
    }

    // This is only required in order to implement `From<CLValue> for state::CLValue` (i.e. the
    // conversion to the Protobuf `CLValue`) in a separate module to this one.
    #[doc(hidden)]
    pub fn destructure(self) -> (CLType, Vec<u8>) {
        (self.cl_type, self.bytes)
    }

    /// The [`CLType`] of the underlying data.
    pub fn cl_type(&self) -> &CLType {
        &self.cl_type
    }

    /// Returns a reference to the serialized form of the underlying value held in this `CLValue`.
    pub fn inner_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Returns the length of the `Vec<u8>` yielded after calling `self.to_bytes()`.
    ///
    /// Note, this method doesn't actually serialize `self`, and hence is relatively cheap.
    pub fn serialized_len(&self) -> usize {
        self.cl_type.serialized_len() + U32_SERIALIZED_LENGTH + self.bytes.len()
    }
}

impl ToBytes for CLValue {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        self.clone().into_bytes()
    }

    fn into_bytes(self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = self.bytes.into_bytes()?;
        self.cl_type.append_bytes(&mut result);
        Ok(result)
    }
}

impl FromBytes for CLValue {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (bytes, remainder) = Vec::<u8>::from_bytes(bytes)?;
        let (cl_type, remainder) = CLType::from_bytes(remainder)?;
        let cl_value = CLValue { cl_type, bytes };
        Ok((cl_value, remainder))
    }
}

impl ToBytes for Vec<CLValue> {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let serialized_len = self.iter().map(CLValue::serialized_len).sum();
        if serialized_len > u32::max_value() as usize - U32_SERIALIZED_LENGTH {
            return Err(bytesrepr::Error::OutOfMemory);
        }

        let mut result = Vec::with_capacity(serialized_len);
        let len = self.len() as u32;
        result.append(&mut len.to_bytes()?);

        for cl_value in self {
            result.append(&mut cl_value.to_bytes()?);
        }

        Ok(result)
    }

    fn into_bytes(self) -> Result<Vec<u8>, bytesrepr::Error> {
        let serialized_len = self.iter().map(CLValue::serialized_len).sum();
        if serialized_len > u32::max_value() as usize - U32_SERIALIZED_LENGTH {
            return Err(bytesrepr::Error::OutOfMemory);
        }

        let mut result = Vec::with_capacity(serialized_len);
        let len = self.len() as u32;
        result.append(&mut len.to_bytes()?);

        for cl_value in self {
            result.append(&mut cl_value.into_bytes()?);
        }

        Ok(result)
    }
}

impl FromBytes for Vec<CLValue> {
    fn from_bytes(mut bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (len, remainder) = u32::from_bytes(bytes)?;
        bytes = remainder;

        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let (cl_value, remainder) = CLValue::from_bytes(bytes)?;
            result.push(cl_value);
            bytes = remainder;
        }
        Ok((result, bytes))
    }
}
