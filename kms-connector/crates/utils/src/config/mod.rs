mod contract;
mod error;
mod raw;
mod wallet;

pub use contract::{ContractConfig, RawContractConfig};
pub use error::{Error, Result};
pub use raw::DeserializeRawConfig;
pub use wallet::{AwsKmsConfig, KmsWallet};

pub fn default_database_pool_size() -> u32 {
    16
}
