//! Durable catchup bookkeeping — the consumer-side half of the catchup
//! lifecycle contract.
//!
//! The listener is a pure executor. It runs exactly the requests and cancels
//! it is given, and it never retires a catchup on a consumer's behalf. Two
//! obligations fall to the consumer as a result, and nothing server-side
//! enforces either one:
//!
//! 1. **Own the id.** A `catchup_id` names a request for its whole life. Lose
//!    it and the request can no longer be cancelled — it runs to completion
//!    regardless of what happens to this process.
//! 2. **Keep at most one active request per flow.** A consumer that mints a
//!    fresh id every boot without retiring the previous one leaves a pile of
//!    catchups behind, each of which fans out and competes for the same
//!    workers.
//!
//! Both follow from a single rule:
//!
//! > **The desired range must be a function of durable state, not of the
//! > current chain head.**
//!
//! A range derived from the head is a different range on every boot, so it
//! mints a different id on every boot. Under this design a crash-looping
//! application written that way would cancel and re-request itself forever
//! and never finish a backfill.
//!
//! [`reconcile`] implements the resulting boot protocol. It is written to be
//! copied.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use consumer::ListenerConsumer;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, watch};
use tracing::info;
use uuid::Uuid;

/// Which catchup flow a piece of state belongs to.
///
/// The two flows are independent: each has its own id, its own range, and its
/// own control queues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Catchup,
    FinalCatchup,
}

impl Flow {
    /// Log tag, matching the `LIVE-CATCHUP` / `FINAL-CATCHUP` tags the rest of
    /// the example uses.
    fn tag(self) -> &'static str {
        match self {
            Flow::Catchup => "LIVE-CATCHUP",
            Flow::FinalCatchup => "FINAL-CATCHUP",
        }
    }

    /// Prefix of the `*_START` / `*_END` environment overrides.
    fn env_prefix(self) -> &'static str {
        match self {
            Flow::Catchup => "CATCHUP",
            Flow::FinalCatchup => "FINAL_CATCHUP",
        }
    }
}

/// An inclusive block range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub block_start: u64,
    pub block_end: u64,
}

/// A catchup request this consumer owns: the id, and the range it was issued
/// for. The range is stored so a later boot can tell "same request" from
/// "the configuration changed" without asking the chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatchupRecord {
    pub id: Uuid,
    pub block_start: u64,
    pub block_end: u64,
}

impl CatchupRecord {
    fn covers(&self, range: Range) -> bool {
        self.block_start == range.block_start && self.block_end == range.block_end
    }
}

/// Per-flow durable state.
///
/// `retiring` is the crash-safety field. It is written *before* the cancel
/// that retires an old request and cleared *after*, so a record that still
/// carries one on boot means the swap was interrupted and must be finished.
///
/// `cancelled` is the tombstone. Without it "no request on record" would be
/// ambiguous between *never asked* and *asked, then cancelled* — and the
/// first of those derives a fresh range from the head, so a consumer would
/// silently un-cancel itself on the next restart. It holds the record that
/// was retired, not just a flag, so a later boot can tell an operator who
/// left the old configuration in place from one who asked for something new.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct FlowState {
    #[serde(default)]
    retiring: Option<Uuid>,
    #[serde(default)]
    current: Option<CatchupRecord>,
    #[serde(default)]
    cancelled: Option<CatchupRecord>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct StateFile {
    #[serde(default)]
    catchup: FlowState,
    #[serde(default)]
    final_catchup: FlowState,
}

impl StateFile {
    fn flow(&self, flow: Flow) -> &FlowState {
        match flow {
            Flow::Catchup => &self.catchup,
            Flow::FinalCatchup => &self.final_catchup,
        }
    }

    fn flow_mut(&mut self, flow: Flow) -> &mut FlowState {
        match flow {
            Flow::Catchup => &mut self.catchup,
            Flow::FinalCatchup => &mut self.final_catchup,
        }
    }
}

/// A JSON file holding the catchup state for both flows.
///
/// A real consumer would keep this next to whatever else it persists — the
/// same transaction as its block watermark, ideally. A file is used here so
/// the example stays dependency-free.
#[derive(Debug, Clone)]
pub struct Store {
    path: PathBuf,
    /// Serializes the read-modify-write cycle in [`reconcile`].
    ///
    /// Both flows reconcile concurrently at boot, and the control plane can
    /// reconcile either of them at any moment. Every one of those paths reads
    /// the whole file, mutates one flow's field, and writes the whole file
    /// back — so two overlapping cycles silently discard whichever update
    /// landed first.
    ///
    /// That is not a cosmetic loss. The discarded update is a `current`
    /// record, and a `current` that exists in the listener but not on disk is
    /// exactly the orphaned, uncancellable catchup this module exists to
    /// prevent. Cloning a `Store` shares the lock, which is what makes the
    /// clone handed to the control plane safe.
    ///
    /// Note this guards one process. Replicas sharing a `consumer_id` must
    /// share durable state or be a singleton — see the module docs.
    lock: Arc<Mutex<()>>,
}

impl Store {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            lock: Arc::new(Mutex::new(())),
        }
    }

    async fn load(&self) -> anyhow::Result<StateFile> {
        match tokio::fs::read(&self.path).await {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            // No file yet: first boot.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(StateFile::default()),
            Err(e) => Err(e.into()),
        }
    }

    /// Write via a temporary file and rename.
    ///
    /// A torn write here loses a `catchup_id`, and a lost id is a catchup that
    /// can never be cancelled. Rename is atomic within a directory, so a
    /// reader sees either the old state or the new one.
    async fn save(&self, state: &StateFile) -> anyhow::Result<()> {
        let tmp = self.path.with_extension("json.tmp");
        tokio::fs::write(&tmp, serde_json::to_vec_pretty(state)?).await?;
        tokio::fs::rename(&tmp, &self.path).await?;
        Ok(())
    }

    /// The id currently on record for `flow`, if any.
    ///
    /// Event handlers use this to drop blocks belonging to a catchup that has
    /// since been retired — see [`crate::live_events`].
    pub async fn current_id(&self, flow: Flow) -> anyhow::Result<Option<Uuid>> {
        Ok(self.load().await?.flow(flow).current.as_ref().map(|r| r.id))
    }

    /// The id in the cancel tombstone for `flow`, if any.
    ///
    /// Reading the tombstone rather than remembering what was current before a
    /// cancel reports what was actually persisted: a cancel that found nothing
    /// to retire leaves this `None`, and says so.
    pub async fn cancelled_id(&self, flow: Flow) -> anyhow::Result<Option<Uuid>> {
        Ok(self
            .load()
            .await?
            .flow(flow)
            .cancelled
            .as_ref()
            .map(|r| r.id))
    }
}

/// Resolve the range this consumer wants replayed, in priority order:
///
/// 1. Explicit configuration (`{PREFIX}_START` / `{PREFIX}_END`) — an operator
///    asking for a specific backfill.
/// 2. The range already on record — a request was issued for it, so keeping it
///    is what holds the id stable across restarts.
/// 3. Derived from the live head, once, on the very first boot.
///
/// The ordering is the point. The head is consulted **last**, and only when
/// there is no durable answer. Consulting it first — `head - DEPTH`, the
/// obvious thing to write — yields a different range every boot and so a
/// different id every boot.
///
/// `None` means *no catchup is wanted*, which is distinct from "replay
/// nothing yet": it is the answer after an explicit cancel. A cancel outlives
/// the process that issued it, so it has to survive a restart, and the only
/// thing that clears it is a **new** intent — a configured range different
/// from the one that was cancelled, or a fresh request through
/// [`reconcile`]. Leaving the old configuration in place does not resurrect a
/// backfill an operator deliberately stopped.
pub async fn desired_range(
    store: &Store,
    flow: Flow,
    head: u64,
    depth: u64,
) -> anyhow::Result<Option<Range>> {
    let state = store.load().await?;
    Ok(resolve(state.flow(flow), range_from_env(flow), head, depth))
}

/// The decision half of [`desired_range`], with the two inputs it reads from
/// the outside world — durable state and configuration — passed in.
///
/// Split out because the precedence rules are the part worth testing, and
/// reading `{PREFIX}_START` from the process environment inside a test makes
/// that test race every other test in the binary.
fn resolve(state: &FlowState, configured: Option<Range>, head: u64, depth: u64) -> Option<Range> {
    if let Some(range) = configured {
        return match &state.cancelled {
            // The same range that was cancelled: the configuration is stale,
            // not a new instruction. Staying cancelled is the safe reading —
            // the alternative resurrects a backfill on every restart.
            Some(cancelled) if cancelled.covers(range) => None,
            _ => Some(range),
        };
    }

    // Cancelled, and nothing configured says otherwise.
    if state.cancelled.is_some() {
        return None;
    }

    if let Some(record) = &state.current {
        return Some(Range {
            block_start: record.block_start,
            block_end: record.block_end,
        });
    }

    Some(Range {
        block_start: head.saturating_sub(depth),
        block_end: head,
    })
}

fn range_from_env(flow: Flow) -> Option<Range> {
    let prefix = flow.env_prefix();
    let start = std::env::var(format!("{prefix}_START"))
        .ok()?
        .parse()
        .ok()?;
    let end = std::env::var(format!("{prefix}_END")).ok()?.parse().ok()?;
    Some(Range {
        block_start: start,
        block_end: end,
    })
}

/// Bring the listener in line with `desired`, and record what was done.
///
/// `desired` is `None` when no catchup is wanted — an explicit cancel. This
/// is the same entry point the boot path uses, deliberately: a runtime cancel
/// and a restart-time cancel must not be two different implementations, or
/// they will drift and only one of them will be the tested one.
///
/// The protocol, and why each step is ordered the way it is:
///
/// - **Finish an interrupted retire first.** A `retiring` id on disk means a
///   previous boot persisted a swap but may not have completed the cancel.
///   Cancelling again is a no-op, so replaying it costs nothing and closes the
///   window where an old catchup keeps running unnoticed.
/// - **Re-send an unchanged request.** The listener records that it has fanned
///   a request out, so repeating one is a true no-op rather than a second
///   fan-out. That is what makes "persist, then call" safe: if the process died
///   before the call, this boot lands it; if it died after, this boot changes
///   nothing.
/// - **Persist before calling, always.** Both `request` and `cancel` are
///   idempotent, so the durable record does not need to be atomic with them —
///   it only needs to be written first, so that a crash leaves a replayable
///   note rather than an orphan.
/// - **A new intent clears the tombstone.** Asking for a range is how a
///   cancelled flow is restarted; nothing else un-cancels it.
pub async fn reconcile(
    consumer: &ListenerConsumer,
    store: &Store,
    flow: Flow,
    desired: Option<Range>,
) -> anyhow::Result<()> {
    // Held for the whole function, not just the load: every branch below is a
    // read-modify-write of the shared file, and releasing between the read and
    // the write is what loses an id.
    let _guard = store.lock.lock().await;

    let mut state = store.load().await?;

    // Step 1 — finish an interrupted retire.
    if let Some(retiring) = state.flow(flow).retiring {
        info!(flow = flow.tag(), catchup_id = %retiring,
            "resuming interrupted retire from a previous boot");
        cancel(consumer, flow, retiring).await?;
        state.flow_mut(flow).retiring = None;
        store.save(&state).await?;
    }

    match (desired, state.flow(flow).current.clone()) {
        // Step 2a — on record and unchanged. Re-sending is a no-op.
        (Some(desired), Some(record)) if record.covers(desired) => {
            info!(flow = flow.tag(), catchup_id = %record.id,
                block_start = record.block_start, block_end = record.block_end,
                "re-sending the catchup already on record (no-op if it already ran)");
            request(consumer, flow, &record).await?;
        }

        // Step 2b — the desired range changed. Retire the old request and
        // mint a replacement.
        (Some(desired), Some(record)) => {
            let replacement = CatchupRecord {
                id: Uuid::now_v7(),
                block_start: desired.block_start,
                block_end: desired.block_end,
            };
            info!(flow = flow.tag(), retiring = %record.id, catchup_id = %replacement.id,
                block_start = replacement.block_start, block_end = replacement.block_end,
                "desired range changed — retiring the current catchup");

            let slot = state.flow_mut(flow);
            slot.retiring = Some(record.id);
            slot.current = Some(replacement.clone());
            slot.cancelled = None;
            store.save(&state).await?;

            cancel(consumer, flow, record.id).await?;
            request(consumer, flow, &replacement).await?;

            state.flow_mut(flow).retiring = None;
            store.save(&state).await?;
        }

        // Step 2c — nothing on record. Mint, persist, then request.
        (Some(desired), None) => {
            let record = CatchupRecord {
                id: Uuid::now_v7(),
                block_start: desired.block_start,
                block_end: desired.block_end,
            };
            info!(flow = flow.tag(), catchup_id = %record.id,
                block_start = record.block_start, block_end = record.block_end,
                "requesting the first catchup for this flow");

            let slot = state.flow_mut(flow);
            slot.current = Some(record.clone());
            slot.cancelled = None;
            store.save(&state).await?;
            request(consumer, flow, &record).await?;
        }

        // Step 2d — no catchup wanted, and one is on record. Retire it and
        // leave a tombstone so the next boot does not resurrect it.
        //
        // `retiring` is set here for the same reason as in 2b: if the process
        // dies between the save and the cancel, the next boot replays it.
        (None, Some(record)) => {
            info!(flow = flow.tag(), catchup_id = %record.id,
                "cancel requested — retiring the current catchup");

            let slot = state.flow_mut(flow);
            slot.retiring = Some(record.id);
            slot.current = None;
            slot.cancelled = Some(record.clone());
            store.save(&state).await?;

            cancel(consumer, flow, record.id).await?;

            state.flow_mut(flow).retiring = None;
            store.save(&state).await?;
        }

        // Step 2e — nothing wanted, nothing owned.
        (None, None) => {}
    }

    Ok(())
}

/// [`reconcile`], plus publishing the resulting id to the flow's event
/// handler.
///
/// The two halves belong together. The handler drops blocks stamped with an
/// id other than the one it is told to expect, so a reconcile whose result is
/// never published leaves the handler accepting blocks from a catchup that
/// was just retired — silently, and only until the next restart. Every caller
/// that changes the desired state should come through here.
pub async fn reconcile_and_track(
    consumer: &ListenerConsumer,
    store: &Store,
    flow: Flow,
    desired: Option<Range>,
    active: &watch::Sender<Option<Uuid>>,
) -> anyhow::Result<()> {
    reconcile(consumer, store, flow, desired).await?;
    let _ = active.send(store.current_id(flow).await?);
    Ok(())
}

async fn request(
    consumer: &ListenerConsumer,
    flow: Flow,
    record: &CatchupRecord,
) -> anyhow::Result<()> {
    match flow {
        Flow::Catchup => {
            consumer
                .request_catchup(record.id, record.block_start, record.block_end)
                .await?
        }
        Flow::FinalCatchup => {
            consumer
                .request_final_catchup(record.id, record.block_start, record.block_end)
                .await?
        }
    }
    Ok(())
}

async fn cancel(consumer: &ListenerConsumer, flow: Flow, id: Uuid) -> anyhow::Result<()> {
    match flow {
        Flow::Catchup => consumer.cancel_catchup_request(id).await?,
        Flow::FinalCatchup => consumer.cancel_final_catchup_request(id).await?,
    }
    Ok(())
}

/// Default location of the state file, overridable with `CATCHUP_STATE_PATH`.
pub fn default_path() -> PathBuf {
    std::env::var("CATCHUP_STATE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new("./catchup-state.json").to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store(dir: &tempfile::TempDir) -> Store {
        Store::new(dir.path().join("catchup-state.json"))
    }

    fn range(block_start: u64, block_end: u64) -> Range {
        Range {
            block_start,
            block_end,
        }
    }

    fn record(id: Uuid, block_start: u64, block_end: u64) -> CatchupRecord {
        CatchupRecord {
            id,
            block_start,
            block_end,
        }
    }

    #[tokio::test]
    async fn desired_range_falls_back_to_head_only_on_first_boot() {
        // First boot: nothing on record, so the head is the only answer
        // available.
        let dir = tempfile::tempdir().unwrap();
        let store = store(&dir);
        let first = desired_range(&store, Flow::Catchup, 1_000, 100)
            .await
            .unwrap();
        assert_eq!(first, Some(range(900, 1_000)));

        // Record that range, then ask again with a head that has moved on.
        // The persisted range must win — this is the property that keeps the
        // id stable, and the one `head - DEPTH` breaks.
        let mut state = StateFile::default();
        state.catchup.current = Some(record(Uuid::now_v7(), 900, 1_000));
        store.save(&state).await.unwrap();

        let second = desired_range(&store, Flow::Catchup, 5_000, 100)
            .await
            .unwrap();
        assert_eq!(second, first);
    }

    #[test]
    fn an_unconfigured_flow_stays_cancelled_across_a_restart() {
        // The whole point of the tombstone: without it this returns
        // `head - depth` and the consumer un-cancels itself on every boot.
        let state = FlowState {
            cancelled: Some(record(Uuid::now_v7(), 900, 1_000)),
            ..Default::default()
        };
        assert_eq!(resolve(&state, None, 5_000, 100), None);
    }

    #[test]
    fn a_stale_configured_range_does_not_resurrect_a_cancelled_catchup() {
        // An operator cancelled at runtime but left CATCHUP_START/END in the
        // deployment. That is not a new instruction.
        let state = FlowState {
            cancelled: Some(record(Uuid::now_v7(), 900, 1_000)),
            ..Default::default()
        };
        assert_eq!(resolve(&state, Some(range(900, 1_000)), 5_000, 100), None);
    }

    #[test]
    fn a_changed_configured_range_overrides_the_tombstone() {
        // A different range *is* a new instruction — otherwise a cancel would
        // be unrecoverable without deleting the state file.
        let state = FlowState {
            cancelled: Some(record(Uuid::now_v7(), 900, 1_000)),
            ..Default::default()
        };
        assert_eq!(
            resolve(&state, Some(range(1, 500)), 5_000, 100),
            Some(range(1, 500))
        );
    }

    #[test]
    fn a_live_record_still_wins_over_the_head() {
        let state = FlowState {
            current: Some(record(Uuid::now_v7(), 900, 1_000)),
            ..Default::default()
        };
        assert_eq!(resolve(&state, None, 5_000, 100), Some(range(900, 1_000)));
    }

    #[tokio::test]
    async fn flows_keep_independent_state() {
        let dir = tempfile::tempdir().unwrap();
        let store = store(&dir);

        let mut state = StateFile::default();
        state.catchup.current = Some(CatchupRecord {
            id: Uuid::now_v7(),
            block_start: 1,
            block_end: 2,
        });
        store.save(&state).await.unwrap();

        assert!(store.current_id(Flow::Catchup).await.unwrap().is_some());
        assert!(
            store
                .current_id(Flow::FinalCatchup)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn the_tombstone_names_the_id_a_cancel_retired() {
        // What `POST /catchup/cancel` answers with. `current_id` is `None`
        // after a cancel by construction, so reporting it tells an operator
        // nothing about *which* catchup they just stopped.
        let dir = tempfile::tempdir().unwrap();
        let store = store(&dir);
        let retired = Uuid::now_v7();

        assert_eq!(store.cancelled_id(Flow::Catchup).await.unwrap(), None);

        let mut state = StateFile::default();
        state.catchup.cancelled = Some(record(retired, 1, 500));
        store.save(&state).await.unwrap();

        assert_eq!(
            store.cancelled_id(Flow::Catchup).await.unwrap(),
            Some(retired)
        );
        // Per-flow, like every other field on the state file.
        assert_eq!(store.cancelled_id(Flow::FinalCatchup).await.unwrap(), None);
    }

    #[tokio::test]
    async fn state_file_survives_a_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let store = store(&dir);

        let id = Uuid::now_v7();
        let mut state = StateFile::default();
        state.final_catchup.retiring = Some(id);
        state.final_catchup.current = Some(CatchupRecord {
            id: Uuid::now_v7(),
            block_start: 10,
            block_end: 20,
        });
        store.save(&state).await.unwrap();

        let loaded = store.load().await.unwrap();
        assert_eq!(loaded.final_catchup.retiring, Some(id));
        assert_eq!(
            loaded.final_catchup.current,
            state.final_catchup.current.clone()
        );
    }

    #[tokio::test]
    async fn missing_file_reads_as_empty_state() {
        let dir = tempfile::tempdir().unwrap();
        let store = store(&dir);
        assert!(store.current_id(Flow::Catchup).await.unwrap().is_none());
    }

    /// Both flows reconcile concurrently at boot against one file.
    ///
    /// This mirrors the read-modify-write cycle in [`reconcile`] rather than
    /// calling it, because `reconcile` needs a live broker connection. The
    /// `yield_now` stands in for the broker round trip that sits between the
    /// read and the write in the real thing.
    ///
    /// Drop the `lock` line and this fails two ways at once: both tasks write
    /// the same `.json.tmp` and one renames it out from under the other
    /// (`ENOENT` — the error seen in the live run), and whichever save does
    /// land erases the other flow's record because both read the same empty
    /// file. Either way a record ends up in the listener but not on disk,
    /// which is an orphaned catchup that can never be cancelled.
    #[tokio::test]
    async fn concurrent_flow_reconciles_do_not_lose_each_others_records() {
        async fn claim(store: &Store, flow: Flow, id: Uuid) {
            let _guard = store.lock.lock().await;
            let mut state = store.load().await.unwrap();
            tokio::task::yield_now().await;
            state.flow_mut(flow).current = Some(record(id, 1, 100));
            store.save(&state).await.unwrap();
        }

        let dir = tempfile::tempdir().unwrap();
        let store = store(&dir);
        let (live, final_) = (Uuid::now_v7(), Uuid::now_v7());

        // Clones on purpose: the control plane gets a clone of this store
        // (`main.rs`), so the lock has to be shared through `Clone` to be
        // worth anything.
        let (a, b) = (store.clone(), store.clone());
        tokio::join!(
            claim(&a, Flow::Catchup, live),
            claim(&b, Flow::FinalCatchup, final_),
        );

        let state = store.load().await.unwrap();
        assert_eq!(
            state.flow(Flow::Catchup).current.as_ref().map(|r| r.id),
            Some(live),
            "the live-catchup record was lost"
        );
        assert_eq!(
            state
                .flow(Flow::FinalCatchup)
                .current
                .as_ref()
                .map(|r| r.id),
            Some(final_),
            "the final-catchup record was lost"
        );
    }
}
