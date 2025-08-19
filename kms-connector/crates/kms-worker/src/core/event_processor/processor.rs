use crate::core::event_processor::{
    KmsClient,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
};
use alloy::providers::Provider;
use connector_utils::types::{GatewayEvent, KmsGrpcRequest, KmsResponse};
use sqlx::{Pool, Postgres};
use tracing::info;

/// Interface used to process Gateway's events.
pub trait EventProcessor: Send {
    type Event: Send;

    fn process(
        &mut self,
        event: &Self::Event,
    ) -> impl Future<Output = anyhow::Result<KmsResponse>> + Send;
}

/// Struct that processes Gateway's events coming from a `Postgres` database.
#[derive(Clone)]
pub struct DbEventProcessor<P: Provider> {
    /// The GRPC client used to communicate with the KMS Core.
    kms_client: KmsClient,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<P>,

    /// The DB connection pool used to reset events `under_process` field on error.
    db_pool: Pool<Postgres>,
}

impl<P: Provider> EventProcessor for DbEventProcessor<P> {
    type Event = GatewayEvent;

    #[tracing::instrument(skip_all)]
    async fn process(&mut self, event: &Self::Event) -> anyhow::Result<KmsResponse> {
        info!("Starting to process {:?}...", event);
        match self.inner_process(event).await {
            Ok(response) => {
                info!("Event successfully processed!");
                Ok(response)
            }
            Err(e) => {
                event.mark_as_pending(&self.db_pool).await;
                Err(e)
            }
        }
    }
}

impl<P: Provider> DbEventProcessor<P> {
    pub fn new(
        kms_client: KmsClient,
        decryption_processor: DecryptionProcessor<P>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            kms_client,
            decryption_processor,
            db_pool,
        }
    }

    /// Prepares the GRPC request associated to the received `event`.
    #[tracing::instrument(skip_all)]
    async fn prepare_request(&self, event: GatewayEvent) -> anyhow::Result<KmsGrpcRequest> {
        match event {
            GatewayEvent::PublicDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
                        req.extraData.into(),
                        None,
                    )
                    .await
            }
            GatewayEvent::UserDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
                        req.extraData.into(),
                        Some(UserDecryptionExtraData::new(req.userAddress, req.publicKey)),
                    )
                    .await
            }
            _ => unimplemented!(),
        }
    }

    /// Core event processing logic function.
    async fn inner_process(&mut self, event: &GatewayEvent) -> anyhow::Result<KmsResponse> {
        let request = self.prepare_request(event.clone()).await?;
        let grpc_response = self.kms_client.send_request(request).await?;
        KmsResponse::process(grpc_response)
    }
}
