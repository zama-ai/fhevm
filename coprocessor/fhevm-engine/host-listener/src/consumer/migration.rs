//! One-shot retirement of a predecessor consumer identity.
//!
//! **This module is temporary.** It exists to carry each environment across the
//! single release in which the broker identity became the compiled
//! [`DEFAULT_CONSUMER_ID`](super::DEFAULT_CONSUMER_ID) constant instead of a
//! value derived from `--service-name`. Once every environment has been through
//! that release, delete this file, the `--migrate-from-service-name` flag and
//! the call site. It is the only deliberately disposable part of this change.
//!
//! # Why a task inside the pod, and not a Job or a runbook
//!
//! The cleanup has a precondition — the predecessor must have stopped reading —
//! that can only be *observed*, never scheduled. A Kubernetes Job runs when the
//! scheduler says so, and a runbook step runs when a human says so; neither can
//! wait for a fact about the data plane.
//!
//! More importantly, running here means the janitor is co-located with the
//! survivor. Whatever this task tears down, the pod it runs in is already
//! subscribed to the path that survives. That is what bounds the blast radius
//! of a misfire to replayed work rather than lost work, and it is a guarantee a
//! Job or a laptop does not have.
//!
//! # The two branches
//!
//! Selected by whether the old identity differs from the one this build uses:
//!
//! - **Rename** (`old != current`) — the predecessor owns its own filter row
//!   and its own four streams. Stop the publisher writing to them
//!   (`unregister_contracts`), *then* delete them. That order is the whole
//!   point: the reverse recreates every stream on the next published event, and
//!   leaves a filter row pointing at streams nobody reads, which is exactly the
//!   shape of the 25 September incident.
//!
//!   Ordering those two calls is necessary but not sufficient, because only the
//!   second is synchronous. `unregister_contracts` publishes a *command*; the
//!   filter row it removes lives in the listener's database and goes away some
//!   time later. Deleting the streams in the same breath therefore deletes them
//!   while publishing is still in flight — and a publisher that finds its
//!   stream missing does not recreate it. It parks on the existence check and
//!   retries for ever, and because fan-out is sequential that stalls every
//!   consumer on the chain until the pod is restarted. The delete therefore
//!   waits on a second observed fact: that the streams have stopped being
//!   written to. See [`Phase`].
//! - **Same ID** (`old == current`) — the predecessor differs only by consumer
//!   group name, because it predates group suffixes. Its streams and its filter
//!   row are *shared with this build* and must not be touched. Only the
//!   unsuffixed group is removed.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use alloy::primitives::Address;
use consumer::{Broker, GroupStatus, ListenerConsumer};
use tokio::task::JoinHandle;
use tokio::time::{interval, MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

/// How often the predecessor's streams are sampled.
const POLL_INTERVAL: Duration = Duration::from_secs(60);

/// How many consecutive samples a group must sit at the same last-delivered ID,
/// with entries waiting behind it, before it counts as abandoned.
///
/// Ten samples at a one-minute interval is ten minutes of stillness. That is
/// deliberately far longer than any rolling restart, node drain or brief
/// outage, during which a live group looks identical to a dead one: its cursor
/// stops and its lag climbs. Being slow costs nothing here — the orphan is
/// inert, and item 1's stream ceiling already bounds what it can accumulate.
const REQUIRED_FROZEN_SAMPLES: u32 = 10;

/// How many consecutive samples the predecessor's streams must sit at the same
/// write position before the filter row is believed to be gone.
///
/// Two samples a minute apart is two minutes without a single append. The
/// unregister command that causes this takes milliseconds to apply, so two
/// minutes is a wide margin; the margin buys cover for the case the ordering
/// alone cannot cover, which is a command that was dropped rather than merely
/// slow. Each sample re-sends it, so a dropped one heals rather than hangs.
const REQUIRED_QUIET_SAMPLES: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Branch {
    /// The predecessor has its own filter row and streams to remove.
    Rename,
    /// The predecessor shares this build's streams; only its group goes.
    SameId,
}

/// Where the rename branch has got to.
///
/// The same-ID branch has no phases: it destroys a consumer group, which
/// leaves the stream and every other group on it untouched, so there is
/// nothing for a publisher to trip over and nothing to wait for.
enum Phase {
    /// Sampling the predecessor's groups, waiting for them to stop reading.
    Watching,
    /// Its filters have been unregistered. Sampling write positions now,
    /// waiting for the publisher to stop, before the streams can go.
    Draining {
        positions: Vec<(String, Option<String>)>,
        quiet: u32,
    },
}

/// Watch the predecessor identity until it is abandoned, retire it, and stop.
///
/// The task exits on its own once the work is done, so a pod that has already
/// migrated pays one Redis round trip a minute until it is restarted without
/// the flag.
pub fn spawn_id_migration(
    broker: Broker,
    client: ListenerConsumer,
    old_consumer_id: String,
    contracts: Vec<Address>,
    cancel: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        run(broker, client, old_consumer_id, contracts, cancel).await;
    })
}

async fn run(
    broker: Broker,
    client: ListenerConsumer,
    old_consumer_id: String,
    contracts: Vec<Address>,
    cancel: CancellationToken,
) {
    let branch = if old_consumer_id == client.consumer_id() {
        Branch::SameId
    } else {
        Branch::Rename
    };

    // In the rename branch the subject is the *predecessor*, so it is built
    // with the old ID and no group suffix: it names the old streams and the old
    // filter row. In the same-ID branch the subject is this build itself, whose
    // suffix is what makes removing the unsuffixed group safe.
    let subject = match branch {
        Branch::Rename => {
            ListenerConsumer::new(&broker, client.chain_id(), &old_consumer_id)
        }
        Branch::SameId => client,
    };

    match branch {
        Branch::Rename => info!(
            old_consumer_id = %old_consumer_id,
            "Consumer identity migration armed: will unregister and delete the predecessor's streams once it stops reading"
        ),
        Branch::SameId => info!(
            consumer_id = %old_consumer_id,
            "Consumer identity migration armed: identity is unchanged, will destroy the unsuffixed consumer group once it stops reading"
        ),
    }

    let mut stillness: HashMap<(String, String), (String, u32)> =
        HashMap::new();
    let mut phase = Phase::Watching;
    let mut ticker = interval(POLL_INTERVAL);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                info!("Consumer identity migration cancelled before it could finish");
                return;
            }
            _ = ticker.tick() => {}
        }

        match &mut phase {
            Phase::Watching => {
                let statuses = match subject.group_status().await {
                    Ok(statuses) => statuses,
                    Err(err) => {
                        warn!(error = %err, "Could not read the predecessor's consumer groups, retrying");
                        continue;
                    }
                };

                let targets = select_targets(&statuses, branch);

                // Finding no groups means one of two unrelated things, and
                // `XINFO GROUPS` cannot say which: the predecessor's streams
                // are gone, or they are still there with nobody reading them.
                // Only the second is something to retire. The write position
                // tells them apart, because it is `None` only when the key
                // itself is missing.
                if branch == Branch::Rename && targets.is_empty() {
                    match subject.write_positions().await {
                        Ok(positions)
                            if positions.iter().all(|(_, at)| at.is_none()) =>
                        {
                            let streams: Vec<&str> = positions
                                .iter()
                                .map(|(stream, _)| stream.as_str())
                                .collect();
                            info!(
                                old_consumer_id = %old_consumer_id,
                                streams = %streams.join(", "),
                                "Nothing to retire: the predecessor has no streams. Either it has already been retired, or this is not the identity this environment used — compare the name against filters.consumer_id"
                            );
                            return;
                        }
                        Ok(_) => {}
                        Err(err) => {
                            warn!(error = %err, "Could not check whether the predecessor still has streams, retrying");
                            continue;
                        }
                    }
                }

                if !is_abandoned(&targets, &mut stillness) {
                    continue;
                }

                match branch {
                    Branch::SameId => {
                        // Destroying a group touches neither the stream nor
                        // the filter row, so there is nothing to drain.
                        match subject.destroy_unsuffixed_groups().await {
                            Ok(destroyed) => {
                                info!(
                                    consumer_id = %subject.consumer_id(),
                                    destroyed,
                                    "Destroyed the predecessor's unsuffixed consumer groups"
                                );
                                info!(
                                    old_consumer_id = %old_consumer_id,
                                    "Consumer identity migration complete"
                                );
                                return;
                            }
                            Err(err) => {
                                error!(error = %err, "Could not destroy the predecessor's consumer groups, retrying");
                            }
                        }
                    }
                    Branch::Rename => {
                        match start_draining(&subject, &contracts).await {
                            Ok(positions) => {
                                phase = Phase::Draining {
                                    positions,
                                    quiet: 0,
                                };
                            }
                            Err(err) => {
                                error!(error = %err, "Could not unregister the predecessor's filters, retrying");
                            }
                        }
                    }
                }
            }

            Phase::Draining { positions, quiet } => {
                // Re-sent every sample. `remove_filter` matches an exact tuple,
                // so repeating it is a no-op once the row is gone — and it is
                // what turns a dropped command into a delay instead of a hang.
                if let Err(err) = unregister(&subject, &contracts).await {
                    warn!(error = %err, "Could not re-send the predecessor's filter removal, retrying");
                    continue;
                }

                let current = match subject.write_positions().await {
                    Ok(current) => current,
                    Err(err) => {
                        warn!(error = %err, "Could not read the predecessor's stream write positions, retrying");
                        continue;
                    }
                };

                if current == *positions {
                    *quiet += 1;
                } else {
                    info!(
                        consumer_id = %subject.consumer_id(),
                        "The predecessor's streams are still being written to, so the filter row is still live"
                    );
                    *positions = current;
                    *quiet = 0;
                    continue;
                }

                if *quiet < REQUIRED_QUIET_SAMPLES {
                    continue;
                }

                match subject.delete_streams().await {
                    Ok(deleted) => {
                        info!(
                            consumer_id = %subject.consumer_id(),
                            deleted,
                            "Deleted the predecessor's streams"
                        );
                        info!(
                            old_consumer_id = %old_consumer_id,
                            "Consumer identity migration complete"
                        );
                        return;
                    }
                    Err(err) => {
                        error!(error = %err, "Could not delete the predecessor's streams, retrying");
                    }
                }
            }
        }
    }
}

/// Stop the publisher writing to the predecessor's streams, and take the first
/// reading of where they have been written to.
///
/// The reading is taken *after* the unregister so that the window it covers
/// starts no earlier than the command does.
async fn start_draining(
    subject: &ListenerConsumer,
    contracts: &[Address],
) -> anyhow::Result<Vec<(String, Option<String>)>> {
    unregister(subject, contracts).await?;
    info!(
        consumer_id = %subject.consumer_id(),
        contracts = contracts.len(),
        "Unregistered the predecessor's filters, waiting for its streams to go quiet before deleting them"
    );
    Ok(subject.write_positions().await?)
}

async fn unregister(
    subject: &ListenerConsumer,
    contracts: &[Address],
) -> anyhow::Result<()> {
    if contracts.is_empty() {
        return Ok(());
    }
    subject.unregister_contracts(contracts).await?;
    Ok(())
}

/// The groups whose stillness gates the cleanup, as `(stream, group)`.
///
/// The rename branch deletes whole streams, so *every* group on them matters.
/// The same-ID branch removes one group per stream and leaves the rest alone,
/// so only the unsuffixed group matters — and it is identified by name equality
/// with the stream key, which is what a build with no group suffix produces.
fn select_targets(
    statuses: &[(String, Vec<GroupStatus>)],
    branch: Branch,
) -> Vec<(&str, &GroupStatus)> {
    statuses
        .iter()
        .flat_map(|(stream, groups)| {
            groups
                .iter()
                .filter(move |group| match branch {
                    Branch::Rename => true,
                    Branch::SameId => group.name == *stream,
                })
                .map(move |group| (stream.as_str(), group))
        })
        .collect()
}

/// Whether every target group has stopped reading.
///
/// A group qualifies when its last-delivered ID has not moved for
/// [`REQUIRED_FROZEN_SAMPLES`] consecutive samples *and* entries are waiting
/// behind it. Both halves are needed: a cursor that does not move with nothing
/// to read is simply an idle group on a quiet stream, which says nothing about
/// whether anyone is holding it.
///
/// A group whose lag is unknown — Redis older than 7.0 does not report it —
/// never qualifies. Declining to act is the safe direction: the cost is an
/// orphan that outlives the release, against cutting a live reader's cursor.
///
/// No targets at all means nothing is reading. The caller separates the two
/// ways that happens before trusting it: streams that still exist with no
/// reader are the end state this waits for, whereas streams that were never
/// there are a name matching nothing, which is not a migration to run.
fn is_abandoned(
    targets: &[(&str, &GroupStatus)],
    stillness: &mut HashMap<(String, String), (String, u32)>,
) -> bool {
    let present: HashSet<(String, String)> = targets
        .iter()
        .map(|(stream, group)| ((*stream).to_string(), group.name.clone()))
        .collect();
    stillness.retain(|key, _| present.contains(key));

    let mut abandoned = true;
    for (stream, group) in targets {
        let key = ((*stream).to_string(), group.name.clone());
        let entry = stillness
            .entry(key)
            .or_insert_with(|| (group.last_delivered_id.clone(), 0));

        if entry.0 == group.last_delivered_id {
            entry.1 += 1;
        } else {
            entry.0.clone_from(&group.last_delivered_id);
            entry.1 = 1;
        }
        let samples = entry.1;

        let Some(lag) = group.lag else {
            warn!(
                stream = %stream,
                group = %group.name,
                "Redis does not report consumer group lag, so the predecessor cannot be retired automatically; upgrade to Redis 7.0 or clean up by hand"
            );
            abandoned = false;
            continue;
        };

        if samples < REQUIRED_FROZEN_SAMPLES || lag == 0 {
            abandoned = false;
        }

        if samples >= REQUIRED_FROZEN_SAMPLES && lag == 0 {
            // Still worth saying: a frozen cursor with nothing behind it is a
            // group that is caught up, not one that has been abandoned.
            info!(
                stream = %stream,
                group = %group.name,
                "Predecessor group is idle but caught up, so it is not yet safe to call it abandoned"
            );
        }
    }
    abandoned
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(
        name: &str,
        last_delivered_id: &str,
        lag: Option<u64>,
    ) -> GroupStatus {
        GroupStatus {
            name: name.to_string(),
            last_delivered_id: last_delivered_id.to_string(),
            pending: 0,
            lag,
        }
    }

    fn statuses() -> Vec<(String, Vec<GroupStatus>)> {
        vec![(
            "host-listener-consumer.1.new-event".to_string(),
            vec![
                group("host-listener-consumer.1.new-event", "5-0", Some(3)),
                group("host-listener-consumer.1.new-event.v2", "9-0", Some(0)),
            ],
        )]
    }

    #[test]
    fn same_id_branch_targets_only_the_unsuffixed_group() {
        let statuses = statuses();
        let targets = select_targets(&statuses, Branch::SameId);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].1.name, "host-listener-consumer.1.new-event");
    }

    #[test]
    fn rename_branch_targets_every_group_because_the_stream_is_going() {
        let statuses = statuses();
        assert_eq!(select_targets(&statuses, Branch::Rename).len(), 2);
    }

    #[test]
    fn a_group_must_be_still_for_the_full_window() {
        let statuses = statuses();
        let targets = select_targets(&statuses, Branch::SameId);
        let mut stillness = HashMap::new();

        for sample in 1..REQUIRED_FROZEN_SAMPLES {
            assert!(
                !is_abandoned(&targets, &mut stillness),
                "called abandoned after only {sample} sample(s)"
            );
        }
        assert!(is_abandoned(&targets, &mut stillness));
    }

    #[test]
    fn a_cursor_that_moves_restarts_the_window() {
        let still = vec![("s".to_string(), vec![group("s", "5-0", Some(3))])];
        let moved = vec![("s".to_string(), vec![group("s", "6-0", Some(3))])];
        let mut stillness = HashMap::new();

        // One sample short of retiring it.
        for _ in 0..REQUIRED_FROZEN_SAMPLES - 1 {
            assert!(!is_abandoned(
                &select_targets(&still, Branch::SameId),
                &mut stillness
            ));
        }

        // One sign of life. That sample is now the first of a new window, so
        // the whole wait has to be served again rather than resumed.
        let mut samples_since = 0;
        loop {
            let abandoned = is_abandoned(
                &select_targets(&moved, Branch::SameId),
                &mut stillness,
            );
            samples_since += 1;
            if abandoned {
                break;
            }
            assert!(
                samples_since < REQUIRED_FROZEN_SAMPLES,
                "window never completed after the cursor moved"
            );
        }
        assert_eq!(samples_since, REQUIRED_FROZEN_SAMPLES);
    }

    #[test]
    fn a_still_group_with_nothing_waiting_is_caught_up_not_abandoned() {
        let caught_up =
            vec![("s".to_string(), vec![group("s", "9-0", Some(0))])];
        let targets = select_targets(&caught_up, Branch::SameId);
        let mut stillness = HashMap::new();

        for _ in 0..REQUIRED_FROZEN_SAMPLES * 2 {
            assert!(!is_abandoned(&targets, &mut stillness));
        }
    }

    #[test]
    fn unknown_lag_never_qualifies() {
        let no_lag = vec![("s".to_string(), vec![group("s", "5-0", None)])];
        let targets = select_targets(&no_lag, Branch::SameId);
        let mut stillness = HashMap::new();

        for _ in 0..REQUIRED_FROZEN_SAMPLES * 2 {
            assert!(!is_abandoned(&targets, &mut stillness));
        }
    }

    #[test]
    fn nothing_reading_at_all_is_the_end_state() {
        let mut stillness = HashMap::new();
        assert!(is_abandoned(&[], &mut stillness));
    }
}
