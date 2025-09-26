use crate::db_manager::ResponseStats;
use std::collections::{HashMap, HashSet};
use tracing::debug;

/// Tracks decryption responses across multiple connectors
pub struct ResponseTracker {
    /// All request IDs we're tracking
    all_requests: HashSet<String>,
    
    /// Map of request ID to set of connector indexes that have the response
    responses: HashMap<String, HashSet<usize>>,
    
    /// Number of connectors we're tracking
    connector_count: usize,
    
    /// Current statistics
    stats: ResponseStats,
}

impl ResponseTracker {
    /// Create a new response tracker for N connectors
    pub fn new(connector_count: usize) -> Self {
        Self {
            connector_count,
            responses: HashMap::new(),
            all_requests: HashSet::new(),
            stats: ResponseStats {
                total_requests: 0,
                responses_by_connector: vec![0; connector_count],
                fully_synced: 0,
                partially_synced: 0,
                missing: 0,
            },
        }
    }
    
    /// Register requests that were inserted
    pub fn register_requests(&mut self, request_ids: Vec<String>) {
        for id in request_ids {
            self.all_requests.insert(id.clone());
            self.responses.entry(id).or_default();
        }
        self.stats.total_requests = self.all_requests.len();
    }
    
    /// Update responses from a specific connector
    pub fn update_responses(&mut self, connector_index: usize, response_ids: Vec<String>) {
        debug!(
            "Updating responses from connector {}: {} responses",
            connector_index,
            response_ids.len()
        );
        
        for id in response_ids {
            self.responses
                .entry(id)
                .or_default()
                .insert(connector_index);
        }
        
        self.update_stats();
    }
    
    /// Update internal statistics
    fn update_stats(&mut self) {
        let mut fully_synced = 0;
        let mut partially_synced = 0;
        let mut missing = 0;
        let mut responses_by_connector = vec![0; self.connector_count];
        
        for request_id in &self.all_requests {
            if let Some(connectors) = self.responses.get(request_id) {
                let count = connectors.len();
                
                // Count responses per connector
                for &connector_idx in connectors {
                    if connector_idx < self.connector_count {
                        responses_by_connector[connector_idx] += 1;
                    }
                }
                
                if count == self.connector_count {
                    fully_synced += 1;
                } else if count > 0 {
                    partially_synced += 1;
                } else {
                    missing += 1;
                }
            } else {
                missing += 1;
            }
        }
        
        self.stats.fully_synced = fully_synced;
        self.stats.partially_synced = partially_synced;
        self.stats.missing = missing;
        self.stats.responses_by_connector = responses_by_connector;
    }
    
    /// Get current statistics
    pub fn get_stats(&self) -> ResponseStats {
        ResponseStats {
            total_requests: self.stats.total_requests,
            fully_synced: self.stats.fully_synced,
            partially_synced: self.stats.partially_synced,
            missing: self.stats.missing,
            responses_by_connector: self.stats.responses_by_connector.clone(),
        }
    }
}
