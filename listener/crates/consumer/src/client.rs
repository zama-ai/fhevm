use alloy_primitives::Address;
use async_trait::async_trait;
pub use broker::redis::GroupStatus;
use broker::redis::{StreamManager, id_before};
pub use broker::{AckDecision, Broker, HandlerError};
use broker::{BrokerError, CancellationToken, Consumer, Handler, Message, Topic};
use primitives::event::{
    BlockPayload, CancelCatchupPayload, CatchupPayload, FilterCommand,
    FilterCommandValidationError, FilterType, WatchCommand,
};
use primitives::routing;
use primitives::utils::chain_id_to_namespace;
use std::future::Future;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

pub use crate::error::ConsumerError;
use crate::options::{CatchupConsumerOptions, LiveConsumerOptions};

/// Redis start position meaning "deliver only what arrives from now on".
const NEW_ENTRIES_ONLY: &str = "$";

/// Where `name` sits in the succession of groups on `stream`, or `None` if it
/// is not part of one.
///
/// The unsuffixed group is rung 0. It comes from a build that predates group
/// suffixes entirely, so every suffixed build is newer than it, and a build
/// that finds it is the first of the versioned ones.
///
/// Anything that is not `{stream}` or `{stream}.v{digits}` is unranked: a
/// group belonging to a different stream, or a suffix in a shape this
/// ordering was not built to compare.
fn rung(stream: &str, name: &str) -> Option<u32> {
    if name == stream {
        return Some(0);
    }
    name.strip_prefix(stream)
        .and_then(|rest| rest.strip_prefix(".v"))
        .and_then(|digits| digits.parse().ok())
}

/// The group on `stream` whose position a group with `suffix` should start
/// from: the highest rung below its own.
///
/// `None` means there is nothing older to inherit — an empty stream, a stream
/// carrying only newer groups, or a suffix that does not name a version,
/// which is a shape this ordering cannot rank and so will not guess at.
fn predecessor_group<'a>(
    stream: &str,
    suffix: Option<&str>,
    existing: &'a [GroupStatus],
) -> Option<&'a GroupStatus> {
    let mine = rung(stream, &format!("{stream}.{}", suffix?))?;

    existing
        .iter()
        .filter_map(|candidate| {
            rung(stream, &candidate.name)
                .filter(|rung| *rung < mine)
                .map(|rung| (rung, candidate))
        })
        .max_by_key(|(rung, _)| *rung)
        .map(|(_, candidate)| candidate)
}

/// The groups on `stream` that a client with `suffix` releases once its own
/// version is no longer ahead of the live one: every versioned rung strictly
/// below its own.
///
/// Rung 0 is left alone. The unsuffixed group belongs to a build that predates
/// versioning and cannot see a cutover, so being behind the live version does
/// not mean it has stopped reading. Its own group, newer ones, unranked names
/// and an unrankable `suffix` are never returned either.
fn superseded_groups<'a>(
    stream: &str,
    suffix: Option<&str>,
    existing: &'a [GroupStatus],
) -> Vec<&'a GroupStatus> {
    let Some(mine) = suffix.and_then(|suffix| rung(stream, &format!("{stream}.{suffix}"))) else {
        return Vec::new();
    };

    existing
        .iter()
        .filter(|candidate| {
            rung(stream, &candidate.name).is_some_and(|rung| 0 < rung && rung < mine)
        })
        .collect()
}

/// Chain & Consumer-scoped client for the consumer library.
///
/// Downstream services instantiate this once with their broker connection and
/// target chain, then call instance methods for watch/unwatch operations.
///
/// # Cancellation
///
/// Five tokens cooperate:
///
/// - [`cancel_token`](Self::cancel_token) — parent. Cancelling it stops every
///   flow (live, catchup, final, final catchup).
/// - `live_cancel` — child of `cancel_token`. Wired into the live consumer
///   and the live handler. Cancel via [`cancel_live`](Self::cancel_live) to
///   stop only the live flow.
/// - `catchup_cancel` — child of `cancel_token`. Wired into the catchup
///   consumer and the catchup handler. Cancel via
///   [`cancel_catchup`](Self::cancel_catchup) to stop only catchup
///   (typical use: stop the bounded backfill once it has drained while the
///   live stream keeps running).
/// - `final_cancel` — child of `cancel_token`. Wired into the finalized-only
///   consumer and handler. Cancel via [`cancel_final`](Self::cancel_final).
/// - `final_catchup_cancel` — child of `cancel_token`. Wired into the final
///   catchup consumer and handler. Cancel via
///   [`cancel_final_catchup`](Self::cancel_final_catchup).
///
/// Cancelling a child does *not* propagate up — the parent and the siblings
/// keep going. Cancelling the parent cancels every child.
#[derive(Clone)]
pub struct ListenerConsumer {
    broker: Broker,
    chain_id: u64,
    consumer_id: String,
    /// Parent cancellation token. Cancelling this stops all flows.
    pub cancel_token: CancellationToken,
    /// Child token: cancels only the live flow.
    live_cancel: CancellationToken,
    /// Child token: cancels only the catchup flow.
    catchup_cancel: CancellationToken,
    /// Child token: cancels only the final (finalized-only) live flow.
    final_cancel: CancellationToken,
    /// Child token: cancels only the final catchup flow.
    final_catchup_cancel: CancellationToken,
    live_options: LiveConsumerOptions,
    catchup_options: CatchupConsumerOptions,
    /// Optional suffix appended to every delivery group name.
    ///
    /// See [`with_group_suffix`](Self::with_group_suffix).
    group_suffix: Option<String>,
}

impl ListenerConsumer {
    /// Create a new consumer bound to a broker and chain ID, with default
    /// tuning for all pipelines.
    pub fn new(broker: &Broker, chain_id: u64, consumer_id: &str) -> Self {
        Self::with_options(broker, chain_id, consumer_id, None, None)
    }

    /// Create a new consumer with explicit per-pipeline tuning. `None` for
    /// either argument falls back to the corresponding `Default` impl, which
    /// matches the values previously hardcoded in this file.
    ///
    /// The finalized-only flow ([`consume_final`](Self::consume_final))
    /// shares the `live` tuning, and the final catchup flow
    /// ([`consume_final_catchup`](Self::consume_final_catchup)) shares the
    /// `catchup` tuning — "same config as the rest" by construction.
    pub fn with_options(
        broker: &Broker,
        chain_id: u64,
        consumer_id: &str,
        live: Option<LiveConsumerOptions>,
        catchup: Option<CatchupConsumerOptions>,
    ) -> Self {
        let consumer_id_trimmed = consumer_id.trim();
        if consumer_id != consumer_id_trimmed {
            warn!(
                "Consumer ID has leading or trailing whitespace, which may cause issues with routing. Consider trimming it before passing to ListenerConsumer::new."
            );
        }
        let cancel_token = CancellationToken::new();
        let live_cancel = cancel_token.child_token();
        let catchup_cancel = cancel_token.child_token();
        let final_cancel = cancel_token.child_token();
        let final_catchup_cancel = cancel_token.child_token();
        Self {
            broker: broker.clone(),
            chain_id,
            consumer_id: consumer_id_trimmed.into(),
            cancel_token,
            live_cancel,
            catchup_cancel,
            final_cancel,
            final_catchup_cancel,
            live_options: live.unwrap_or_default(),
            catchup_options: catchup.unwrap_or_default(),
            group_suffix: None,
        }
    }

    /// Append a suffix to every delivery group name this consumer creates.
    ///
    /// Group name and stream name are the same string by default, which means
    /// two builds of the same service cannot read the same stream
    /// independently — they share a group, and every entry goes to exactly one
    /// of them. A suffix separates the two: same stream, same `consumer_id`,
    /// same registered filters, but its own cursor and its own copy of every
    /// entry.
    ///
    /// The caller owns the suffix and therefore owns what counts as "a
    /// different reader". Deriving it from something the two builds genuinely
    /// disagree about — a protocol version, say — makes the separation happen
    /// exactly when it is needed and not otherwise. Deriving it from something
    /// an operator sets by hand makes it a footgun.
    ///
    /// A new group does not start from the beginning of retained history. It
    /// takes over from the newest group ranking below its own suffix, and only
    /// starts at the end of the stream when there is nothing below it to take
    /// over from. Suffixes that rank — `v1`, `v2` and so on, with the
    /// unsuffixed name ranking below all of them — are what make that handover
    /// possible; one that does not rank always starts at the end.
    ///
    /// On AMQP the group is the queue, so changing the suffix points the
    /// consumer at a different queue and anything still in the old one is left
    /// behind.
    pub fn with_group_suffix(mut self, suffix: impl Into<String>) -> Self {
        let suffix = suffix.into();
        self.group_suffix = if suffix.is_empty() {
            None
        } else {
            Some(suffix)
        };
        self
    }

    /// Delivery group name for `topic` — the topic itself, plus the suffix
    /// from [`with_group_suffix`](Self::with_group_suffix) if one is set.
    ///
    /// Kept in one place so the four pipelines cannot drift apart: they must
    /// either all carry the suffix or none of them do, or a build reads some
    /// of its streams independently and shares the rest.
    fn group_for(&self, topic: &Topic) -> String {
        match &self.group_suffix {
            Some(suffix) => format!("{topic}.{suffix}"),
            None => topic.to_string(),
        }
    }

    /// Cancel all flows (live, catchup, final, final catchup).
    ///
    /// Equivalent to cancelling [`cancel_token`](Self::cancel_token) directly.
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// Cancel only the live flow. The catchup flow keeps running.
    pub fn cancel_live(&self) {
        self.live_cancel.cancel();
    }

    /// Cancel only the catchup flow. The live flow keeps running.
    ///
    /// This stops *this client* consuming catchup events. It does not retire
    /// the request on the listener — for that, use
    /// [`cancel_catchup_request`](Self::cancel_catchup_request).
    pub fn cancel_catchup(&self) {
        self.catchup_cancel.cancel();
    }

    /// Cancel only the final (finalized-only) live flow. The other flows
    /// keep running.
    pub fn cancel_final(&self) {
        self.final_cancel.cancel();
    }

    /// Cancel only the final catchup flow. The other flows keep running.
    ///
    /// This stops *this client* consuming final catchup events. It does not
    /// retire the request on the listener — for that, use
    /// [`cancel_final_catchup_request`](Self::cancel_final_catchup_request).
    pub fn cancel_final_catchup(&self) {
        self.final_catchup_cancel.cancel();
    }

    /// Return the chain ID this client publishes into.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Return the consumer ID this client publishes into.
    pub fn consumer_id(&self) -> &str {
        &self.consumer_id
    }

    /// Build a filter command scoped to this consumer.
    fn filter_command(
        &self,
        log_address: Option<Address>,
        filter_type: Option<FilterType>,
    ) -> FilterCommand {
        FilterCommand {
            consumer_id: self.consumer_id.clone(),
            from: None,
            to: None,
            log_address,
            filter_type,
        }
    }

    pub fn create_filter_on_log_address(&self, contract: Address) -> FilterCommand {
        self.filter_command(Some(contract), None)
    }

    /// Build a finalized-only filter command for events emitted by `contract`.
    ///
    /// The watcher only receives events once their block is finalized, so it
    /// never sees reorged blocks. Delivery for final watchers is handled by a
    /// separate flow on the listener side.
    pub fn create_final_filter_on_log_address(&self, contract: Address) -> FilterCommand {
        self.filter_command(Some(contract), Some(FilterType::Final))
    }

    /// Build a full-block (wildcard) filter command for this consumer.
    ///
    /// All address fields are `None`, which the listener interprets as
    /// "broadcast the entire block": every transaction with every log is
    /// delivered, letting the consumer parse the chain dynamically.
    pub fn create_full_block_filter(&self) -> FilterCommand {
        self.filter_command(None, None)
    }

    /// Build a finalized-only full-block (wildcard) filter command.
    ///
    /// Same wildcard semantics as [`create_full_block_filter`](Self::create_full_block_filter),
    /// but blocks are only delivered once finalized.
    pub fn create_final_full_block_filter(&self) -> FilterCommand {
        self.filter_command(None, Some(FilterType::Final))
    }

    /// Register a full-block (wildcard) subscription for this consumer.
    ///
    /// After this call the consumer receives the complete [`BlockPayload`]
    /// (all transactions, all logs) for every block on the chain. Registering
    /// the same full-block filter again is a no-op on the listener side
    /// (deduplicated silently).
    pub async fn register_full_block(&self) -> Result<(), ConsumerError> {
        self.register_filter(&self.create_full_block_filter()).await
    }

    /// Remove this consumer's full-block (wildcard) subscription.
    pub async fn unregister_full_block(&self) -> Result<(), ConsumerError> {
        self.unregister_filter(&self.create_full_block_filter())
            .await
    }

    /// Register a finalized-only full-block (wildcard) subscription.
    pub async fn register_final_full_block(&self) -> Result<(), ConsumerError> {
        self.register_filter(&self.create_final_full_block_filter())
            .await
    }

    /// Remove this consumer's finalized-only full-block (wildcard) subscription.
    pub async fn unregister_final_full_block(&self) -> Result<(), ConsumerError> {
        self.unregister_filter(&self.create_final_full_block_filter())
            .await
    }

    /// Publish a filter removal command to the unwatch topic.
    pub async fn unregister_filter(&self, command: &FilterCommand) -> Result<(), ConsumerError> {
        self.publish_filter_command(command, routing::UNWATCH).await
    }

    /// Publish a filter registration command to the watch topic.
    pub async fn register_filter(&self, command: &FilterCommand) -> Result<(), ConsumerError> {
        self.publish_filter_command(command, routing::WATCH).await
    }

    /// Request a historical catch-up over `[block_start, block_end]` (inclusive).
    ///
    /// Publishes a [`CatchupPayload`] to the chain-namespaced
    /// `routing::CATCHUP` control plane. The listener fans the range out to
    /// its `range-catchup` workers; the resulting events are delivered on
    /// `{consumer_id}.catchup-event` and consumed via
    /// [`consume_catchup`](Self::consume_catchup).
    ///
    /// `catchup_id` is supplied by the caller and identifies this request for
    /// its whole lifetime. It must be a UUIDv7 — mint it with
    /// [`Uuid::now_v7`]; any other version is rejected with
    /// [`ConsumerError::InvalidCatchupPayload`] before anything is published.
    /// Store it durably before calling: it is the only
    /// handle that can later
    /// [`cancel_catchup_request`](Self::cancel_catchup_request) the request.
    /// Re-sending an id the listener has already fanned out or finished is a
    /// no-op, so a crash between persisting the id and this call is recovered
    /// by simply calling again on the next boot. If the listener itself dies
    /// part-way through fanning a request out, it fans the whole range out
    /// again on redelivery: blocks can arrive twice, never zero times.
    pub async fn request_catchup(
        &self,
        catchup_id: Uuid,
        block_start: u64,
        block_end: u64,
    ) -> Result<(), ConsumerError> {
        self.publish_catchup_request(catchup_id, block_start, block_end, routing::CATCHUP)
            .await
    }

    /// Request a historical replay of **finalized** blocks
    /// `[block_start, block_end]` (inclusive).
    ///
    /// Publishes a [`CatchupPayload`] to the chain-namespaced
    /// `routing::FINAL_CATCHUP` control plane. The listener clamps the range
    /// to the finalized head (blocks that are not final yet are skipped —
    /// re-issue the request later), splits it into bounded sub-ranges, and
    /// delivers the resulting events on `{consumer_id}.final-catchup-event`,
    /// consumed via [`consume_final_catchup`](Self::consume_final_catchup).
    ///
    /// Requests are dropped by the listener when its finality flow is
    /// inactive (`finality_active: false`).
    ///
    /// See [`request_catchup`](Self::request_catchup) for the `catchup_id`
    /// contract.
    pub async fn request_final_catchup(
        &self,
        catchup_id: Uuid,
        block_start: u64,
        block_end: u64,
    ) -> Result<(), ConsumerError> {
        self.publish_catchup_request(catchup_id, block_start, block_end, routing::FINAL_CATCHUP)
            .await
    }

    /// Cancel a catch-up previously requested with
    /// [`request_catchup`](Self::request_catchup).
    ///
    /// The listener marks the request cancelled and stops fetching within one
    /// sub-range. Blocks already in flight may still be delivered; they carry
    /// the cancelled `catchup_id` on [`BlockPayload::catchup_id`] so a handler
    /// can drop them.
    ///
    /// Cancelling an id twice, or an id that never existed, is a no-op. An id
    /// that is not a UUIDv7 is rejected with
    /// [`ConsumerError::InvalidCancelCatchupPayload`] before anything is
    /// published.
    /// Cancelling an id owned by a different consumer is rejected by the
    /// listener and recorded there — this call still returns `Ok`.
    ///
    /// This retires the request on the listener. To stop *this client* from
    /// consuming catchup events without touching the listener, use
    /// [`cancel_catchup`](Self::cancel_catchup).
    pub async fn cancel_catchup_request(&self, catchup_id: Uuid) -> Result<(), ConsumerError> {
        self.publish_catchup_cancel(catchup_id, routing::CANCEL_CATCHUP)
            .await
    }

    /// Cancel a final catch-up previously requested with
    /// [`request_final_catchup`](Self::request_final_catchup).
    ///
    /// See [`cancel_catchup_request`](Self::cancel_catchup_request) for the
    /// semantics.
    pub async fn cancel_final_catchup_request(
        &self,
        catchup_id: Uuid,
    ) -> Result<(), ConsumerError> {
        self.publish_catchup_cancel(catchup_id, routing::CANCEL_FINAL_CATCHUP)
            .await
    }

    async fn publish_catchup_request(
        &self,
        catchup_id: Uuid,
        block_start: u64,
        block_end: u64,
        routing_key: &'static str,
    ) -> Result<(), ConsumerError> {
        let mut payload = CatchupPayload {
            consumer_id: self.consumer_id.clone(),
            block_start,
            block_end,
            catchup_id: Some(catchup_id),
            sub_range_seq: None,
        };
        payload.validate()?;
        let namespace = chain_id_to_namespace(self.chain_id);
        let publisher = self.broker.publisher(&namespace).await?;
        publisher.publish(routing_key, &payload).await?;
        info!(
            %catchup_id,
            consumer_id = %self.consumer_id,
            chain_id = self.chain_id,
            block_start,
            block_end,
            routing_key,
            "Requested catchup"
        );
        Ok(())
    }

    async fn publish_catchup_cancel(
        &self,
        catchup_id: Uuid,
        routing_key: &'static str,
    ) -> Result<(), ConsumerError> {
        let mut payload = CancelCatchupPayload {
            consumer_id: self.consumer_id.clone(),
            catchup_id,
        };
        payload.validate()?;
        let namespace = chain_id_to_namespace(self.chain_id);
        let publisher = self.broker.publisher(&namespace).await?;
        publisher.publish(routing_key, &payload).await?;
        info!(
            %catchup_id,
            consumer_id = %self.consumer_id,
            chain_id = self.chain_id,
            routing_key,
            "Requested catchup cancellation"
        );
        Ok(())
    }

    async fn publish_filter_command(
        &self,
        command: &FilterCommand,
        routing_key: &'static str,
    ) -> Result<(), ConsumerError> {
        let mut command = command.clone();
        if command.consumer_id != self.consumer_id {
            return Err(ConsumerError::InconsistentConsumerId(
                command.consumer_id.clone(),
                self.consumer_id.clone(),
            ));
        }
        command.validate()?;

        let namespace = chain_id_to_namespace(self.chain_id);
        let publisher = self.broker.publisher(&namespace).await?;
        publisher.publish(routing_key, &command).await?;
        Ok(())
    }

    pub fn consumer_topic(&self) -> Topic {
        let routing = routing::consumer_new_event_routing(self.consumer_id.clone());
        Topic::new(routing)
    }

    pub fn catchup_consumer_topic(&self) -> Topic {
        let routing = routing::consumer_catchup_event_routing(self.consumer_id.clone());
        Topic::new(routing)
    }

    /// Topic of this consumer's finalized-only event queue:
    /// `{consumer_id}.final-event`.
    pub fn final_consumer_topic(&self) -> Topic {
        let routing = routing::consumer_final_event_routing(self.consumer_id.clone());
        Topic::new(routing)
    }

    /// Topic of this consumer's final catchup queue:
    /// `{consumer_id}.final-catchup-event`.
    pub fn final_catchup_consumer_topic(&self) -> Topic {
        let routing = routing::consumer_final_catchup_event_routing(self.consumer_id.clone());
        Topic::new(routing)
    }

    fn broker_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.consumer_topic();
        let cancel = self.live_cancel.clone();
        let opts = &self.live_options;

        let mut builder = self
            .broker
            .consumer(&topic)
            .group(self.group_for(&topic))
            .prefetch(opts.prefetch())
            .max_retries(opts.max_retries())
            .with_cancellation(cancel);

        if let Some((threshold, cooldown)) = opts.circuit_breaker() {
            builder = builder.circuit_breaker(threshold, cooldown);
        }

        let builder = match &self.broker {
            Broker::Redis { .. } => {
                let mut b = builder;
                if let Some(d) = opts.redis_claim_min_idle() {
                    b = b.redis_claim_min_idle(d.as_secs());
                }
                if let Some(d) = opts.redis_claim_interval() {
                    b = b.redis_claim_interval(d.as_secs());
                }
                b
            }
            Broker::Amqp { .. } => builder,
        };

        builder.build()
    }

    /// Build the finalized-only broker consumer: `{consumer_id}.final-event`
    /// with the final cancel token and the same tuning as the live pipeline
    /// ([`LiveConsumerOptions`]).
    fn broker_final_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.final_consumer_topic();
        let cancel = self.final_cancel.clone();
        let opts = &self.live_options;

        let mut builder = self
            .broker
            .consumer(&topic)
            .group(self.group_for(&topic))
            .prefetch(opts.prefetch())
            .max_retries(opts.max_retries())
            .with_cancellation(cancel);

        if let Some((threshold, cooldown)) = opts.circuit_breaker() {
            builder = builder.circuit_breaker(threshold, cooldown);
        }

        let builder = match &self.broker {
            Broker::Redis { .. } => {
                let mut b = builder;
                if let Some(d) = opts.redis_claim_min_idle() {
                    b = b.redis_claim_min_idle(d.as_secs());
                }
                if let Some(d) = opts.redis_claim_interval() {
                    b = b.redis_claim_interval(d.as_secs());
                }
                b
            }
            Broker::Amqp { .. } => builder,
        };

        builder.build()
    }

    /// Build the final catchup broker consumer:
    /// `{consumer_id}.final-catchup-event` with the final catchup cancel
    /// token and the same tuning as the catchup pipeline
    /// ([`CatchupConsumerOptions`]).
    fn broker_final_catchup_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.final_catchup_consumer_topic();
        let cancel = self.final_catchup_cancel.clone();
        let opts = &self.catchup_options;

        let mut builder = self
            .broker
            .consumer(&topic)
            .group(self.group_for(&topic))
            .prefetch(opts.prefetch())
            .max_retries(opts.max_retries())
            .with_cancellation(cancel);

        if let Some((threshold, cooldown)) = opts.circuit_breaker() {
            builder = builder.circuit_breaker(threshold, cooldown);
        }

        let builder = match &self.broker {
            Broker::Redis { .. } => {
                let mut b = builder;
                if let Some(d) = opts.redis_claim_min_idle() {
                    b = b.redis_claim_min_idle(d.as_secs());
                }
                if let Some(d) = opts.redis_claim_interval() {
                    b = b.redis_claim_interval(d.as_secs());
                }
                b
            }
            Broker::Amqp { .. } => builder,
        };

        builder.build()
    }

    fn broker_catchup_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.catchup_consumer_topic();
        let cancel = self.catchup_cancel.clone();
        let opts = &self.catchup_options;

        let mut builder = self
            .broker
            .consumer(&topic)
            .group(self.group_for(&topic))
            .prefetch(opts.prefetch())
            .max_retries(opts.max_retries())
            .with_cancellation(cancel);

        if let Some((threshold, cooldown)) = opts.circuit_breaker() {
            builder = builder.circuit_breaker(threshold, cooldown);
        }

        let builder = match &self.broker {
            Broker::Redis { .. } => {
                let mut b = builder;
                if let Some(d) = opts.redis_claim_min_idle() {
                    b = b.redis_claim_min_idle(d.as_secs());
                }
                if let Some(d) = opts.redis_claim_interval() {
                    b = b.redis_claim_interval(d.as_secs());
                }
                b
            }
            Broker::Amqp { .. } => builder,
        };

        builder.build()
    }

    /// Register the complete live contract set in one atomic WATCH command.
    ///
    /// Declare delivery queues before calling this method. Core validates all
    /// filters and adds them in one transaction; existing filters are retained.
    /// An empty contract list is rejected. Success confirms command publication,
    /// not database activation. Requires core with atomic WATCH support.
    /// `register_filter` remains available for a single custom filter.
    pub async fn register_contracts(&self, contracts: &[Address]) -> Result<(), ConsumerError> {
        let mut command = WatchCommand::Atomic(
            contracts
                .iter()
                .map(|address| self.create_filter_on_log_address(*address))
                .collect(),
        );
        command.validate()?;
        let namespace = chain_id_to_namespace(self.chain_id);
        let publisher = self.broker.publisher(&namespace).await?;
        publisher.publish(routing::WATCH, &command).await?;
        Ok(())
    }

    /// Publish filters removal command to unwatch contracts.
    pub async fn unregister_contracts(&self, contracts: &[Address]) -> Result<(), ConsumerError> {
        if contracts.is_empty() {
            return Err(ConsumerError::InvalidFilterCommand(
                FilterCommandValidationError::MissingContractAddresses,
            ));
        }
        for contract in contracts {
            self.unregister_filter(&self.create_filter_on_log_address(*contract))
                .await?;
        }
        Ok(())
    }

    /// Publish filters registration command to watch contracts with
    /// finalized-only delivery.
    ///
    /// Events are only delivered once their block is finalized, so the
    /// consumer never sees reorged blocks.
    pub async fn register_final_contracts(
        &self,
        contracts: &[Address],
    ) -> Result<(), ConsumerError> {
        if contracts.is_empty() {
            return Err(ConsumerError::InvalidParameter(
                "contracts array cannot be empty".into(),
            ));
        }
        for contract in contracts {
            self.register_filter(&self.create_final_filter_on_log_address(*contract))
                .await?;
        }
        Ok(())
    }

    /// Publish filters removal command to unwatch finalized-only contracts.
    pub async fn unregister_final_contracts(
        &self,
        contracts: &[Address],
    ) -> Result<(), ConsumerError> {
        if contracts.is_empty() {
            return Err(ConsumerError::InvalidFilterCommand(
                FilterCommandValidationError::MissingContractAddresses,
            ));
        }
        for contract in contracts {
            self.unregister_filter(&self.create_final_filter_on_log_address(*contract))
                .await?;
        }
        Ok(())
    }

    /// Ensure the consumer topology is set up in the broker.
    pub async fn ensure_consumer(&self) -> Result<(), BrokerError> {
        // TODO: start core listener
        self.broker_consumer()?.ensure_topology().await?;
        self.seed_group(&self.consumer_topic()).await
    }

    /// Ensure the catchup consumer topology is set up in the broker.
    pub async fn ensure_catchup_consumer(&self) -> Result<(), BrokerError> {
        self.broker_catchup_consumer()?.ensure_topology().await?;
        self.seed_group(&self.catchup_consumer_topic()).await
    }

    /// Ensure the finalized-only consumer topology is set up in the broker.
    ///
    /// On AMQP this declares the exchanges/queues/bindings up front. On Redis
    /// it creates the stream and its dead-letter companion, so the destination
    /// exists before the listener publishes to it — a publisher that checks
    /// for the stream first would otherwise wait forever for a stream only a
    /// running consumer would create.
    ///
    /// Call this before registering FINAL filters
    /// ([`register_final_contracts`](Self::register_final_contracts) /
    /// [`register_final_full_block`](Self::register_final_full_block)), so no
    /// event is ever published at a destination that does not exist yet.
    pub async fn ensure_final_consumer(&self) -> Result<(), BrokerError> {
        self.broker_final_consumer()?.ensure_topology().await?;
        self.seed_group(&self.final_consumer_topic()).await
    }

    /// Ensure the final catchup consumer topology is set up in the broker.
    ///
    /// On AMQP this declares the exchanges/queues/bindings up front. On Redis
    /// it creates the stream and its dead-letter companion. Call it before
    /// [`request_final_catchup`](Self::request_final_catchup) so replayed
    /// events have somewhere to land.
    pub async fn ensure_final_catchup_consumer(&self) -> Result<(), BrokerError> {
        self.broker_final_catchup_consumer()?
            .ensure_topology()
            .await?;
        self.seed_group(&self.final_catchup_consumer_topic()).await
    }

    /// Create this identity's consumer group on one of its streams, at the
    /// position it should start reading from.
    ///
    /// A consumer group's start position is fixed when the group is created
    /// and never again: once it exists, creating it is a no-op and the
    /// position argument is ignored. So the only chance to get it right is the
    /// first time, and the right answer depends on what is already on the
    /// stream:
    ///
    /// - **This group already exists.** Nothing to decide — it resumes from
    ///   its own cursor.
    /// - **An older group is reading this stream under the same identity.**
    ///   Start where its unfinished work begins. Groups on one stream are
    ///   ordered by the version in their suffix, the unsuffixed group oldest
    ///   of all; the one to inherit from is the newest below our own, because
    ///   an interrupted cutover can leave more than one behind and only the
    ///   latest has a position worth having. A *newer* group is never
    ///   inherited from: during a blue/green the successor reads the same
    ///   stream, and it is not a predecessor.
    ///
    ///   Its cursor records what it was handed, not what it completed, and a
    ///   consumer killed mid-batch leaves the difference outstanding for
    ///   good: nothing reclaims another consumer's pending entries, and the
    ///   host listener takes no part in its own shutdown. Inheriting the
    ///   cursor steps over that work. Starting at the oldest entry the
    ///   predecessor still owes repeats a few blocks into the same database
    ///   instead, which is the cheaper of the two mistakes.
    /// - **Nothing older is reading this stream.** Start at the end. On a
    ///   stream nothing has ever been published to — a fresh identity, and
    ///   the usual case, since the group is created before the filters that
    ///   set the publisher going — the end is also the beginning and nothing
    ///   is skipped. On a populated one the retained entries are of unknown
    ///   age with no cursor to inherit, and the host listener wires no
    ///   catchup flow that would fetch them from the chain instead, so the
    ///   tip is a floor and not a recovery.
    ///
    /// Without this, a group created on a populated stream starts at `0` and
    /// replays everything retained — which, on a stream whose trimming has
    /// been pinned by an abandoned group, is as much history as the stream is
    /// allowed to hold.
    ///
    /// AMQP has no consumer groups and no start positions; this does nothing.
    async fn seed_group(&self, topic: &Topic) -> Result<(), BrokerError> {
        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(()),
        };
        let manager = StreamManager::new((**conn).clone());

        let stream = topic.key();
        let group = self.group_for(topic);
        let existing = manager.list_groups(&stream).await?;

        if existing.iter().any(|candidate| candidate.name == group) {
            return Ok(());
        }

        let start = match predecessor_group(&stream, self.group_suffix.as_deref(), &existing) {
            Some(predecessor) => {
                match manager
                    .oldest_pending_id(&stream, &predecessor.name)
                    .await?
                {
                    Some(oldest) => id_before(&oldest),
                    None => predecessor.last_delivered_id.clone(),
                }
            }
            None => NEW_ENTRIES_ONLY.to_string(),
        };

        manager
            .ensure_consumer_group(&stream, &group, &start)
            .await?;
        Ok(())
    }

    /// The four streams this identity owns: live, catchup, finalized-only and
    /// final catchup.
    fn owned_topics(&self) -> [Topic; 4] {
        [
            self.consumer_topic(),
            self.catchup_consumer_topic(),
            self.final_consumer_topic(),
            self.final_catchup_consumer_topic(),
        ]
    }

    /// Report the consumer groups attached to each of this identity's four
    /// streams, as `(stream key, groups)`.
    ///
    /// This is the read half of retiring a predecessor. A group whose
    /// last-delivered ID does not move between samples while its lag is
    /// non-zero has no live reader: entries are waiting and nobody is taking
    /// them. How long "does not move" must hold before acting is the caller's
    /// policy — this only reports what Redis says.
    ///
    /// Lag is `None` on Redis older than 7.0, which reports no lag field. A
    /// caller that cannot tell how far behind a group is should decline to act
    /// rather than assume zero.
    ///
    /// A stream that does not exist reports no groups. AMQP has no consumer
    /// groups, so it reports nothing at all.
    pub async fn group_status(&self) -> Result<Vec<(String, Vec<GroupStatus>)>, BrokerError> {
        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(Vec::new()),
        };
        let manager = StreamManager::new((**conn).clone());

        let mut statuses = Vec::with_capacity(4);
        for topic in self.owned_topics() {
            let key = topic.key();
            let groups = manager.list_groups(&key).await?;
            statuses.push((key, groups));
        }
        Ok(statuses)
    }

    /// Where each of this identity's four streams was last written to.
    ///
    /// This is how a caller establishes that the publisher has *stopped*
    /// before deleting anything. Unregistering an identity's filters only
    /// queues a command; the row that drives publishing is removed some time
    /// later, and until it is, traffic keeps arriving. Two identical readings
    /// a poll apart mean nothing has been appended in that window.
    ///
    /// Stream length would be the wrong signal — it stays flat on a stream
    /// trimmed as fast as it is written. These positions only move forwards.
    ///
    /// A stream that does not exist reports `None`. AMQP reports nothing.
    pub async fn write_positions(&self) -> Result<Vec<(String, Option<String>)>, BrokerError> {
        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(Vec::new()),
        };
        let manager = StreamManager::new((**conn).clone());

        let mut positions = Vec::with_capacity(4);
        for topic in self.owned_topics() {
            let key = topic.key();
            let last = manager.last_generated_id(&key).await?;
            positions.push((key, last));
        }
        Ok(positions)
    }

    /// Destroy the *unsuffixed* consumer group on each of this identity's four
    /// streams, returning how many were actually destroyed.
    ///
    /// This retires a predecessor that read the same streams under the same
    /// identity but without a group suffix. Group names are derived per
    /// stream, so one logical reader holds four of them and there is no single
    /// name to pass in; the name this removes is exactly the one
    /// [`group_for`](Self::group_for) would produce with no suffix set.
    ///
    /// Destroying a group discards its pending list and its cursor. The stream
    /// and its entries are untouched, which is what makes this safe to run
    /// while another group is still reading the same stream.
    ///
    /// # Errors
    ///
    /// Refuses with [`ConsumerError::InvalidParameter`] when this client has no
    /// group suffix, because then the unsuffixed group *is* this client's own
    /// group and the call would cut the caller's own cursor out from under it.
    pub async fn destroy_unsuffixed_groups(&self) -> Result<usize, ConsumerError> {
        if self.group_suffix.is_none() {
            return Err(ConsumerError::InvalidParameter(
                "refusing to destroy the unsuffixed groups: this client has no group suffix, so \
                 they are its own"
                    .into(),
            ));
        }

        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(0),
        };
        let manager = StreamManager::new((**conn).clone());

        let mut destroyed = 0;
        for topic in self.owned_topics() {
            let group = topic.to_string();
            if manager
                .destroy_group(&topic.key(), &group)
                .await
                .map_err(BrokerError::from)?
            {
                destroyed += 1;
            }
        }
        Ok(destroyed)
    }

    /// Destroy *this client's own* consumer group on each of its four streams,
    /// returning how many were actually destroyed.
    ///
    /// This is how a retired stack gives back what it holds. Once
    /// `versioning.consensus_version` has moved past the version this binary
    /// was built against, the stack writes nothing — guarded transactions are
    /// refused and the consume handler drops what it reads — so its cursor is
    /// pure overhead. Left behind after the pods go, that cursor never moves
    /// again and the trimmer will not reclaim past it.
    ///
    /// Only the groups are removed. The streams, their entries and the groups
    /// belonging to *other* suffixes are untouched, which is what makes this
    /// safe to run while the incoming stack reads the same four streams.
    ///
    /// Idempotent: destroying an absent group is not an error, it just does
    /// not count towards the total.
    ///
    /// # Errors
    ///
    /// Refuses with [`ConsumerError::InvalidParameter`] when this client has
    /// no group suffix. An unsuffixed client's own group is the bare legacy
    /// name, which a predecessor sharing these streams may still be reading —
    /// see [`destroy_unsuffixed_groups`](Self::destroy_unsuffixed_groups),
    /// whose guard this mirrors from the other side.
    pub async fn destroy_own_groups(&self) -> Result<usize, ConsumerError> {
        if self.group_suffix.is_none() {
            return Err(ConsumerError::InvalidParameter(
                "refusing to destroy this client's own groups: without a group suffix they are \
                 the bare legacy names, which a predecessor may still hold"
                    .into(),
            ));
        }

        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(0),
        };
        let manager = StreamManager::new((**conn).clone());

        let mut destroyed = 0;
        for topic in self.owned_topics() {
            if manager
                .destroy_group(&topic.key(), &self.group_for(&topic))
                .await
                .map_err(BrokerError::from)?
            {
                destroyed += 1;
            }
        }
        Ok(destroyed)
    }

    /// Destroy the groups older versions left on this identity's four
    /// streams, returning how many were actually destroyed.
    ///
    /// This is the backstop for [`destroy_own_groups`](Self::destroy_own_groups),
    /// which a retired stack only reaches if it is still running when it
    /// notices the cutover. Whatever it left behind is released here by the
    /// stack that replaced it. See `superseded_groups` for what counts: the
    /// versioned rungs below this client's own, never the unsuffixed one.
    ///
    /// Call this only once this client's version is no longer ahead of the
    /// live one. Before that, the rungs below it include the live stack's own
    /// group, and destroying a group under a reader breaks that reader.
    ///
    /// Idempotent, and a client without a rankable suffix destroys nothing.
    pub async fn destroy_superseded_groups(&self) -> Result<usize, ConsumerError> {
        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(0),
        };
        let manager = StreamManager::new((**conn).clone());

        let mut destroyed = 0;
        for topic in self.owned_topics() {
            let stream = topic.key();
            let existing = manager
                .list_groups(&stream)
                .await
                .map_err(BrokerError::from)?;
            for group in superseded_groups(&stream, self.group_suffix.as_deref(), &existing) {
                if manager
                    .destroy_group(&stream, &group.name)
                    .await
                    .map_err(BrokerError::from)?
                {
                    destroyed += 1;
                }
            }
        }
        Ok(destroyed)
    }

    /// Delete this identity's four streams and their dead-letter companions,
    /// returning how many keys were actually removed.
    ///
    /// Unconditional and destructive: it does not check whether anything is
    /// reading. Establish that the identity is drained first — see
    /// [`group_status`](Self::group_status) — and stop the publisher writing to
    /// it first, via [`unregister_contracts`](Self::unregister_contracts), or
    /// the streams will simply be recreated by the next published event.
    pub async fn delete_streams(&self) -> Result<usize, BrokerError> {
        let conn = match &self.broker {
            Broker::Redis { conn, .. } => conn,
            Broker::Amqp { .. } => return Ok(0),
        };
        let manager = StreamManager::new((**conn).clone());

        let mut deleted = 0;
        for topic in self.owned_topics() {
            for key in [topic.key(), topic.dead_key()] {
                if manager.delete_stream(&key).await? {
                    deleted += 1;
                }
            }
        }
        Ok(deleted)
    }

    /// Start consuming messages with the provided handler function.
    ///
    /// The returned future owns an internal clone of the client, so it can be
    /// spawned without forcing the caller to clone `ListenerConsumer` first.
    pub fn consume<F, Fut>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + 'static
    where
        F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
    {
        let client = self.clone();
        async move {
            let consumer = client.broker_consumer()?;
            let handler = ConsumerHandler {
                call: Arc::new(f),
                cancel: client.live_cancel.clone(),
            };
            consumer.run(handler).await?;
            Ok(())
        }
    }

    /// Start consuming catchup messages with the provided handler function.
    ///
    /// Same shape and ownership as [`consume`](Self::consume); only differs
    /// by subscribing to `{consumer_id}.catchup-event` instead of
    /// `{consumer_id}.new-event`.
    pub fn consume_catchup<F, Fut>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + 'static
    where
        F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
    {
        let client = self.clone();
        async move {
            let consumer = client.broker_catchup_consumer()?;
            let handler = ConsumerHandler {
                call: Arc::new(f),
                cancel: client.catchup_cancel.clone(),
            };
            consumer.run(handler).await?;
            Ok(())
        }
    }

    /// Start consuming finalized-only events with the provided handler
    /// function.
    ///
    /// Subscribes to `{consumer_id}.final-event`, where the listener delivers
    /// [`BlockPayload`]s with `flow == BlockFlow::Final` for the FINAL
    /// watchers registered via
    /// [`register_final_contracts`](Self::register_final_contracts) /
    /// [`register_final_full_block`](Self::register_final_full_block).
    /// Finalized blocks never reorg, so this stream carries no `Reorged`
    /// replays.
    ///
    /// Same shape and ownership as [`consume`](Self::consume): the returned
    /// future owns an internal clone of the client, so it can be spawned
    /// without forcing the caller to clone `ListenerConsumer` first. Stop it
    /// via [`cancel_final`](Self::cancel_final) (or the parent
    /// [`cancel`](Self::cancel)).
    pub fn consume_final<F, Fut>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + 'static
    where
        F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
    {
        let client = self.clone();
        async move {
            let consumer = client.broker_final_consumer()?;
            let handler = ConsumerHandler {
                call: Arc::new(f),
                cancel: client.final_cancel.clone(),
            };
            consumer.run(handler).await?;
            Ok(())
        }
    }

    /// Start consuming final catchup events with the provided handler
    /// function.
    ///
    /// Subscribes to `{consumer_id}.final-catchup-event`, where the listener
    /// delivers [`BlockPayload`]s with `flow == BlockFlow::FinalCatchup` in
    /// response to
    /// [`request_final_catchup`](Self::request_final_catchup).
    ///
    /// Same shape and ownership as [`consume`](Self::consume). Stop it via
    /// [`cancel_final_catchup`](Self::cancel_final_catchup) (or the parent
    /// [`cancel`](Self::cancel)).
    pub fn consume_final_catchup<F, Fut>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + 'static
    where
        F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
    {
        let client = self.clone();
        async move {
            let consumer = client.broker_final_catchup_consumer()?;
            let handler = ConsumerHandler {
                call: Arc::new(f),
                cancel: client.final_catchup_cancel.clone(),
            };
            consumer.run(handler).await?;
            Ok(())
        }
    }
}

struct ConsumerHandler<F> {
    call: Arc<F>,
    cancel: CancellationToken,
}

impl<F> Clone for ConsumerHandler<F> {
    fn clone(&self) -> Self {
        Self {
            call: Arc::clone(&self.call),
            cancel: self.cancel.clone(),
        }
    }
}

#[async_trait]
impl<F, Fut> Handler for ConsumerHandler<F>
where
    F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
{
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let payload: BlockPayload = serde_json::from_slice(&msg.payload)?;
        (self.call)(payload, self.cancel.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use broker::{amqp::RmqPublisher, traits::Publisher};
    use primitives::event::BlockFlow;

    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static TEST_ID: AtomicU64 = AtomicU64::new(0);

    fn unique_name(prefix: &str) -> String {
        format!("{prefix}-{}", TEST_ID.fetch_add(1, Ordering::Relaxed))
    }

    const STREAM: &str = "host-listener-consumer.11155111.new-event";

    fn group(name: &str) -> GroupStatus {
        GroupStatus {
            name: name.to_string(),
            last_delivered_id: "1700000000000-0".to_string(),
            pending: 0,
            lag: Some(0),
        }
    }

    #[test]
    fn the_unsuffixed_group_is_the_oldest_rung() {
        assert_eq!(rung(STREAM, STREAM), Some(0));
    }

    #[test]
    fn a_versioned_suffix_is_its_own_rung() {
        assert_eq!(rung(STREAM, &format!("{STREAM}.v1")), Some(1));
        assert_eq!(rung(STREAM, &format!("{STREAM}.v2")), Some(2));
        assert_eq!(rung(STREAM, &format!("{STREAM}.v17")), Some(17));
    }

    #[test]
    fn names_this_ordering_cannot_rank_are_unranked() {
        // Another stream's group, which `XINFO GROUPS` would never return for
        // this stream but which the ordering should still refuse to rank.
        assert_eq!(rung(STREAM, "somewhere-else.new-event.v2"), None);
        // Suffixes that are not `v<digits>`.
        assert_eq!(rung(STREAM, &format!("{STREAM}.v")), None);
        assert_eq!(rung(STREAM, &format!("{STREAM}.vx")), None);
        assert_eq!(rung(STREAM, &format!("{STREAM}.v2x")), None);
        assert_eq!(rung(STREAM, &format!("{STREAM}.beta")), None);
        // Suffixed, but not with the separator `group_for` writes.
        assert_eq!(rung(STREAM, &format!("{STREAM}v2")), None);
    }

    #[test]
    fn the_newest_group_below_mine_is_the_predecessor() {
        let existing = vec![
            group(STREAM),
            group(&format!("{STREAM}.v1")),
            group(&format!("{STREAM}.v2")),
        ];
        let found = predecessor_group(STREAM, Some("v3"), &existing).unwrap();
        assert_eq!(
            found.name,
            format!("{STREAM}.v2"),
            "an interrupted cutover leaves several behind; only the latest has \
             a position worth having"
        );
    }

    #[test]
    fn a_newer_group_is_never_inherited_from() {
        // The shape of a blue/green: the successor is reading the same stream
        // while this one is being created.
        let existing = vec![group(&format!("{STREAM}.v3"))];
        assert!(predecessor_group(STREAM, Some("v2"), &existing).is_none());
    }

    #[test]
    fn my_own_group_is_not_my_predecessor() {
        let existing = vec![group(&format!("{STREAM}.v2"))];
        assert!(predecessor_group(STREAM, Some("v2"), &existing).is_none());
    }

    #[test]
    fn an_unsuffixed_predecessor_is_still_found() {
        // The rung-0 case, and the only one the matcher handled before.
        let existing = vec![group(STREAM)];
        let found = predecessor_group(STREAM, Some("v2"), &existing).unwrap();
        assert_eq!(found.name, STREAM);
    }

    #[test]
    fn nothing_on_the_stream_means_no_predecessor() {
        assert!(predecessor_group(STREAM, Some("v2"), &[]).is_none());
    }

    fn names(groups: Vec<&GroupStatus>) -> Vec<String> {
        groups.into_iter().map(|group| group.name.clone()).collect()
    }

    #[test]
    fn the_versions_below_mine_are_superseded() {
        let existing = vec![
            group(&format!("{STREAM}.v1")),
            group(&format!("{STREAM}.v2")),
            group(&format!("{STREAM}.v3")),
        ];
        assert_eq!(
            names(superseded_groups(STREAM, Some("v3"), &existing)),
            vec![format!("{STREAM}.v1"), format!("{STREAM}.v2")],
            "an interrupted cutover leaves several behind and all of them go"
        );
    }

    #[test]
    fn the_unsuffixed_group_is_never_superseded() {
        // It predates versioning and cannot see a cutover, so being below the
        // live version says nothing about whether it is still reading.
        let existing = vec![group(STREAM), group(&format!("{STREAM}.v2"))];
        assert!(superseded_groups(STREAM, Some("v2"), &existing).is_empty());
    }

    #[test]
    fn newer_and_unranked_groups_are_never_superseded() {
        let existing = vec![
            group(&format!("{STREAM}.v3")),
            group(&format!("{STREAM}.beta")),
            group("somewhere-else.new-event.v1"),
        ];
        assert!(superseded_groups(STREAM, Some("v2"), &existing).is_empty());
    }

    #[test]
    fn an_unrankable_suffix_supersedes_nothing() {
        let existing = vec![group(&format!("{STREAM}.v1"))];
        assert!(superseded_groups(STREAM, Some("beta"), &existing).is_empty());
        assert!(superseded_groups(STREAM, None, &existing).is_empty());
    }

    #[test]
    fn an_unrankable_suffix_declines_to_guess() {
        let existing = vec![group(STREAM), group(&format!("{STREAM}.v1"))];
        assert!(predecessor_group(STREAM, Some("beta"), &existing).is_none());
        assert!(predecessor_group(STREAM, None, &existing).is_none());
    }

    #[tokio::test]
    #[ignore = "requires Docker"]
    async fn test_consumer_happy_path() {
        let broker_url = "amqp://user:pass@localhost:5672";
        let broker = Broker::amqp(broker_url).build().await.unwrap();
        let chain_id = 1;
        let consumer_id = unique_name("copro-1-host-eth");
        let consumer = ListenerConsumer::new(&broker, chain_id, &consumer_id);
        let contracts = vec![Address::ZERO];
        consumer.register_contracts(&contracts).await.unwrap();
        consumer.ensure_consumer().await.unwrap();
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let consumer_task = consumer.consume(|payload, cancel| async move {
            let v = COUNTER.fetch_add(1, Ordering::Relaxed);
            eprintln!("Received payload: {:?} {v}", payload);
            if v + 1 >= 2 {
                cancel.cancel();
                println!("Cancel after receiving 2 payloads");
            }
            Ok(AckDecision::Ack)
        });
        let consumer_run = tokio::spawn(consumer_task);
        eprintln!("Consumer task spawned, waiting for messages or timeout...");
        let routing_key = consumer.consumer_topic();
        let publisher = RmqPublisher::connect(broker_url, "main").await;
        let fake_block = BlockPayload {
            flow: BlockFlow::Live,
            chain_id,
            block_number: 0,
            block_hash: B256::ZERO,
            parent_hash: B256::ZERO,
            timestamp: 0,
            transactions: vec![],
            catchup_id: None,
        };
        for _ in 1..=2 {
            publisher
                .publish(&routing_key.to_string(), &fake_block)
                .await
                .unwrap();
        }
        let with_timeout =
            tokio::time::timeout(std::time::Duration::from_secs(5), consumer_run).await;
        eprintln!("Consumer task completed or timed out: {with_timeout:?}");
        consumer.cancel();
        consumer.unregister_contracts(&contracts).await.unwrap();
        assert!(
            with_timeout.is_ok(),
            "Consumer should have cancel and not timeout"
        );
        assert_eq!(COUNTER.fetch_add(0, Ordering::Relaxed), 2);
    }

    #[tokio::test]
    #[ignore = "requires Docker"]
    async fn test_consumer_catchup_happy_path() {
        let broker_url = "amqp://user:pass@localhost:5672";
        let broker = Broker::amqp(broker_url).build().await.unwrap();
        let chain_id = 1;
        let consumer_id = unique_name("copro-1-host-eth-catchup");
        let consumer = ListenerConsumer::new(&broker, chain_id, &consumer_id);
        consumer.ensure_catchup_consumer().await.unwrap();
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let consumer_task = consumer.consume_catchup(|payload, cancel| async move {
            let v = COUNTER.fetch_add(1, Ordering::Relaxed);
            eprintln!("Received catchup payload: {:?} {v}", payload);
            if v + 1 >= 2 {
                cancel.cancel();
                println!("Cancel after receiving 2 catchup payloads");
            }
            Ok(AckDecision::Ack)
        });
        let consumer_run = tokio::spawn(consumer_task);
        eprintln!("Catchup consumer task spawned, waiting for messages or timeout...");
        let routing_key = consumer.catchup_consumer_topic();
        assert_eq!(
            routing_key.to_string(),
            format!("{consumer_id}.catchup-event"),
            "catchup_consumer_topic must derive from consumer_catchup_event_routing"
        );
        let publisher = RmqPublisher::connect(broker_url, "main").await;
        let fake_block = BlockPayload {
            flow: BlockFlow::Catchup,
            chain_id,
            block_number: 0,
            block_hash: B256::ZERO,
            parent_hash: B256::ZERO,
            timestamp: 0,
            transactions: vec![],
            catchup_id: None,
        };
        for _ in 1..=2 {
            publisher
                .publish(&routing_key.to_string(), &fake_block)
                .await
                .unwrap();
        }
        let with_timeout =
            tokio::time::timeout(std::time::Duration::from_secs(5), consumer_run).await;
        eprintln!("Catchup consumer task completed or timed out: {with_timeout:?}");
        consumer.cancel();
        assert!(
            with_timeout.is_ok(),
            "Catchup consumer should have cancel and not timeout"
        );
        assert_eq!(COUNTER.fetch_add(0, Ordering::Relaxed), 2);
    }

    #[test]
    fn catchup_consumer_topic_uses_catchup_event_routing() {
        let consumer_id = "copro-1-host-eth";
        let expected = format!("{consumer_id}.{}", routing::CATCHUP_EVENT);
        assert_eq!(
            routing::consumer_catchup_event_routing(consumer_id.into()),
            expected,
        );
        assert_ne!(
            routing::consumer_catchup_event_routing(consumer_id.into()),
            routing::consumer_new_event_routing(consumer_id.into()),
            "catchup and new-event routings must not collide",
        );
    }

    #[test]
    fn final_consumer_topic_uses_final_event_routing() {
        let consumer_id = "copro-1-host-eth";
        let expected = format!("{consumer_id}.{}", routing::FINAL_EVENT);
        assert_eq!(
            routing::consumer_final_event_routing(consumer_id.into()),
            expected,
        );
        assert_ne!(
            routing::consumer_final_event_routing(consumer_id.into()),
            routing::consumer_new_event_routing(consumer_id.into()),
            "final and new-event routings must not collide",
        );
    }

    #[test]
    fn final_catchup_consumer_topic_uses_final_catchup_event_routing() {
        let consumer_id = "copro-1-host-eth";
        let expected = format!("{consumer_id}.{}", routing::FINAL_CATCHUP_EVENT);
        assert_eq!(
            routing::consumer_final_catchup_event_routing(consumer_id.into()),
            expected,
        );
        assert_ne!(
            routing::consumer_final_catchup_event_routing(consumer_id.into()),
            routing::consumer_catchup_event_routing(consumer_id.into()),
            "final-catchup and catchup-event routings must not collide",
        );
        assert_ne!(
            routing::consumer_final_catchup_event_routing(consumer_id.into()),
            routing::consumer_final_event_routing(consumer_id.into()),
            "final-catchup and final-event routings must not collide",
        );
    }

    /// A full-block filter carries no address fields and validates as a
    /// wildcard subscription tied to this consumer's id. `build()` does not
    /// open a connection, so this runs without Docker.
    #[tokio::test]
    async fn create_full_block_filter_is_wildcard_and_valid() {
        let broker = Broker::amqp("amqp://user:pass@localhost:5672")
            .build()
            .await
            .unwrap();
        let consumer = ListenerConsumer::new(&broker, 1, "copro-1-host-eth");

        let mut cmd = consumer.create_full_block_filter();
        assert_eq!(cmd.consumer_id, "copro-1-host-eth");
        assert!(cmd.from.is_none());
        assert!(cmd.to.is_none());
        assert!(cmd.log_address.is_none());
        assert!(cmd.filter_type.is_none(), "live filters carry no type");
        cmd.validate().expect("full-block filter must validate");
    }

    /// Final filter builders mark the command as finalized-only and keep the
    /// same address semantics as their live counterparts. `build()` does not
    /// open a connection, so this runs without Docker.
    #[tokio::test]
    async fn create_final_filters_carry_final_type_and_validate() {
        let broker = Broker::amqp("amqp://user:pass@localhost:5672")
            .build()
            .await
            .unwrap();
        let consumer = ListenerConsumer::new(&broker, 1, "copro-1-host-eth");

        let mut cmd = consumer.create_final_full_block_filter();
        assert_eq!(cmd.consumer_id, "copro-1-host-eth");
        assert!(cmd.from.is_none());
        assert!(cmd.to.is_none());
        assert!(cmd.log_address.is_none());
        assert_eq!(cmd.filter_type, Some(FilterType::Final));
        cmd.validate()
            .expect("final full-block filter must validate");

        let contract: Address = "0x00000000000000000000000000000000deadbeef"
            .parse()
            .unwrap();
        let mut cmd = consumer.create_final_filter_on_log_address(contract);
        assert_eq!(cmd.log_address, Some(contract));
        assert_eq!(cmd.filter_type, Some(FilterType::Final));
        cmd.validate()
            .expect("final log-address filter must validate");
    }

    /// Locks in the parent/child cancellation contract that `ListenerConsumer`
    /// relies on. If `tokio_util` ever changes child-token semantics, this
    /// test fails before any consumer-lib regression reaches a user.
    #[test]
    fn parent_child_cancellation_semantics() {
        let parent = CancellationToken::new();
        let live = parent.child_token();
        let catchup = parent.child_token();
        let final_flow = parent.child_token();
        let final_catchup = parent.child_token();

        // Cancelling a child must not propagate to siblings or to the parent.
        live.cancel();
        assert!(live.is_cancelled());
        assert!(!catchup.is_cancelled(), "live cancel must not stop catchup");
        assert!(
            !final_flow.is_cancelled(),
            "live cancel must not stop the final flow"
        );
        assert!(
            !final_catchup.is_cancelled(),
            "live cancel must not stop final catchup"
        );
        assert!(!parent.is_cancelled(), "child cancel must not stop parent");

        // A final-flow child cancel is equally isolated.
        final_flow.cancel();
        assert!(final_flow.is_cancelled());
        assert!(
            !final_catchup.is_cancelled(),
            "final cancel must not stop final catchup"
        );
        assert!(!parent.is_cancelled(), "child cancel must not stop parent");

        // Cancelling the parent must cascade to every remaining child.
        parent.cancel();
        assert!(parent.is_cancelled());
        assert!(catchup.is_cancelled(), "parent cancel must stop catchup");
        assert!(
            final_catchup.is_cancelled(),
            "parent cancel must stop final catchup"
        );
    }
}
