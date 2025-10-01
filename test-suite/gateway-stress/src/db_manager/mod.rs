pub mod config;
pub mod connector;
pub mod request_builder;
pub mod response_tracker;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use config::DbConnectorConfig;
pub use connector::DbConnector;
pub use request_builder::RequestBuilder;
pub use response_tracker::ResponseTracker;

/// Manager for handling multiple database connectors
pub struct DbManager {
    connectors: Vec<Arc<DbConnector>>,
    request_builder: RequestBuilder,
    response_tracker: Arc<RwLock<ResponseTracker>>,
    /// Keep track of all inserted request IDs for this test run
    inserted_request_ids: Arc<RwLock<Vec<String>>>,
}

impl DbManager {
    /// Create a new DbManager with N connectors
    pub async fn new(configs: Vec<DbConnectorConfig>) -> Result<Self> {
        let mut connectors = Vec::new();
        
        for (index, config) in configs.iter().enumerate() {
            let connector = DbConnector::new(config.clone(), index).await?;
            connectors.push(Arc::new(connector));
        }
        
        Ok(Self {
            connectors,
            request_builder: RequestBuilder::new(),
            response_tracker: Arc::new(RwLock::new(
                ResponseTracker::new(configs.len())
            )),
            inserted_request_ids: Arc::new(RwLock::new(Vec::new())),
        })
    }
    /// Insert a batch of decryption requests across all connectors
    pub async fn insert_requests(
        &self,
        request_type: DecryptionRequestType,
        count: usize,
    ) -> Result<Vec<String>> {
        let requests = self.request_builder.build_requests(request_type, count)?;
        let mut request_ids = Vec::new();
        
        for request in &requests {
            for connector in &self.connectors {
                connector.insert_request(request.clone()).await?;
            }
            request_ids.push(request.id().to_string());
        }
        
        // Track the inserted requests
        let mut tracker = self.response_tracker.write().await;
        tracker.register_requests(request_ids.clone());
        
        // Also keep track of all IDs for this test run
        let mut all_ids = self.inserted_request_ids.write().await;
        all_ids.extend(request_ids.clone());
        
        Ok(request_ids)
    }
    
    /// Track responses across all connectors
    pub async fn track_responses(&self) -> Result<()> {
        // Get the list of request IDs we're tracking
        let request_ids = self.inserted_request_ids.read().await;
        
        let mut tracker = self.response_tracker.write().await;
        
        for (index, connector) in self.connectors.iter().enumerate() {
            // Only fetch responses for the requests we inserted
            let responses = connector.fetch_responses_for_requests(&request_ids).await?;
            tracker.update_responses(index, responses);
        }
        
        Ok(())
    }
    
    /// Get tracking statistics
    pub async fn get_stats(&self) -> ResponseStats {
        let tracker = self.response_tracker.read().await;
        tracker.get_stats()
    }
    
    /// Check connectivity for all connectors
    pub async fn health_check(&self) -> Result<Vec<bool>> {
        let mut results = Vec::new();
        
        for connector in &self.connectors {
            let healthy = connector.health_check().await.is_ok();
            results.push(healthy);
        }
        
        Ok(results)
    }
    
    /// Clear database tables for all connectors
    pub async fn clear_databases(&self) -> Result<()> {
        for connector in &self.connectors {
            connector.clear_tables().await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DecryptionRequestType {
    Public,
    User,
    Mixed,
}

#[derive(Debug, Clone)]
pub struct ResponseStats {
    pub total_requests: usize,
    pub responses_by_connector: Vec<usize>,
    pub fully_synced: usize,
    pub partially_synced: usize,
    pub missing: usize,
}
