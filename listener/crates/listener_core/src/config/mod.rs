#[allow(clippy::module_inception)]
pub mod config;

// Re-export configuration types for easier access
pub use config::{
    BlockFetcherStrategy, BlockchainConfig, BrokerConfig, BrokerType, ConfigError, DatabaseConfig,
    PoolConfig, PublishConfig, Settings, StrategyConfig,
};
