use crate::core::event_processor::{
    KmsClient,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
};
use alloy::providers::Provider;
use connector_utils::types::{GatewayEvent, KmsGrpcRequest, KmsResponse};
use sqlx::{Pool, Postgres};
use tracing::{info, warn};

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

    async fn process(&mut self, event: &Self::Event) -> anyhow::Result<KmsResponse> {
        info!("Starting to process {event}...");
        match self.inner_process(event).await {
            Ok(response) => Ok(response),
            Err(e) => {
                self.mark_event_as_free(event).await;
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
    async fn prepare_request(&self, event: GatewayEvent) -> anyhow::Result<KmsGrpcRequest> {
        match event {
            GatewayEvent::PublicDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(req.decryptionId, req.snsCtMaterials, None)
                    .await
            }
            GatewayEvent::UserDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
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

    /// Changes the `under_process` field of the event to `false` in database.
    async fn mark_event_as_free(&self, event: &GatewayEvent) {
        let query = match event {
            GatewayEvent::PublicDecryption(e) => sqlx::query!(
                "UPDATE public_decryption_requests SET under_process = FALSE WHERE decryption_id = $1",
                e.decryptionId.as_le_slice()
            ),
            GatewayEvent::UserDecryption(e) => sqlx::query!(
                "UPDATE user_decryption_requests SET under_process = FALSE WHERE decryption_id = $1",
                e.decryptionId.as_le_slice()
            ),
            GatewayEvent::PreprocessKeygen(e) => sqlx::query!(
                "UPDATE preprocess_keygen_requests SET under_process = FALSE WHERE pre_keygen_request_id = $1",
                e.preKeygenRequestId.as_le_slice()
            ),
            GatewayEvent::PreprocessKskgen(e) => sqlx::query!(
                "UPDATE preprocess_kskgen_requests SET under_process = FALSE WHERE pre_kskgen_request_id = $1",
                e.preKskgenRequestId.as_le_slice()
            ),
            GatewayEvent::Keygen(e) => sqlx::query!(
                "UPDATE keygen_requests SET under_process = FALSE WHERE pre_key_id = $1",
                e.preKeyId.as_le_slice()
            ),
            GatewayEvent::Kskgen(e) => sqlx::query!(
                "UPDATE kskgen_requests SET under_process = FALSE WHERE pre_ksk_id = $1",
                e.preKskId.as_le_slice()
            ),
            GatewayEvent::Crsgen(e) => sqlx::query!(
                "UPDATE crsgen_requests SET under_process = FALSE WHERE crsgen_request_id = $1",
                e.crsgenRequestId.as_le_slice()
            ),
        };

        let query_result = match query.execute(&self.db_pool).await {
            Ok(result) => result,
            Err(e) => return warn!("{e}"),
        };

        if query_result.rows_affected() == 1 {
            info!("Successfully restore {event} as free in DB");
        } else {
            warn!(
                "Unexpected query result while restoring {} as free: {:?}",
                event, query_result
            )
        }
    }
}
