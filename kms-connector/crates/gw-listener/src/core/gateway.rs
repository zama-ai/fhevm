use crate::{
    core::{
        Config,
        publish::{ChainName, publish_batch},
    },
    monitoring::metrics::{EVENT_LISTENING_ERRORS, EVENT_RECEIVED_COUNTER, EVENT_REJECTED_COUNTER},
};
use alloy::{
    network::Ethereum,
    primitives::B256,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEventInterface,
};
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        ProtocolEvent, ProtocolEventKind,
        db::{EventType, RequestSource},
        solana_request::{SolanaPublicDecryptionRequest, SolanaUserDecryptionRequestV1},
    },
};
use fhevm_gateway_bindings::decryption::Decryption::DecryptionEvents;
use sqlx::{Pool, Postgres};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, info_span, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

const DECRYPTION_EVENT_TYPES: [EventType; 2] = [
    EventType::PublicDecryptionRequest,
    EventType::UserDecryptionRequest,
];

/// The handles of a decryption event, for trace lookup by handle.
fn ct_handles(event: &ProtocolEventKind) -> Vec<B256> {
    match event {
        ProtocolEventKind::PublicDecryption(e) => e.ctHandles.clone(),
        ProtocolEventKind::SolanaPublicDecryption(e) => e.ct_handles(),
        ProtocolEventKind::UserDecryption(e) => e.ctHandles.clone(),
        ProtocolEventKind::UserDecryptionV2(e) => e.handles.iter().map(|h| h.handle).collect(),
        ProtocolEventKind::SolanaUserDecryptionV1(e) => e.ct_handles(),
        _ => Vec::new(),
    }
}

/// Struct monitoring and storing Gateway's decryption events.
pub struct GatewayListener<P>
where
    P: Provider,
{
    /// The database pool for storing Gateway's events.
    db_pool: Pool<Postgres>,

    /// The Gateway RPC Provider.
    provider: P,

    /// The configuration of the `GatewayListener`.
    config: Config,

    /// The cancellation token to handle the graceful shutdown of the listener.
    cancel_token: CancellationToken,
}

impl<P> GatewayListener<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Creates a new `GatewayListener` instance.
    pub fn new(
        db_pool: Pool<Postgres>,
        provider: P,
        config: &Config,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            db_pool,
            provider,
            config: config.clone(),
            cancel_token,
        }
    }

    /// Starts the `GatewayListener`.
    ///
    /// Polls for Decryption events on the Gateway chain.
    pub async fn start(self) {
        select! {
            biased;
            _ = self.cancel_token.cancelled() => info!("Decryption polling cancelled..."),
            result = self.run_poll_loop() => if let Err(e) = result {
                error!("Decryption polling failed: {e}");
            }
        }
        self.cancel_token.cancel();
        info!("GatewayListener stopped successfully!");
    }

    /// Polling loop to listen to [`Decryption`] contract events.
    async fn run_poll_loop(&self) -> anyhow::Result<()> {
        let contract_address = self.config.decryption_contract.address;
        let poll_interval = self.config.decryption_polling;
        let from_block_config = self.config.decryption_from_block_number;
        let event_types = DECRYPTION_EVENT_TYPES.as_slice();

        // `UserDecryptionRequest` contributes both the legacy and the RFC016 topic0 hashes, which
        // is why we flat-map over `signature_hashes()` instead of collecting a single hash per
        // event type.
        let event_signatures = event_types
            .iter()
            .flat_map(|e| e.signature_hashes())
            .collect::<Vec<_>>();
        let base_filter = Filter::new()
            .address(contract_address)
            .event_signature(event_signatures);

        let mut from_block = match from_block_config {
            Some(from_block) => {
                info!("Found configured from_block_number ({from_block}) for polling");
                from_block
            }
            None => self.fetch_start_block().await?,
        };

        info!("Started Decryption polling from block {from_block}");

        let mut ticker = tokio::time::interval(poll_interval);
        let max_errors = self.config.max_consecutive_polling_errors;
        let mut consecutive_errors: usize = 0;
        loop {
            ticker.tick().await;
            match self
                .fetch_and_publish(base_filter.clone(), from_block)
                .await
            {
                Ok((new_from_block, has_more)) => {
                    consecutive_errors = 0;
                    from_block = new_from_block;
                    if has_more {
                        ticker.reset_immediately();
                    }
                }
                Err(e) => {
                    EVENT_LISTENING_ERRORS
                        .with_label_values(&["decryption"])
                        .inc();
                    consecutive_errors = consecutive_errors.saturating_add(1);
                    warn!("Decryption listening error: {e} ({consecutive_errors}/{max_errors})");
                    if consecutive_errors >= max_errors {
                        anyhow::bail!("Too many consecutive errors for Decryption");
                    }
                }
            }
        }
    }

    /// Fetches logs for a block range, decodes them, and publishes them in a single transaction.
    ///
    /// Returns `(new_from_block, has_more_blocks)`.
    async fn fetch_and_publish(
        &self,
        base_filter: Filter,
        from_block: u64,
    ) -> anyhow::Result<(u64, bool)> {
        let current_block = self.provider.get_block_number().await?;

        if from_block > current_block {
            return Ok((from_block, false));
        }

        let to_block = std::cmp::min(
            from_block.saturating_add(self.config.get_logs_batch_size.saturating_sub(1)),
            current_block,
        );

        let filter = base_filter.from_block(from_block).to_block(to_block);

        let logs = self.provider.get_logs(&filter).await?;
        let events = Self::prepare_events(logs)?;
        publish_batch(&self.db_pool, events, ChainName::Gateway, to_block).await?;

        Ok((to_block.saturating_add(1), to_block < current_block))
    }

    /// Decodes logs and prepares `ProtocolEvent` structs with OTLP context and metrics.
    fn prepare_events(logs: Vec<Log>) -> anyhow::Result<Vec<ProtocolEvent>> {
        let mut events = Vec::with_capacity(logs.len());
        for log in logs {
            let event = DecryptionEvents::decode_log(&log.inner)
                .map_err(|e| anyhow!("Failed to decode Decryption event: {e}"))?;
            let event_kind = match event.data {
                DecryptionEvents::PublicDecryptionRequest_2(event) => {
                    let decryption_id = event.decryptionId;
                    match SolanaPublicDecryptionRequest::try_from(event) {
                        Ok(request) => request.into(),
                        Err(e) => {
                            warn!(
                                %decryption_id,
                                tx_hash = ?log.transaction_hash,
                                "Skipping Solana public decryption that does not decode: {e:#}"
                            );
                            EVENT_REJECTED_COUNTER
                                .with_label_values(&[EventType::PublicDecryptionRequest.as_str()])
                                .inc();
                            continue;
                        }
                    }
                }
                DecryptionEvents::UserDecryptionRequest_4(event) => {
                    let decryption_id = event.decryptionId;
                    match SolanaUserDecryptionRequestV1::try_from(event) {
                        Ok(request) => request.into(),
                        Err(e) => {
                            warn!(
                                %decryption_id,
                                tx_hash = ?log.transaction_hash,
                                "Skipping Solana user decryption that does not decode: {e:#}"
                            );
                            EVENT_REJECTED_COUNTER
                                .with_label_values(&[EventType::UserDecryptionRequest.as_str()])
                                .inc();
                            continue;
                        }
                    }
                }
                event => event.try_into()?,
            };
            let event_type = EventType::from(&event_kind).as_str();
            EVENT_RECEIVED_COUNTER
                .with_label_values(&[event_type])
                .inc();

            let span = info_span!(
                "handle_gateway_event",
                event = %event_kind,
                event_type,
                source = %RequestSource::OnChain,
                ciphertext_handle = tracing::field::Empty,
            );
            if let [handle] = ct_handles(&event_kind).as_slice() {
                span.record("ciphertext_handle", format!("{handle:#x}"));
            }
            let otlp_ctx = PropagationContext::inject(&span.context());
            events.push(ProtocolEvent::new(
                event_kind,
                log.transaction_hash,
                otlp_ctx,
                RequestSource::OnChain,
            ));
        }
        Ok(events)
    }

    /// Determines the block to start event listening from.
    async fn fetch_start_block(&self) -> anyhow::Result<u64> {
        let chain = ChainName::Gateway.as_str();
        info!("Fetching last block polled from DB for chain {chain}...");
        let last_block_polled = sqlx::query_scalar!(
            "SELECT block_number FROM last_block_polled_by_chain WHERE chain_name = $1",
            chain,
        )
        .fetch_one(&self.db_pool)
        .await?;

        match last_block_polled {
            Some(block_i64) => {
                let block = u64::try_from(block_i64).expect("block_number should be a valid u64");
                Ok(block.checked_add(1).expect("block < u64::MAX"))
            }
            None => {
                info!("No block polled yet. Listening from latest block number instead...");
                Ok(self.provider.get_block_number().await?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Bytes, FixedBytes, U256};
    use alloy::providers::{
        Identity, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        mock::Asserter,
    };
    use alloy::rpc::json_rpc::ErrorPayload;
    use connector_utils::tests::setup::{TestInstance, TestInstanceBuilder};
    use fhevm_gateway_bindings::decryption::IDecryption::RequestValiditySeconds;
    use std::time::Duration;

    #[rstest::rstest]
    fn user_decryption_span_carries_the_handle(#[values(false, true)] solana: bool) {
        use alloy::sol_types::SolEvent;
        use connector_utils::tests::rand::solana_user_decryption_event;
        use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_2;
        use tracing_subscriber::fmt::format::FmtSpan;
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(false)
            .with_span_events(FmtSpan::CLOSE)
            .with_writer(connector_utils::tests::setup::CustomTestWriter::new(sender))
            .finish();
        let handle = FixedBytes::repeat_byte(0xab);
        let data = if solana {
            solana_user_decryption_event(U256::from(42), handle).encode_log_data()
        } else {
            UserDecryptionRequest_2 {
                decryptionId: U256::from(42),
                ctHandles: vec![handle],
                ..Default::default()
            }
            .encode_log_data()
        };
        let log = Log {
            inner: alloy::primitives::Log {
                address: Default::default(),
                data,
            },
            ..Default::default()
        };
        tracing::subscriber::with_default(subscriber, || {
            GatewayListener::<RootProvider>::prepare_events(vec![log]).unwrap();
        });
        let mut output = String::new();
        while let Ok(bytes) = receiver.try_recv() {
            output.push_str(&String::from_utf8(bytes).unwrap());
        }
        for attribute in [
            "event_type=\"user_decryption_request\"".to_owned(),
            "source=onchain".to_owned(),
            format!("ciphertext_handle=\"{handle:#x}\""),
        ] {
            assert!(output.contains(&attribute), "missing {attribute}: {output}");
        }
    }

    #[tokio::test]
    async fn malformed_solana_event_does_not_discard_valid_peers() {
        use alloy::sol_types::SolEvent;
        use fhevm_gateway_bindings::decryption::Decryption::{
            PublicDecryptionRequest_1, PublicDecryptionRequest_2, UserDecryptionRequest_4,
        };
        let invalid_user = UserDecryptionRequest_4 {
            decryptionId: U256::from(42),
            ctHandles: vec![],
            requestValidity: RequestValiditySeconds::default(),
            publicKey: Bytes::new(),
            extraData: Bytes::new(),
            solanaRequest: Bytes::new(),
        };
        // A Solana public decryption naming fewer stores than handles.
        let invalid_public = PublicDecryptionRequest_2 {
            decryptionId: U256::from(44),
            ctHandles: vec![FixedBytes::ZERO; 2],
            extraData: Bytes::new(),
            encryptedStores: vec![FixedBytes::ZERO],
        };
        let valid = PublicDecryptionRequest_1 {
            decryptionId: U256::from(43),
            ctHandles: vec![FixedBytes::ZERO],
            extraData: Bytes::new(),
        };
        let log = |data| Log {
            inner: alloy::primitives::Log {
                address: Default::default(),
                data,
            },
            ..Default::default()
        };
        let logs = vec![
            log(invalid_user.encode_log_data()),
            log(invalid_public.encode_log_data()),
            log(valid.encode_log_data()),
        ];
        let events = GatewayListener::<RootProvider>::prepare_events(logs).unwrap();
        assert_eq!(events.len(), 1);
        assert!(
            matches!(&events[0].kind, ProtocolEventKind::PublicDecryption(e) if e.decryptionId == U256::from(43))
        );
        let db = connector_utils::tests::setup::DbInstance::setup_external()
            .await
            .unwrap();
        publish_batch(&db.db, events, ChainName::Gateway, 123)
            .await
            .unwrap();
        let saved: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM public_decryption_requests WHERE decryption_id = $1",
        )
        .bind(U256::from(43).as_le_slice())
        .fetch_one(&db.db)
        .await
        .unwrap();
        let cursor: i64 = sqlx::query_scalar(
            "SELECT block_number FROM last_block_polled_by_chain WHERE chain_name = 'gateway'",
        )
        .fetch_one(&db.db)
        .await
        .unwrap();
        assert_eq!(saved, 1);
        assert_eq!(cursor, 123);
    }

    #[test]
    fn malformed_abi_still_stops_the_batch() {
        assert!(GatewayListener::<RootProvider>::prepare_events(vec![Log::default()]).is_err());
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_consecutive_get_logs_error_stops_listener() {
        // `from_block_number` is configured so the poll loop doesn't read the cursor from DB.
        let (_test_instance, asserter, gw_listener) = test_setup(Some(100));

        for _ in 0..MAX_CONSECUTIVE_POLLING_ERRORS {
            // Loop get_block_number succeeds
            asserter.push_success(&101_u64);

            // get_logs fails
            asserter.push_failure(ErrorPayload {
                code: -32000,
                message: "get logs error".into(),
                data: None,
            });
        }

        gw_listener.start().await;
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_listener_ended_by_cancel_token() {
        let (mut test_instance, _asserter, gw_listener) = test_setup(None);

        gw_listener.cancel_token.cancel();

        gw_listener.start().await;
        test_instance
            .wait_for_log("GatewayListener stopped successfully")
            .await;
    }

    type MockProvider = FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >;

    const MAX_CONSECUTIVE_POLLING_ERRORS: usize = 2;

    fn test_setup(
        decryption_from_block_number: Option<u64>,
    ) -> (TestInstance, Asserter, GatewayListener<MockProvider>) {
        let test_instance = TestInstanceBuilder::default().build();
        // Use a lazy DB pool as tests do not need a real Postgres server
        let db_pool = Pool::<Postgres>::connect_lazy("postgres://unused").unwrap();

        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        let config = Config {
            decryption_polling: Duration::from_millis(500),
            decryption_from_block_number,
            max_consecutive_polling_errors: MAX_CONSECUTIVE_POLLING_ERRORS,
            ..Default::default()
        };
        let listener =
            GatewayListener::new(db_pool, mock_provider, &config, CancellationToken::new());
        (test_instance, asserter, listener)
    }
}
