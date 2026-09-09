//! Exercise the shared worker lifecycle with authorization outcomes injected at the processor
//! boundary. Proof/permit suites cover authorization; this suite covers durable job handling.
use alloy::primitives::U256;
use connector_utils::{
    tests::{
        db::requests::{InsertRequestOptions, TestEventType, insert_rand_request},
        setup::DbInstance,
    },
    types::{
        KmsGrpcResponse, KmsResponseKind, ProtocolEvent, ProtocolEventKind,
        db::{OperationStatus, RequestSource},
    },
};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{UserDecryptionResponse, UserDecryptionResponsePayload};
use kms_worker::core::{
    Config, DbEventPicker, DbKmsResponsePublisher, KmsWorker,
    event_processor::{EventProcessor, ProcessingError, RequestCheckError},
    solana::{
        failure::AuthorizationFailure, handle_binding::HandleBindingFailure,
        snapshot::SnapshotError,
    },
};
use rstest::rstest;
use sqlx::{PgPool, Row};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Copy, Debug)]
enum Failure {
    Terminal,
    Transient,
    Stale,
}

impl Failure {
    fn error(self, solana: bool) -> ProcessingError {
        if solana {
            let failure = match self {
                Self::Terminal => AuthorizationFailure::SignatureMismatch,
                Self::Transient => AuthorizationFailure::Snapshot(SnapshotError::Unavailable {
                    reason: "RPC unavailable".into(),
                }),
                Self::Stale => AuthorizationFailure::HandleBinding {
                    index: 0,
                    source: HandleBindingFailure::ProofRecordBehind {
                        record_leaf_count: 1,
                        live_leaf_count: 2,
                    },
                },
            };
            RequestCheckError::from(failure).record()
        } else {
            match self {
                Self::Terminal => ProcessingError::irrecoverable(
                    ErrorCode::Unprocessable,
                    anyhow::anyhow!("invalid request"),
                ),
                Self::Transient | Self::Stale => {
                    ProcessingError::transient(anyhow::anyhow!("host state not available yet"))
                }
            }
        }
    }
}

#[derive(Clone)]
struct Processor {
    solana: bool,
    failure: Failure,
    attempts: Arc<AtomicUsize>,
}

impl EventProcessor for Processor {
    type Event = ProtocolEvent;
    async fn process(
        &mut self,
        event: &mut ProtocolEvent,
    ) -> Result<Option<KmsResponseKind>, ProcessingError> {
        let id = match (&event.kind, self.solana) {
            (ProtocolEventKind::UserDecryptionV3(request), true) => request.decryptionId,
            (ProtocolEventKind::UserDecryptionV2(request), false) => request.decryptionId,
            _ => panic!("picker routed the request to the wrong protocol"),
        };
        if id == U256::from(1) {
            self.attempts.fetch_add(1, Ordering::SeqCst);
            event.error_counter += 1;
            return Err(self.failure.error(self.solana));
        }
        Ok(Some(
            KmsResponseKind::process(KmsGrpcResponse::UserDecryption {
                decryption_id: id,
                grpc_response: UserDecryptionResponse {
                    payload: Some(UserDecryptionResponsePayload::default()),
                    ..Default::default()
                },
            })
            .expect("valid mock KMS response"),
        ))
    }
}

async fn insert(db: &PgPool, id: u64, solana: bool, source: RequestSource) -> anyhow::Result<()> {
    insert_rand_request(
        db,
        TestEventType::UserDecryptionV2,
        InsertRequestOptions::new()
            .with_id(U256::from(id))
            .with_source(source)
            .with_status(OperationStatus::UnderProcess),
    )
    .await?;
    if solana {
        // The processor is the injection boundary: only the picker needs the V3 discriminator.
        sqlx::query(
            "UPDATE user_decryption_requests SET solana_request = $1 WHERE decryption_id = $2",
        )
        .bind(b"{}".as_slice())
        .bind(U256::from(id).to_le_bytes::<32>().as_slice())
        .execute(db)
        .await?;
    }
    sqlx::query("UPDATE user_decryption_requests SET status = 'pending' WHERE decryption_id = $1")
        .bind(U256::from(id).to_le_bytes::<32>().as_slice())
        .execute(db)
        .await?;
    Ok(())
}

async fn wait_status(db: &PgPool, id: u64, expected: &str) -> anyhow::Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let status: String = sqlx::query_scalar(
                "SELECT status::text FROM user_decryption_requests WHERE decryption_id = $1",
            )
            .bind(U256::from(id).to_le_bytes::<32>().as_slice())
            .fetch_one(db)
            .await?;
            if status == expected {
                return Ok::<_, anyhow::Error>(());
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await??;
    Ok(())
}

#[rstest]
#[tokio::test]
async fn errors_preserve_the_shared_worker_lifecycle(
    #[values(false, true)] solana: bool,
    #[values(RequestSource::OnChain, RequestSource::Http)] source: RequestSource,
    #[values(Failure::Terminal, Failure::Transient, Failure::Stale)] failure: Failure,
) -> anyhow::Result<()> {
    let instance = DbInstance::setup_external().await?;
    let config = Config {
        db_fast_event_polling: Duration::from_millis(20),
        ..Default::default()
    };
    // Insert before starting the picker, so the V3 discriminator update cannot race a claim.
    insert(&instance.db, 1, solana, source).await?;
    let picker = DbEventPicker::connect(instance.db.clone(), &config).await?;
    let attempts = Arc::new(AtomicUsize::new(0));
    let worker = KmsWorker::new(
        picker,
        Processor {
            solana,
            failure,
            attempts: attempts.clone(),
        },
        DbKmsResponsePublisher::new(instance.db.clone()),
        3,
    );
    let cancel = CancellationToken::new();
    let _cancel_on_drop = cancel.clone().drop_guard();
    let task = tokio::spawn(worker.start(cancel.clone()));
    wait_status(&instance.db, 1, "failed").await?;
    let expected_attempts =
        if source == RequestSource::OnChain && !matches!(failure, Failure::Terminal) {
            3
        } else {
            1
        };
    assert_eq!(attempts.load(Ordering::SeqCst), expected_attempts);
    let response = sqlx::query(
        "SELECT error_code, error_details FROM user_decryption_responses WHERE decryption_id = $1",
    )
    .bind(U256::from(1).to_le_bytes::<32>().as_slice())
    .fetch_optional(&instance.db)
    .await?;
    if source == RequestSource::Http {
        let response = response.expect("HTTP failures produce a caller-visible response");
        assert_eq!(
            response.get::<String, _>("error_code"),
            match failure {
                Failure::Terminal => "unprocessable",
                Failure::Transient | Failure::Stale => "upstream_transient",
            }
        );
        assert!(!response.get::<String, _>("error_details").is_empty());
    } else {
        assert!(
            response.is_none(),
            "on-chain failures do not create HTTP error responses"
        );
    }
    assert!(
        !task.is_finished(),
        "a request failure must not stop the worker"
    );
    // Exercise the same worker after failure with a successful event and real DB publishing.
    insert(&instance.db, 2, solana, source).await?;
    wait_status(&instance.db, 2, "completed").await?;
    cancel.cancel();
    task.await?;
    Ok(())
}
