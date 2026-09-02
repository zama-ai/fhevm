//! TTL'd in-memory snapshot of the local context and epoch DB tables.

use alloy::primitives::U256;
use sqlx::{Pool, Postgres};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

/// A periodically-refreshed in-memory mirror of the context and epoch DB tables.
#[derive(Clone)]
pub struct KmsContextCache {
    /// DB instance used to (re)load the cache snapshot.
    db_pool: Pool<Postgres>,

    /// The current cache snapshot.
    // Cheap to clone: the outer `Arc` shares the lock across clones; the `RwLock` gives interior
    // mutability for the swap; the inner `Arc` makes reads snapshot-and-release.
    snapshot: Arc<RwLock<Arc<KmsContextCacheSnapshot>>>,
}

/// Outcome of a context/epoch lookup in the `KmsContextCache`.
pub enum LocalCheck {
    Valid,
    Destroyed,
    Unknown,
}

/// An immutable snapshot of the context and epoch DB tables at one point in time.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KmsContextCacheSnapshot {
    /// `id -> is_valid` for every cached KMS context.
    contexts: HashMap<U256, bool>,
    /// Every cached KMS epoch, by ID.
    epochs: HashMap<U256, EpochEntry>,
}

/// A cached epoch entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpochEntry {
    // `None` for invalidations written from `KmsEpochDestroyed`, which only carries the epoch ID.
    context_id: Option<U256>,
    is_valid: bool,
}

impl KmsContextCacheSnapshot {
    /// Loads a fresh snapshot from the context and epoch DB tables.
    pub async fn load(db_pool: &Pool<Postgres>) -> anyhow::Result<Self> {
        let (context_rows, epoch_rows) = tokio::try_join!(
            sqlx::query!("SELECT id, is_valid FROM kms_context").fetch_all(db_pool),
            sqlx::query!("SELECT id, context_id, is_valid FROM kms_epoch").fetch_all(db_pool),
        )?;

        let contexts = context_rows
            .into_iter()
            .filter_map(|row| decode_id("kms_context.id", &row.id).map(|id| (id, row.is_valid)))
            .collect();
        let epochs = epoch_rows
            .into_iter()
            .filter_map(|row| {
                let entry = EpochEntry {
                    context_id: row
                        .context_id
                        .and_then(|id| decode_id("kms_epoch.context_id", &id)),
                    is_valid: row.is_valid,
                };
                decode_id("kms_epoch.id", &row.id).map(|id| (id, entry))
            })
            .collect();

        Ok(Self { contexts, epochs })
    }

    /// Checks the requested context and epoch against the cached tables.
    pub fn check(&self, context_id: U256, epoch_id: Option<U256>) -> LocalCheck {
        let Some(&context_is_valid) = self.contexts.get(&context_id) else {
            return LocalCheck::Unknown; // Context not cached
        };
        if !context_is_valid {
            return LocalCheck::Destroyed;
        }
        let Some(epoch_id) = epoch_id else {
            // v1 extra_data carries no epoch: the context entry alone concludes
            return LocalCheck::Valid;
        };
        match self.epochs.get(&epoch_id) {
            None => LocalCheck::Unknown, // Epoch not cached
            Some(epoch) if !epoch.is_valid => LocalCheck::Destroyed,
            Some(epoch) if epoch.context_id == Some(context_id) => LocalCheck::Valid,
            // In case of mismatch between the cached context and the requested one, double-check
            // on-chain instead of rejecting, and let `insert_valid_pair` fix the entry if needed
            Some(_) => {
                warn!("Requested context does not match the one cached for this epoch");
                LocalCheck::Unknown
            }
        }
    }
}

/// Decodes an ID column written with `U256::as_le_slice`.
///
/// A malformed row is persistent DB state, so failing on it would crash-loop the worker: it is
/// skipped with a warning instead, and the lookup falls back to the on-chain validation.
fn decode_id(source: &str, bytes: &[u8]) -> Option<U256> {
    let id = U256::try_from_le_slice(bytes);
    if id.is_none() {
        warn!(
            "Malformed {source} ID in DB ({} bytes), skipping it",
            bytes.len()
        );
    }
    id
}

impl KmsContextCache {
    /// Loads the initial snapshot and spawns the background refresh task.
    ///
    /// `cancel_token` is the worker-wide shutdown token: the refresh task cancels it on a
    /// critical failure (see [`Self::spawn_refresh_task`]).
    pub async fn connect(
        db_pool: Pool<Postgres>,
        refresh_interval: Duration,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        anyhow::ensure!(
            !refresh_interval.is_zero(),
            "kms_context_cache_refresh must be non-zero"
        );

        let snapshot = KmsContextCacheSnapshot::load(&db_pool).await?;
        let cache = Self {
            db_pool,
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
        };
        cache.spawn_refresh_task(refresh_interval, cancel_token);
        Ok(cache)
    }

    /// Clones the inner `Arc` of the current snapshot and drops the guard, so no lock is held.
    pub fn snapshot(&self) -> Arc<KmsContextCacheSnapshot> {
        self.snapshot
            .read()
            .expect("KMS context cache lock poisoned")
            .clone()
    }

    /// Registers a pair confirmed valid on-chain.
    ///
    /// This way, following requests conclude from the cache instead of re-validating on-chain
    /// until the next cache refresh.
    pub fn insert_valid_pair(&self, context_id: U256, epoch_id: U256) {
        let Ok(mut guard) = self.snapshot.write() else {
            // Let the background task handle the lock poisoning, only warning here.
            return warn!("KMS context cache lock poisoned, not caching pair");
        };
        let mut snapshot = (**guard).clone();

        // We don't override the context and epoch `is_valid` flags, in case a destruction event
        // is processed between the on-chain read and this write.
        snapshot.contexts.entry(context_id).or_insert(true);
        snapshot
            .epochs
            .entry(epoch_id)
            .and_modify(|epoch| epoch.context_id = Some(context_id))
            .or_insert(EpochEntry {
                context_id: Some(context_id),
                is_valid: true,
            });
        *guard = Arc::new(snapshot);
    }

    /// Spawns the background task that reloads the cache on the configured TTL.
    ///
    /// A transient reload failure keeps the previous snapshot. A poisoned snapshot lock cancels
    /// `cancel_token` to bring the whole worker down.
    fn spawn_refresh_task(&self, refresh_interval: Duration, cancel_token: CancellationToken) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(refresh_interval);
            // First tick fires immediately; the snapshot is already fresh from `connect`, so
            // consume it before the reload loop.
            interval.tick().await;

            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => break,
                    _ = interval.tick() => {}
                }

                let snapshot = match KmsContextCacheSnapshot::load(&this.db_pool).await {
                    Ok(snapshot) => snapshot,
                    Err(e) => {
                        warn!(
                            "Failed to refresh KMS context cache, keeping previous snapshot: {e}"
                        );
                        continue;
                    }
                };
                let Ok(mut guard) = this.snapshot.write() else {
                    error!("Shutting down worker on poisoned KMS context cache lock");
                    cancel_token.cancel();
                    break;
                };
                *guard = Arc::new(snapshot);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(
        contexts: &[(u64, bool)],
        epochs: &[(u64, Option<u64>, bool)],
    ) -> KmsContextCacheSnapshot {
        KmsContextCacheSnapshot {
            contexts: contexts
                .iter()
                .map(|&(id, is_valid)| (U256::from(id), is_valid))
                .collect(),
            epochs: epochs
                .iter()
                .map(|&(id, context_id, is_valid)| {
                    let entry = EpochEntry {
                        context_id: context_id.map(U256::from),
                        is_valid,
                    };
                    (U256::from(id), entry)
                })
                .collect(),
        }
    }

    #[test]
    fn check_concludes_from_cached_pairs() {
        let snapshot = snapshot(
            &[(1, true), (2, false)],
            &[(10, Some(1), true), (11, Some(1), false), (12, None, false)],
        );

        assert!(matches!(
            snapshot.check(U256::from(1), Some(U256::from(10))),
            LocalCheck::Valid
        ));
        // v1 extra_data: the context alone concludes
        assert!(matches!(
            snapshot.check(U256::from(1), None),
            LocalCheck::Valid
        ));
        // Destroyed context rejects any epoch, even one not cached
        assert!(matches!(
            snapshot.check(U256::from(2), Some(U256::from(99))),
            LocalCheck::Destroyed
        ));
        // Destroyed epoch rejects, even without a cached context association
        assert!(matches!(
            snapshot.check(U256::from(1), Some(U256::from(11))),
            LocalCheck::Destroyed
        ));
        assert!(matches!(
            snapshot.check(U256::from(1), Some(U256::from(12))),
            LocalCheck::Destroyed
        ));
    }

    #[test]
    fn check_falls_back_to_unknown() {
        let snapshot = snapshot(&[(1, true), (2, true)], &[(10, Some(1), true)]);

        // Context not cached
        assert!(matches!(
            snapshot.check(U256::from(9), Some(U256::from(10))),
            LocalCheck::Unknown
        ));
        // Epoch not cached
        assert!(matches!(
            snapshot.check(U256::from(1), Some(U256::from(99))),
            LocalCheck::Unknown
        ));
        // Epoch cached as valid under another context: the association is not authoritative
        assert!(matches!(
            snapshot.check(U256::from(2), Some(U256::from(10))),
            LocalCheck::Unknown
        ));
    }

    #[tokio::test]
    async fn insert_valid_pair_is_immediately_visible() {
        let cache = KmsContextCache {
            db_pool: sqlx::Pool::connect_lazy("postgres://unused").unwrap(),
            snapshot: Arc::new(RwLock::new(Arc::new(KmsContextCacheSnapshot::default()))),
        };
        let (context_id, epoch_id) = (U256::from(1), U256::from(10));

        assert!(matches!(
            cache.snapshot().check(context_id, Some(epoch_id)),
            LocalCheck::Unknown
        ));
        cache.insert_valid_pair(context_id, epoch_id);
        assert!(matches!(
            cache.snapshot().check(context_id, Some(epoch_id)),
            LocalCheck::Valid
        ));
    }

    #[tokio::test]
    async fn insert_valid_pair_repairs_association_but_never_validity() {
        let cache = KmsContextCache {
            db_pool: sqlx::Pool::connect_lazy("postgres://unused").unwrap(),
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot(
                &[(1, false)],
                &[(10, Some(2), false)],
            )))),
        };
        cache.insert_valid_pair(U256::from(1), U256::from(10));

        let snapshot = cache.snapshot();
        // The destroyed context and epoch validities must not be reverted...
        assert!(!snapshot.contexts[&U256::from(1)]);
        assert!(!snapshot.epochs[&U256::from(10)].is_valid);
        // ...while the epoch's context association is repaired
        assert_eq!(
            snapshot.epochs[&U256::from(10)].context_id,
            Some(U256::from(1))
        );
    }

    #[tokio::test]
    async fn connect_rejects_zero_refresh_interval() {
        let db_pool = sqlx::Pool::connect_lazy("postgres://unused").unwrap();
        let result =
            KmsContextCache::connect(db_pool, Duration::ZERO, CancellationToken::new()).await;
        if result.is_ok() {
            panic!("zero refresh interval should be rejected");
        };
    }
}
