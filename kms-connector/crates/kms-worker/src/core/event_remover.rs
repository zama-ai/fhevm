use alloy::primitives::U256;
use connector_utils::types::GatewayEvent;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tracing::{error, info, warn};

/// Interface used to remove Gateway's events from some storage.
pub trait EventRemover: Send {
    type Event;

    fn remove_event(&self, event: Self::Event) -> impl Future<Output = ()> + Send;
}

/// Struct that removes Gateway's events from a `Postgres` database.
#[derive(Clone)]
pub struct DbEventRemover {
    db_pool: Pool<Postgres>,
}

impl EventRemover for DbEventRemover {
    type Event = GatewayEvent;

    async fn remove_event(&self, event: Self::Event) {
        match self.remove_from_db(&event).await {
            Ok(query_result) => {
                if query_result.rows_affected() == 1 {
                    info!("Successfully removed {event} from DB!");
                } else {
                    warn!(
                        "Unexpected query result while removing {}: {:?}",
                        event, query_result
                    )
                }
            }

            Err(err) => error!("Failed to remove {event} from DB: {err}"),
        }
    }
}

impl DbEventRemover {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    async fn remove_from_db(&self, event: &GatewayEvent) -> sqlx::Result<PgQueryResult> {
        match event {
            GatewayEvent::PublicDecryption(e) => {
                self.remove_public_decryption(e.decryptionId).await
            }
            GatewayEvent::UserDecryption(e) => self.remove_user_decryption(e.decryptionId).await,
            GatewayEvent::PreprocessKeygen(e) => {
                self.remove_preprocess_keygen(e.preKeygenRequestId).await
            }
            GatewayEvent::PreprocessKskgen(e) => {
                self.remove_preprocess_kskgen(e.preKskgenRequestId).await
            }
            GatewayEvent::Keygen(e) => self.remove_keygen(e.preKeyId).await,
            GatewayEvent::Kskgen(e) => self.remove_kskgen(e.preKskId).await,
            GatewayEvent::Crsgen(e) => self.remove_crsgen(e.crsgenRequestId).await,
        }
    }

    async fn remove_public_decryption(&self, decryption_id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM public_decryption_requests WHERE decryption_id = $1",
            decryption_id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_user_decryption(&self, decryption_id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM user_decryption_requests WHERE decryption_id = $1",
            decryption_id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_preprocess_keygen(&self, id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM preprocess_keygen_requests WHERE pre_keygen_request_id = $1",
            id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_preprocess_kskgen(&self, id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM preprocess_kskgen_requests WHERE pre_kskgen_request_id = $1",
            id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_keygen(&self, id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM keygen_requests WHERE pre_key_id = $1",
            id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_kskgen(&self, id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM kskgen_requests WHERE pre_ksk_id = $1",
            id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_crsgen(&self, id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM crsgen_requests WHERE crsgen_request_id = $1",
            id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }
}
