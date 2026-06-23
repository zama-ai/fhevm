#[allow(clippy::module_inception)]
pub mod config;

// Re-export configuration types for easier access
pub use config::{
    BlockFetcherStrategy, BlockStartConfig, BlockchainConfig, BrokerConfig, BrokerType,
    CatchupConfig, ConfigError, DatabaseConfig, LogConfig, LogFormat, PoolConfig, PublishConfig,
    Settings, StrategyConfig,
};
