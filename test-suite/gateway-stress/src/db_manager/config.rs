use serde::{Deserialize, Serialize};

/// Configuration for a single database connector
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConnectorConfig {
    /// Database URL (PostgreSQL)
    pub database_url: String,
    
    /// Optional name for this connector (for identification in logs)
    pub name: Option<String>,
    
    /// Connection pool settings
    pub pool_size: Option<u32>,
    
    /// Connection timeout in seconds
    pub connection_timeout: Option<u64>,
    
    /// Enable SSL/TLS for database connection
    pub ssl_mode: Option<String>,
}

impl DbConnectorConfig {
    /// Create a new connector configuration
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
            name: None,
            pool_size: Some(10),
            connection_timeout: Some(30),
            ssl_mode: None,
        }
    }
}
