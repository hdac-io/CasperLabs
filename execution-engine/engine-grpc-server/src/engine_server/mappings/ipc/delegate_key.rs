use std::convert::{TryFrom, TryInto};

use engine_core::engine_state::genesis::DelegateKey;

use crate::engine_server::{ipc::ChainSpec_DelegateKey, mappings::MappingError};

impl From<DelegateKey> for ChainSpec_DelegateKey {
    fn from(delegate_key: DelegateKey) -> Self {
        let mut pb_delegate_key = ChainSpec_DelegateKey::new();

        pb_delegate_key.set_delegator(delegate_key.delegator().to_vec());
        pb_delegate_key.set_validator(delegate_key.validator().to_vec());

        pb_delegate_key
    }
}

impl TryFrom<ChainSpec_DelegateKey> for DelegateKey {
    type Error = MappingError;

    fn try_from(pb_delegate_key: ChainSpec_DelegateKey) -> Result<Self, Self::Error> {
        // TODO: our TryFromSliceForPublicKeyError should convey length info
        let delegator = pb_delegate_key.get_delegator().try_into().map_err(|_| {
            MappingError::invalid_public_key_length(pb_delegate_key.delegator.len())
        })?;
        let validator = pb_delegate_key.get_validator().try_into().map_err(|_| {
            MappingError::invalid_public_key_length(pb_delegate_key.validator.len())
        })?;
        Ok(DelegateKey::new(delegator, validator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_server::mappings::test_utils;

    #[test]
    fn round_trip() {
        let delegate_key = rand::random();
        test_utils::protobuf_round_trip::<DelegateKey, ChainSpec_DelegateKey>(delegate_key);
    }
}
