use crate::core::event_processor::{
    KmsClient,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
};
use alloy::providers::Provider;
use connector_utils::types::GatewayEvent;
use sqlx::{Pool, Postgres};
use tracing::error;

#[derive(Clone)]
/// Struct responsible of interacting with the KMS Core to process events coming from the Gateway.
pub struct EventProcessor<P: Provider> {
    /// The GRPC client used to communicate with the KMS Core.
    pub kms_client: KmsClient,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<P>,

    /// The DB connection pool used to reset events `under_process` field on error.
    db_pool: Pool<Postgres>,
}

impl<P: Provider> EventProcessor<P> {
    /// Creates a new `EventProcessor<P>` instance.
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

    /// Processes an incoming `event` coming from the Gateway.
    pub async fn process(&mut self, event: &GatewayEvent) {
        if let Err(e) = self.forward_to_kms(event).await {
            event.mark_as_pending(&self.db_pool).await;
            error!("{e}");
        }
    }

    /// Prepares the GRPC request associated to the received `event` and sends it to the KMS Core.
    async fn forward_to_kms(&mut self, event: &GatewayEvent) -> anyhow::Result<()> {
        let request_id;
        let grpc_request = match event {
            GatewayEvent::PublicDecryption(req) => {
                request_id = req.decryptionId;
                self.decryption_processor
                    .prepare_decryption_request(req.decryptionId, &req.snsCtMaterials, None)
                    .await
            }
            GatewayEvent::UserDecryption(req) => {
                request_id = req.decryptionId;
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &req.snsCtMaterials,
                        Some(UserDecryptionExtraData::new(
                            req.userAddress,
                            req.publicKey.clone(),
                        )),
                    )
                    .await
            }
            _ => unimplemented!(),
        }?;

        self.kms_client.send_request(request_id, grpc_request).await
    }
}
