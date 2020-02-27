mod address_generator;
mod error;
#[macro_use]
mod executor;
#[cfg(test)]
mod tests;

pub use self::{
    address_generator::{AddressGenerator, AddressGeneratorBuilder},
    error::Error,
    executor::Executor,
};

pub const MINT_NAME: &str = "mint";
pub const POS_NAME: &str = "pos";
pub const CLIENT_API_PROXY_NAME: &str = "client_api_proxy";

pub(crate) const FN_STORE_ID_INITIAL: u32 = 0;
