use crate::core::event_processor::{
    KmsClient,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
};
use alloy::providers::Provider;
use connector_utils::types::{
    GatewayEvent, KmsGrpcRequest, KmsResponse, db::GatewayEventTransaction,
};

/// Interface used to process Gateway's events.
pub trait EventProcessor: Send {
    type Event: Send;

    fn process(
        self,
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
}

impl<P: Provider> DbEventProcessor<P> {
    pub fn new(kms_client: KmsClient, decryption_processor: DecryptionProcessor<P>) -> Self {
        Self {
            kms_client,
            decryption_processor,
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
}

impl<P: Provider> EventProcessor for DbEventProcessor<P> {
    type Event = GatewayEventTransaction;

    async fn process(self, event_tx: &Self::Event) -> anyhow::Result<KmsResponse> {
        let request = self.prepare_request(event_tx.event.clone()).await?;
        let grpc_response = self.kms_client.send_request(request).await?;
        KmsResponse::process(grpc_response)
    }
}
