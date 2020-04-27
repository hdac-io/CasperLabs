use std::convert::{TryFrom, TryInto};

use engine_core::engine_state::genesis::Delegator;
use engine_shared::motes::Motes;

use crate::engine_server::{ipc::ChainSpec_Delegator, mappings::MappingError};

impl From<Delegator> for ChainSpec_Delegator {
    fn from(delegator: Delegator) -> Self {
        let mut pb_delegator = ChainSpec_Delegator::new();

        pb_delegator.set_delegate_key(delegator.delegator_key().into());
        pb_delegator.set_amount(delegator.amount().value().into());

        pb_delegator
    }
}

impl TryFrom<ChainSpec_Delegator> for Delegator {
    type Error = MappingError;

    fn try_from(mut pb_delegator: ChainSpec_Delegator) -> Result<Self, Self::Error> {
        // TODO: our TryFromSliceForPublicKeyError should convey length info
        let delegate_key = pb_delegator.take_delegate_key().try_into()?;
        let amount = pb_delegator.take_amount().try_into().map(Motes::new)?;
        Ok(Delegator::new(delegate_key, amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_server::mappings::test_utils;

    #[test]
    fn round_trip() {
        let delegator = rand::random();
        test_utils::protobuf_round_trip::<Delegator, ChainSpec_Delegator>(delegator);
    }
}
