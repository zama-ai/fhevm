//! Serves static KeyUrl data by emitting event on initialization.
//! Can be extended later to handle dynamic updates.

use crate::{
    config::settings::KeyUrl as KeyUrlConfig,
    core::event::{
        ApiCategory, ApiVersion, KeyData, KeyUrlData, KeyUrlEventData, RelayerEvent,
        RelayerEventData,
    },
    core::job_id::INTERNAL_EVENT_JOB_ID,
    orchestrator::Orchestrator,
};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Clone)]
pub struct KeyUrlGatewayHandler {
    orchestrator: Arc<Orchestrator>,
    config: KeyUrlConfig,
}

impl KeyUrlGatewayHandler {
    pub fn new(orchestrator: Arc<Orchestrator>, config: KeyUrlConfig) -> Self {
        Self {
            orchestrator,
            config,
        }
    }

    pub async fn initialize(&self) {
        info!("Initializing KeyUrl cache with config data");

        let key_data = self.load_key_data().await;

        // Emit KeyDataUpdated event through orchestrator
        let event = RelayerEvent::new(
            INTERNAL_EVENT_JOB_ID,
            ApiVersion::new(ApiCategory::PRODUCTION, 1),
            RelayerEventData::KeyUrl(KeyUrlEventData::KeyDataUpdated { key_data }),
        );

        if let Err(e) = self.orchestrator.dispatch_event(event).await {
            error!("Failed to emit KeyDataUpdated event: {}", e);
        } else {
            info!("KeyUrl cache initialized successfully - event emitted");
        }
    }

    async fn load_key_data(&self) -> KeyUrlData {
        // Currently load from config, future: can load from database, external API, etc.
        KeyUrlData {
            fhe_public_key: KeyData {
                data_id: self.config.fhe_public_key.data_id.clone(),
                url: self.config.fhe_public_key.url.clone(),
            },
            crs: KeyData {
                data_id: self.config.crs.data_id.clone(),
                url: self.config.crs.url.clone(),
            },
        }
    }
}

// NOTE: This handler can be extended in the future to:
// 1. Listen for KeyUrl events from the gateway chain (e.g., key rotation events)
// 2. Trigger cache updates by emitting KeyUrlEventData::KeyDataUpdated events
// 3. Handle dynamic key material loading from external sources or databases
