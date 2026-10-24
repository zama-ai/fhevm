//! Persisted manual replay. Bounds are resolved once per requested flags.

use anyhow::{bail, Context, Result};

pub mod store;

/// Reconcile persisted commands at startup and while replicas add or suppress
/// requests. Empty CLI arguments leave all existing intent untouched.
pub(super) async fn run(
    db: crate::database::tfhe_event_propagate::Database,
    client: consumer::ListenerConsumer,
    args: ManualCatchupArgs,
    reference: tokio::sync::oneshot::Receiver<u64>,
    heads: LiveReference,
    mode: std::sync::Arc<fhevm_engine_common::versioning::StackMode>,
) -> Result<()> {
    let creator_owner = uuid::Uuid::new_v4();
    let (requested, requested_id) = tokio::sync::oneshot::channel();
    let work = async {
        tokio::try_join!(
            configure_requested(
                &db,
                &client,
                &args,
                reference,
                creator_owner,
                requested
            ),
            maintain_leases(
                &db,
                &client,
                &heads,
                &mode,
                creator_owner,
                requested_id
            ),
        )?;
        Ok(())
    };
    let result = tokio::select! {
        _ = client.cancel_token.cancelled() => Ok(()),
        result = work => result,
    };
    if mode.is_paused() {
        client.cancel();
    }
    result
}

async fn configure_requested(
    db: &crate::database::tfhe_event_propagate::Database,
    client: &consumer::ListenerConsumer,
    args: &ManualCatchupArgs,
    reference: tokio::sync::oneshot::Receiver<u64>,
    creator_owner: uuid::Uuid,
    requested: tokio::sync::oneshot::Sender<uuid::Uuid>,
) -> Result<()> {
    if !args.enabled() {
        return Ok(());
    }
    let chain_id = i64::try_from(db.chain_id.as_u64())?;
    let existing = store::ensure_requested(
        &db.pool().await,
        chain_id,
        client.consumer_id(),
        args,
        None,
        Some(creator_owner),
        db.gcs_mode(),
    )
    .await?;
    let id = match existing {
        Some(id) => Some(id),
        None => {
            let live_block = reference
                .await
                .context("live flow ended before resolving catchup bounds")?;
            store::ensure_requested(
                &db.pool().await,
                chain_id,
                client.consumer_id(),
                args,
                Some(live_block),
                Some(creator_owner),
                db.gcs_mode(),
            )
            .await?
        }
    };
    if let Some(id) = id {
        let _ = requested.send(id);
    }
    Ok(())
}

/// Resume abandoned requests at startup, then keep leases alive and recover
/// requests abandoned by other listeners while this one is running.
async fn maintain_leases(
    db: &crate::database::tfhe_event_propagate::Database,
    client: &consumer::ListenerConsumer,
    heads: &LiveReference,
    mode: &fhevm_engine_common::versioning::StackMode,
    creator_owner: uuid::Uuid,
    mut requested_id: tokio::sync::oneshot::Receiver<uuid::Uuid>,
) -> Result<()> {
    if mode.is_paused() {
        return Ok(());
    }
    let chain_id = i64::try_from(db.chain_id.as_u64())?;
    let mut owned = store::take_request_ownership(
        &db.pool().await,
        chain_id,
        client.consumer_id(),
        db.gcs_mode(),
    )
    .await?;
    publish_pending_requests(db, client, heads, mode, &mut owned).await?;

    let period = std::time::Duration::from_secs(5);
    let mut interval =
        tokio::time::interval_at(tokio::time::Instant::now() + period, period);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        interval.tick().await;
        if mode.is_paused() {
            return Ok(());
        }

        // Renew current leases and adopt the requested catchup's creation lease.
        let mut ids = owned.iter().map(|r| r.id).collect::<Vec<_>>();
        let mut owners = owned.iter().map(|r| r.owner_id).collect::<Vec<_>>();
        if let Ok(id) = requested_id.try_recv() {
            ids.push(id);
            owners.push(creator_owner);
        }
        owned = store::renew(&db.pool().await, &ids, &owners, db.gcs_mode())
            .await?;
        owned.extend(
            store::take_request_ownership(
                &db.pool().await,
                chain_id,
                client.consumer_id(),
                db.gcs_mode(),
            )
            .await?,
        );
        publish_pending_requests(db, client, heads, mode, &mut owned).await?;
    }
}

async fn publish_pending_requests(
    db: &crate::database::tfhe_event_propagate::Database,
    client: &consumer::ListenerConsumer,
    heads: &LiveReference,
    mode: &fhevm_engine_common::versioning::StackMode,
    owned: &mut [store::Request],
) -> Result<()> {
    owned.sort_by_key(|r| (!r.cancel_pending, r.id));

    for command in owned
        .iter_mut()
        .filter(|r| r.request_pending || r.cancel_pending)
    {
        if mode.is_paused() {
            return Ok(());
        }
        if !store::owns_command(&db.pool().await, command).await? {
            continue;
        }
        if !command.cancel_pending {
            let head = heads.head(command.finalized).await;
            let Some(end) = store::bound_for_publication(
                &db.pool().await,
                command,
                head,
                db.gcs_mode(),
            )
            .await?
            else {
                continue;
            };
            command.block_end = end;
        }
        publish_command(client, command).await?;
        let acknowledged = if command.cancel_pending {
            store::cancelled(
                &db.pool().await,
                command.id,
                command.owner_id,
                db.gcs_mode(),
            )
            .await?
        } else {
            store::requested(
                &db.pool().await,
                command.id,
                command.owner_id,
                db.gcs_mode(),
            )
            .await?
        };
        if acknowledged {
            tracing::info!(catchup_id = %command.id, owner_id = %command.owner_id,
                finalized = command.finalized, cancelled = command.cancel_pending,
                start = command.block_start, end = command.block_end,
                "Published persisted catchup command");
        }
    }
    Ok(())
}

async fn publish_command(
    client: &consumer::ListenerConsumer,
    command: &store::Request,
) -> Result<()> {
    let start = u64::try_from(command.block_start)?;
    let end = u64::try_from(command.block_end)?;
    let publish = async {
        match (command.cancel_pending, command.finalized) {
            (true, true) => {
                client.cancel_final_catchup_request(command.id).await
            }
            (true, false) => client.cancel_catchup_request(command.id).await,
            (false, true) => {
                client.request_final_catchup(command.id, start, end).await
            }
            (false, false) => {
                client.request_catchup(command.id, start, end).await
            }
        }
    };
    // Bound publication so a stalled broker cannot block lease renewal forever.
    tokio::time::timeout(std::time::Duration::from_secs(15), publish)
        .await
        .context("catchup command publication timed out")??;
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct ManualCatchupArgs {
    pub catchup_from_block: Option<i64>,
    pub catchup_up_to_block: Option<i64>,
    pub catchup_unfinalized: bool,
    pub no_override_catchup: bool,
}

impl ManualCatchupArgs {
    pub fn validate(&self) -> Result<()> {
        if self.catchup_unfinalized && !self.enabled() {
            bail!("--catchup-unfinalized requires --catchup-from-block");
        }
        if self.no_override_catchup && !self.enabled() {
            bail!("--no-override-catchup requires --catchup-from-block");
        }
        if self.catchup_up_to_block.is_some()
            && self.catchup_from_block.is_none()
        {
            bail!("--catchup-up-to-block requires --catchup-from-block");
        }
        if let (Some(from), Some(to)) =
            (self.catchup_from_block, self.catchup_up_to_block)
        {
            if from >= 0 && to >= 0 && from > to {
                bail!("--catchup-from-block must not exceed --catchup-up-to-block");
            }
        }
        Ok(())
    }

    pub fn enabled(&self) -> bool {
        self.catchup_from_block.is_some()
    }

    /// The flags as given, `--catchup-up-to-block` defaulting to -1. They
    /// identify the catchup across restarts.
    pub fn requested(&self) -> Result<(i64, i64)> {
        let from = self
            .catchup_from_block
            .context("manual catchup is disabled")?;
        Ok((from, self.catchup_up_to_block.unwrap_or(-1)))
    }

    pub fn resolve(&self, live_block: u64) -> Result<(u64, u64)> {
        self.validate()?;
        let resolve = |value: i64| {
            if value < 0 {
                live_block.saturating_sub(value.unsigned_abs())
            } else {
                value as u64
            }
        };
        let (from, to) = self.requested()?;
        let start = resolve(from);
        let end = resolve(to);
        if start > end {
            bail!("Invalid manual catchup range [{start}, {end}] at live block {live_block}");
        }
        Ok((start, end))
    }
}

/// One reference per process startup, shared by concurrent handler invocations.
#[derive(Clone)]
pub(super) struct LiveReference {
    chain_id: u64,
    heads: std::sync::Arc<tokio::sync::RwLock<ObservedHeads>>,
    sender: std::sync::Arc<
        tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<u64>>>,
    >,
}

#[derive(Default)]
struct ObservedHeads {
    live: Option<u64>,
    finalized: Option<u64>,
}

impl LiveReference {
    pub fn new(chain_id: u64) -> (Self, tokio::sync::oneshot::Receiver<u64>) {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        (
            Self {
                chain_id,
                heads: Default::default(),
                sender: std::sync::Arc::new(tokio::sync::Mutex::new(Some(
                    sender,
                ))),
            },
            receiver,
        )
    }

    /// Observed eligible heights may lag core. Never infer finality from LIVE.
    async fn head(&self, finalized: bool) -> Option<u64> {
        let heads = self.heads.read().await;
        if finalized {
            heads.finalized
        } else {
            heads.live
        }
    }

    pub async fn observe(&self, payload: &consumer::BlockPayload) {
        use primitives::event::BlockFlow;
        if payload.chain_id != self.chain_id {
            return;
        }
        {
            let mut heads = self.heads.write().await;
            let head = match payload.flow {
                BlockFlow::Live => Some(&mut heads.live),
                BlockFlow::Final => Some(&mut heads.finalized),
                _ => None,
            };
            if let Some(head) = head {
                *head = Some(head.unwrap_or(0).max(payload.block_number));
            }
        }
        if payload.flow == BlockFlow::Live {
            if let Some(sender) = self.sender.lock().await.take() {
                let _ = sender.send(payload.block_number);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(from: i64, to: Option<i64>) -> ManualCatchupArgs {
        ManualCatchupArgs {
            catchup_from_block: Some(from),
            catchup_up_to_block: to,
            catchup_unfinalized: false,
            no_override_catchup: false,
        }
    }

    #[tokio::test]
    async fn reference_uses_only_the_first_live_block_on_the_right_chain() {
        use primitives::event::BlockFlow;
        let (reference, mut receiver) = LiveReference::new(1);
        let mut payload = consumer::BlockPayload {
            chain_id: 1,
            flow: BlockFlow::FinalCatchup,
            block_number: 100,
            block_hash: Default::default(),
            parent_hash: Default::default(),
            timestamp: 0,
            transactions: vec![],
            catchup_id: None,
        };
        reference.observe(&payload).await;
        payload.flow = BlockFlow::Reorged;
        reference.observe(&payload).await;
        payload.flow = BlockFlow::Live;
        payload.chain_id = 2;
        reference.observe(&payload).await;
        assert!(matches!(
            receiver.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        payload.chain_id = 1;
        reference.observe(&payload).await;
        payload.block_number = 200;
        reference.clone().observe(&payload).await;
        assert_eq!(receiver.await.unwrap(), 100);
        assert_eq!(reference.head(false).await, Some(200));
        assert_eq!(reference.head(true).await, None);
        payload.flow = BlockFlow::FinalCatchup;
        payload.block_number = 300;
        reference.observe(&payload).await;
        assert_eq!(reference.head(true).await, None);
        payload.flow = BlockFlow::Final;
        payload.block_number = 150;
        reference.observe(&payload).await;
        payload.block_number = 100;
        reference.observe(&payload).await;
        assert_eq!(reference.head(true).await, Some(150));
        payload.chain_id = 2;
        payload.block_number = 1000;
        reference.observe(&payload).await;
        assert_eq!(reference.head(true).await, Some(150));
    }

    #[test]
    fn requested_flags_default_the_end_to_minus_one() {
        assert_eq!(args(900, None).requested().unwrap(), (900, -1));
        assert_eq!(args(-10, Some(-2)).requested().unwrap(), (-10, -2));
    }

    #[test]
    fn inclusive_ranges_and_relative_bounds() {
        for (from, to, expected) in [
            (10, Some(20), (10, 20)),
            (42, Some(42), (42, 42)),
            (-100, Some(-10), (900, 990)),
            (10, None, (10, 999)),
            (-100, None, (900, 999)),
            (0, Some(0), (0, 0)),
            (i64::MIN, None, (0, 999)),
        ] {
            assert_eq!(args(from, to).resolve(1000).unwrap(), expected);
        }
        assert_eq!(args(-100, None).resolve(10).unwrap(), (0, 9));
    }

    #[test]
    fn omitted_end_matches_minus_one_and_keeps_live_boundary_out_of_replay() {
        assert_eq!(args(-100, None).resolve(1000).unwrap(), (900, 999));
        assert_eq!(
            args(-100, None).resolve(1000).unwrap(),
            args(-100, Some(-1)).resolve(1000).unwrap()
        );
        assert_eq!(args(-1, None).resolve(1000).unwrap(), (999, 999));
        assert!(args(1000, None).resolve(1000).is_err());
        // Explicit ends remain inclusive, including the first live block.
        assert_eq!(args(1000, Some(1000)).resolve(1000).unwrap(), (1000, 1000));
    }

    #[test]
    fn default_end_saturates_at_genesis() {
        assert_eq!(args(0, None).resolve(0).unwrap(), (0, 0));
        assert_eq!(args(-1, None).resolve(1).unwrap(), (0, 0));
        assert_eq!(
            args(0, None).resolve(0).unwrap(),
            args(0, Some(-1)).resolve(0).unwrap()
        );
    }

    #[test]
    fn invalid_ranges_are_rejected() {
        assert!(args(20, Some(10)).validate().is_err());
        assert!(args(-10, Some(-20)).resolve(1000).is_err());
        assert!(args(1001, None).resolve(1000).is_err());
        assert!(ManualCatchupArgs::default().resolve(1000).is_err());
    }

    #[test]
    fn absolute_bounds_do_not_depend_on_a_backlogged_live_reference() {
        assert_eq!(args(2000, Some(3000)).resolve(1000).unwrap(), (2000, 3000));
        assert_eq!(args(-100, Some(3000)).resolve(1000).unwrap(), (900, 3000));
    }
}
