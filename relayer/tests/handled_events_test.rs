// Cursor discipline of the gateway event registry, against a real Postgres schema and a
// real orchestrator.
//
// The claim rules and the prefix bookkeeping are unit-tested inside the module; what needs
// a database and a dispatcher is the writing half: the cursor moves only once handlers have
// returned, it moves past a handler that panicked, and it stays put while one is stuck -
// which is what makes a hard kill replay the events instead of losing them.
//
// Not covered here, because it needs the RPC mock to filter `eth_getLogs` by block range:
// the same guarantee end to end, with a polling listener re-reading the chain after a kill.

mod common;

use common::test_schema::TestSchema;
use common::utils::TEST_CONFIG_PATH;

use alloy::primitives::{FixedBytes, TxHash};
use alloy::rpc::types::Log;
use anyhow::Context;
use async_trait::async_trait;
use fhevm_relayer::config::settings::{DispatcherLockConfig, Settings};
use fhevm_relayer::core::event::{GatewayChainEventData, GatewayChainEventId, RelayerEvent};
use fhevm_relayer::gateway::handled_events::{
    EventKey, HandledEvents, ObservedEvent, RangeObserved,
};
use fhevm_relayer::orchestrator::traits::EventHandler;
use fhevm_relayer::orchestrator::{DispatcherLock, LockState, Orchestrator, TokioEventDispatcher};
use fhevm_relayer::store::sql::repositories::Repositories;
use prometheus::Registry;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

const INSTANCE_ID: usize = 0;
const DEDUP_TTL: Duration = Duration::from_secs(60);

/// A handler that counts what it is given, optionally waits for a permit before finishing,
/// and optionally panics once it has.
struct TestHandler {
    entered: Arc<AtomicUsize>,
    handled: Arc<AtomicUsize>,
    gate: Option<Arc<Semaphore>>,
    panics: bool,
}

#[async_trait]
impl EventHandler<RelayerEvent> for TestHandler {
    async fn handle_event(&self, _event: RelayerEvent) {
        self.entered.fetch_add(1, Ordering::SeqCst);

        if let Some(gate) = &self.gate {
            let permit = gate.acquire().await.expect("gate open");
            permit.forget();
        }

        self.handled.fetch_add(1, Ordering::SeqCst);

        if self.panics {
            panic!("handler exploded");
        }
    }
}

/// An isolated schema, the component writing the cursor into it, and the handler it
/// dispatches to.
struct Harness {
    repositories: Repositories,
    schema: TestSchema,
    handled_events: Arc<HandledEvents>,
    entered: Arc<AtomicUsize>,
    handled: Arc<AtomicUsize>,
}

impl Harness {
    async fn new(gate: Option<Arc<Semaphore>>, panics: bool) -> anyhow::Result<Self> {
        // The repositories report query latency, and the metric handle is a OnceLock that
        // the relayer's startup would normally fill.
        let settings = Settings::new(Some(TEST_CONFIG_PATH.to_string()))?;
        fhevm_relayer::metrics::init_db_metrics(&Registry::new(), settings.metrics.clone());

        let schema = TestSchema::new().await?;

        let mut storage = settings.storage.clone();
        storage.sql_database_url = schema.database_url();
        storage.app_pool.max_connections = 2;
        storage.app_pool.min_connections = 0;
        storage.cron_pool.max_connections = 1;
        storage.cron_pool.min_connections = 0;

        // Real, held lock rather than a throwaway unrun one - the cursor tests below exercise
        // `ChainCursorRepository::advance`'s epoch fence, so they need a live epoch behind it,
        // matching production wiring. Left running for the test process's lifetime; nextest
        // gives each test its own process, so there is nothing to release.
        let dispatcher_lock =
            DispatcherLock::connect(&DispatcherLockConfig::default(), &storage.sql_database_url)
                .await?;
        {
            let lock = dispatcher_lock.clone();
            tokio::spawn(async move { lock.run(CancellationToken::new()).await });
        }
        // Bounded, not an unconditional poll loop: an unacquirable lock (a schema/connection
        // problem) should fail this test, not hang the whole CI run.
        tokio::time::timeout(Duration::from_secs(5), async {
            while dispatcher_lock.state() != LockState::Held {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .context("dispatcher lock did not reach Held within budget")?;

        let repositories = Repositories::new(storage, dispatcher_lock).await?;

        let tracker = TaskTracker::new();
        let orchestrator = Orchestrator::new(
            Arc::new(TokioEventDispatcher::new(tracker.clone())),
            tracker,
        );

        let entered = Arc::new(AtomicUsize::new(0));
        let handled = Arc::new(AtomicUsize::new(0));
        orchestrator.register_handler(
            &[GatewayChainEventId::PublicDecryptionResponse.into()],
            Arc::new(TestHandler {
                entered: entered.clone(),
                handled: handled.clone(),
                gate,
                panics,
            }),
        );

        let handled_events = Arc::new(HandledEvents::new(
            DEDUP_TTL,
            1_000,
            repositories.chain_cursor.clone(),
            orchestrator,
        ));

        Ok(Self {
            repositories,
            schema,
            handled_events,
            entered,
            handled,
        })
    }

    async fn cursor(&self) -> u64 {
        self.repositories
            .chain_cursor
            .get()
            .await
            .expect("read cursor")
            .unwrap_or(0)
    }

    /// Wait for the cursor to reach `block_number`, reporting where it got to.
    async fn await_cursor(&self, block_number: u64) {
        if !await_until(|| async { self.cursor().await >= block_number }).await {
            panic!(
                "cursor stopped at {} instead of reaching {block_number}",
                self.cursor().await
            );
        }
    }

    /// Wait for `count` events to have been handled.
    async fn await_handled(&self, count: usize) {
        let handled = self.handled.clone();
        if !await_until(|| async { handled.load(Ordering::SeqCst) >= count }).await {
            panic!(
                "{} events handled instead of {count}",
                handled.load(Ordering::SeqCst)
            );
        }
    }

    /// Wait for `count` events to have reached the handler. Behind a gate that is the moment
    /// an event is provably in flight and provably unfinished.
    async fn await_entered(&self, count: usize) {
        let entered = self.entered.clone();
        if !await_until(|| async { entered.load(Ordering::SeqCst) >= count }).await {
            panic!(
                "{} events reached the handler instead of {count}",
                entered.load(Ordering::SeqCst)
            );
        }
    }

    async fn shutdown(mut self) {
        self.repositories.close_pools().await;
        if let Err(e) = self.schema.cleanup().await {
            eprintln!("Failed to clean up test schema: {e}");
        }
    }
}

/// Poll `condition` for up to five seconds, reporting whether it came true.
///
/// The budget asserts nothing about latency - it only keeps a condition that will never come
/// true from hanging CI, so it is sized for the slowest passing run: every poll is a query,
/// and the suite's threads share one Postgres.
async fn await_until<F, Fut>(condition: F) -> bool
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    for _ in 0..500 {
        if condition().await {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    false
}

fn event(block_number: u64, log_index: u64) -> ObservedEvent {
    ObservedEvent {
        key: EventKey {
            block_number,
            block_hash: FixedBytes::with_last_byte(block_number as u8),
            log_index,
        },
        event: GatewayChainEventData::PublicDecryptionResponse {
            log: Log::default(),
            tx_hash: TxHash::with_last_byte(7),
        },
    }
}

fn range(to_block: u64) -> Option<RangeObserved> {
    Some(RangeObserved { to_block })
}

/// The cursor is what a restart resumes from, so it may only pass an event whose handlers
/// have finished with it.
#[tokio::test]
async fn test_cursor_advances_once_the_handlers_finish() {
    let harness = Harness::new(None, false).await.expect("harness");

    harness
        .handled_events
        .record_and_dispatch(vec![event(101, 0)], range(105), INSTANCE_ID)
        .await;

    harness.await_cursor(105).await;
    assert_eq!(harness.handled.load(Ordering::SeqCst), 1);

    harness.shutdown().await;
}

/// A range with nothing in it is complete on arrival: there is no event to wait for, and
/// holding the cursor back would make every quiet poll a step towards replaying the chain.
#[tokio::test]
async fn test_an_empty_range_advances_the_cursor() {
    let harness = Harness::new(None, false).await.expect("harness");

    harness
        .handled_events
        .record_and_dispatch(Vec::new(), range(110), INSTANCE_ID)
        .await;

    harness.await_cursor(110).await;
    assert_eq!(harness.handled.load(Ordering::SeqCst), 0);

    harness.shutdown().await;
}

/// A stuck handler pins the cursor to the last block that finished, so a kill in that state
/// re-reads the unfinished range. Once it returns, the whole completed prefix collapses into
/// one cursor write.
#[tokio::test]
async fn test_a_stalled_handler_holds_the_cursor_until_it_returns() {
    let gate = Arc::new(Semaphore::new(0));
    let harness = Harness::new(Some(gate.clone()), false)
        .await
        .expect("harness");

    harness
        .handled_events
        .record_and_dispatch(Vec::new(), range(120), INSTANCE_ID)
        .await;
    harness.await_cursor(120).await;

    harness
        .handled_events
        .record_and_dispatch(vec![event(121, 0)], range(125), INSTANCE_ID)
        .await;
    harness
        .handled_events
        .record_and_dispatch(Vec::new(), range(130), INSTANCE_ID)
        .await;

    // Waited on rather than slept through: on a loaded machine a sleep can expire before
    // block 121 is dispatched at all, where the cursor still reads 120 and this passes vacuously.
    harness.await_entered(1).await;
    assert_eq!(
        harness.cursor().await,
        120,
        "a later range must not carry the cursor past an unfinished one"
    );

    gate.add_permits(1);

    harness.await_cursor(130).await;
    assert_eq!(harness.handled.load(Ordering::SeqCst), 1);

    harness.shutdown().await;
}

/// A handler that panics has still had its turn. Handlers report nothing back, so there is
/// nothing to retry against, and holding the cursor for one dead event would stall recovery
/// for every event after it.
#[tokio::test]
async fn test_the_cursor_advances_past_a_panicking_handler() {
    let harness = Harness::new(None, true).await.expect("harness");

    harness
        .handled_events
        .record_and_dispatch(vec![event(131, 0)], range(135), INSTANCE_ID)
        .await;

    harness.await_cursor(135).await;
    assert_eq!(harness.handled.load(Ordering::SeqCst), 1);

    harness.shutdown().await;
}

/// The same log reaching two listener instances is dispatched once. The second instance
/// carries a range, so it may not simply drop the event - it dispatches again to be able to
/// attest its own range - but a completed event is skipped by everyone.
#[tokio::test]
async fn test_a_completed_event_is_skipped_by_the_next_instance() {
    let harness = Harness::new(None, false).await.expect("harness");

    harness
        .handled_events
        .record_and_dispatch(vec![event(141, 0)], None, INSTANCE_ID)
        .await;

    harness.await_handled(1).await;

    harness
        .handled_events
        .record_and_dispatch(vec![event(141, 0)], range(145), INSTANCE_ID)
        .await;

    harness.await_cursor(145).await;
    assert_eq!(
        harness.handled.load(Ordering::SeqCst),
        1,
        "the duplicate was skipped rather than handled again"
    );

    harness.shutdown().await;
}

/// A listener that completes a range late must not pull the position back to where its own
/// range ended. The statement that writes the row is what refuses it.
#[tokio::test]
async fn test_the_cursor_never_moves_backwards() {
    let harness = Harness::new(None, false).await.expect("harness");

    harness
        .handled_events
        .record_and_dispatch(Vec::new(), range(200), INSTANCE_ID)
        .await;
    harness.await_cursor(200).await;

    harness
        .handled_events
        .record_and_dispatch(Vec::new(), range(150), 1)
        .await;

    assert_eq!(
        harness.cursor().await,
        200,
        "a lower block leaves the recorded position alone"
    );

    harness.shutdown().await;
}
