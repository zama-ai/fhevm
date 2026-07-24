//! Read-only MMR proof construction from a SQL snapshot + on-chain peak check.
//!
//! Unlike the embedded relayer path, this never triggers catch-up ingestion.
//! Background completed-block ingest owns all writes; a lagging snapshot returns
//! retryable `Lagging`, and equal-count peak divergence fails closed.

use async_trait::async_trait;
use zama_solana_acl::mmr::{mmr_build_proof, mmr_peaks_from_leaves, MmrProof};

use crate::chain::{ChainFetcher, OnChainLineageState};
use solana_proof_store::{
    LeafKind, ProofSnapshot, ResolvedProofSnapshot, SemanticLeafKey, SqlProofStore, StoreError,
};

/// Read-only snapshot source for proof construction (no catch-up / writes). Resolution of a
/// semantic key to a leaf index happens inside the same consistent snapshot read.
#[async_trait]
pub trait ProofSnapshotSource: Send + Sync {
    async fn proof_snapshot_for_leaf(
        &self,
        encrypted_value_account: [u8; 32],
        key: &SemanticLeafKey,
    ) -> Result<Option<ResolvedProofSnapshot>, StoreError>;
}

#[async_trait]
impl ProofSnapshotSource for SqlProofStore {
    async fn proof_snapshot_for_leaf(
        &self,
        encrypted_value_account: [u8; 32],
        key: &SemanticLeafKey,
    ) -> Result<Option<ResolvedProofSnapshot>, StoreError> {
        SqlProofStore::proof_snapshot_for_leaf(self, encrypted_value_account, key).await
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MmrProofResult {
    pub mmr_proof: Option<MmrProof>,
    /// EncryptedValueAccount leaf count the proof was built against (matches the confirmed chain read).
    pub leaf_count: u64,
    /// Confirmed RPC context slot of the on-chain peak comparison that produced `verified`.
    pub rpc_context_slot: u64,
    /// Durable ingest slot at which this encrypted_value_account's served leaves were last written
    /// (`solana_proof_encrypted_value_accounts.last_slot`). `None` when no snapshot backed the result.
    pub encrypted_value_account_last_slot: Option<u64>,
    pub verified: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum ProofError {
    #[error("chain error: {0}")]
    Chain(#[from] crate::chain::ChainError),
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("no on-chain account found for encrypted_value_account")]
    LineageNotFound,
    #[error("encrypted_value_account proof data is lagging chain state (leaf_count {leaf_count})")]
    Lagging {
        leaf_count: u64,
        rpc_context_slot: u64,
        encrypted_value_account_last_slot: Option<u64>,
    },
    #[error(
        "encrypted_value_account proof store diverged from chain (leaf_count {leaf_count}); integrity rebuild required"
    )]
    CorruptStore {
        leaf_count: u64,
        rpc_context_slot: u64,
        encrypted_value_account_last_slot: Option<u64>,
    },
    /// Terminal: the encrypted_value_account exists and the store is caught up to chain
    /// (snapshot `leaf_count` == on-chain `leaf_count`), but no leaf matches the
    /// requested semantic key. Distinct from `Lagging`, which is the same miss
    /// while the store is still behind a just-sealed leaf.
    #[error("no leaf found for the requested semantic key (leaf_count {leaf_count})")]
    LeafNotFound {
        leaf_count: u64,
        rpc_context_slot: u64,
        encrypted_value_account_last_slot: Option<u64>,
    },
}

/// Builds a verified historical-access proof: the leaf for `(encrypted_value_account, handle, subject)`
/// (`ZAMA_HIST_ACCESS_LEAF_V1`). Read-only.
pub async fn build_access_proof<C: ChainFetcher, S: ProofSnapshotSource>(
    fetcher: &C,
    store: &S,
    encrypted_value_account: [u8; 32],
    handle: [u8; 32],
    subject: [u8; 32],
) -> Result<MmrProofResult, ProofError> {
    build_semantic_proof(
        fetcher,
        store,
        encrypted_value_account,
        SemanticLeafKey {
            kind: LeafKind::HistoricalAccess,
            handle,
            subject: Some(subject),
        },
    )
    .await
}

/// Builds a verified public-decrypt proof: the earliest public-decrypt leaf for `(encrypted_value_account, handle)`
/// (`ZAMA_PUBLIC_DECRYPT_LEAF_V1`). A handle can carry several public-decrypt leaves (born-public
/// plus later `make_handle_public` re-releases); any one proves publicness, and resolving to the
/// earliest is deterministic and append-stable. Read-only.
pub async fn build_public_proof<C: ChainFetcher, S: ProofSnapshotSource>(
    fetcher: &C,
    store: &S,
    encrypted_value_account: [u8; 32],
    handle: [u8; 32],
) -> Result<MmrProofResult, ProofError> {
    build_semantic_proof(
        fetcher,
        store,
        encrypted_value_account,
        SemanticLeafKey {
            kind: LeafKind::PublicDecrypt,
            handle,
            subject: None,
        },
    )
    .await
}

/// Resolves `key` to a leaf index inside one consistent SQL snapshot and builds a proof for it,
/// verified against the confirmed on-chain account. Read-only: never writes to the store.
///
/// A key that resolves to no leaf is classified against chain: while the snapshot is still
/// behind the on-chain leaf count, the miss is retryable [`ProofError::Lagging`] (ingest has not
/// caught up to a just-sealed leaf); once the snapshot is at parity with chain, the miss is
/// terminal [`ProofError::LeafNotFound`].
async fn build_semantic_proof<C: ChainFetcher, S: ProofSnapshotSource>(
    fetcher: &C,
    store: &S,
    encrypted_value_account: [u8; 32],
    key: SemanticLeafKey,
) -> Result<MmrProofResult, ProofError> {
    let on_chain = fetcher
        .get_encrypted_value_account_state(encrypted_value_account)
        .await?
        .ok_or(ProofError::LineageNotFound)?;

    let resolved = match store
        .proof_snapshot_for_leaf(encrypted_value_account, &key)
        .await
    {
        Ok(resolved) => resolved,
        Err(StoreError::SnapshotInconsistent { .. }) => {
            // Fail closed with the same wire envelope as peak divergence. The torn
            // snapshot never surfaced a durable slot, so there is no checkpoint to report.
            return Err(ProofError::CorruptStore {
                leaf_count: on_chain.leaf_count,
                rpc_context_slot: on_chain.rpc_context_slot,
                encrypted_value_account_last_slot: None,
            });
        }
        Err(err) => return Err(ProofError::Store(err)),
    };
    let Some(ResolvedProofSnapshot {
        snapshot,
        leaf_index,
    }) = resolved
    else {
        // Chain has the encrypted_value_account but the store has not ingested it yet, so this
        // encrypted_value_account has no durable slot to report.
        return Err(ProofError::Lagging {
            leaf_count: on_chain.leaf_count,
            rpc_context_slot: on_chain.rpc_context_slot,
            encrypted_value_account_last_slot: None,
        });
    };

    let Some(leaf_index) = leaf_index else {
        // The leaf for this key is not (yet) in the store. If the snapshot is still behind
        // chain, ingest may not have appended a just-sealed leaf — retryable lag. At parity the
        // store is caught up, so the key genuinely has no leaf — terminal.
        //
        // Liveness caveat: the terminal `LeafNotFound` is only reached AT parity. A nonexistent
        // key queried against a encrypted_value_account that keeps appending leaves stays perpetually behind
        // parity and returns `Lagging` on every attempt — it never becomes terminal server-side.
        // Bounding retries for a wrong key is therefore the client's job (its retry cap), not the
        // server's; the server cannot distinguish "just-sealed, not yet ingested" from "will never
        // exist" without waiting for the store to catch up to that exact chain tip.
        let encrypted_value_account_last_slot = Some(snapshot.last_slot);
        return Err(match snapshot.leaf_count.cmp(&on_chain.leaf_count) {
            std::cmp::Ordering::Equal => ProofError::LeafNotFound {
                leaf_count: on_chain.leaf_count,
                rpc_context_slot: on_chain.rpc_context_slot,
                encrypted_value_account_last_slot,
            },
            _ => ProofError::Lagging {
                leaf_count: on_chain.leaf_count,
                rpc_context_slot: on_chain.rpc_context_slot,
                encrypted_value_account_last_slot,
            },
        });
    };

    try_build_from_snapshot(&snapshot, &on_chain, leaf_index)
}

fn try_build_from_snapshot(
    snapshot: &ProofSnapshot,
    on_chain: &OnChainLineageState,
    leaf_index: u64,
) -> Result<MmrProofResult, ProofError> {
    // The snapshot is in hand, so its durable slot is the honest checkpoint for
    // every result (success or divergence) built from it.
    let encrypted_value_account_last_slot = Some(snapshot.last_slot);
    // Internal consistency: recomputed peaks must match the persisted peaks.
    let recomputed = mmr_peaks_from_leaves(&snapshot.leaves);
    if recomputed != snapshot.peaks || snapshot.leaf_count != snapshot.leaves.len() as u64 {
        return Err(ProofError::CorruptStore {
            leaf_count: on_chain.leaf_count,
            rpc_context_slot: on_chain.rpc_context_slot,
            encrypted_value_account_last_slot,
        });
    }

    match snapshot.leaf_count.cmp(&on_chain.leaf_count) {
        std::cmp::Ordering::Less => {
            return Err(ProofError::Lagging {
                leaf_count: on_chain.leaf_count,
                rpc_context_slot: on_chain.rpc_context_slot,
                encrypted_value_account_last_slot,
            });
        }
        std::cmp::Ordering::Greater => {
            // Store can briefly lead a different confirmed RPC node after ingest.
            // Treat as retryable lag, not integrity failure.
            return Err(ProofError::Lagging {
                leaf_count: on_chain.leaf_count,
                rpc_context_slot: on_chain.rpc_context_slot,
                encrypted_value_account_last_slot,
            });
        }
        std::cmp::Ordering::Equal => {}
    }

    if snapshot.peaks != on_chain.peaks {
        return Err(ProofError::CorruptStore {
            leaf_count: on_chain.leaf_count,
            rpc_context_slot: on_chain.rpc_context_slot,
            encrypted_value_account_last_slot,
        });
    }

    // `leaf_index` was resolved from a real leaf row in this same snapshot, so it is always in
    // range; a build failure here means the snapshot's leaves diverge from its metadata.
    let proof = mmr_build_proof(&snapshot.leaves, leaf_index).ok_or(ProofError::CorruptStore {
        leaf_count: on_chain.leaf_count,
        rpc_context_slot: on_chain.rpc_context_slot,
        encrypted_value_account_last_slot,
    })?;

    Ok(MmrProofResult {
        mmr_proof: Some(proof),
        leaf_count: on_chain.leaf_count,
        rpc_context_slot: on_chain.rpc_context_slot,
        encrypted_value_account_last_slot,
        verified: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    use zama_solana_acl::mmr::mmr_append;

    use crate::chain::ChainError;

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn snapshot_with_leaves(
        encrypted_value_account: [u8; 32],
        leaves: Vec<[u8; 32]>,
    ) -> ProofSnapshot {
        let peaks = mmr_peaks_from_leaves(&leaves);
        ProofSnapshot {
            encrypted_value_account,
            current_handle: None,
            subjects: vec![],
            leaf_count: leaves.len() as u64,
            peaks,
            leaves,
            last_slot: 1,
        }
    }

    struct FakeChain {
        states: Mutex<HashMap<[u8; 32], OnChainLineageState>>,
        fetches: AtomicUsize,
    }

    impl FakeChain {
        fn new() -> Self {
            Self {
                states: Mutex::new(HashMap::new()),
                fetches: AtomicUsize::new(0),
            }
        }

        fn set(&self, encrypted_value_account: [u8; 32], state: OnChainLineageState) {
            self.states
                .lock()
                .unwrap()
                .insert(encrypted_value_account, state);
        }
    }

    #[async_trait]
    impl ChainFetcher for FakeChain {
        async fn get_encrypted_value_account_state(
            &self,
            address: [u8; 32],
        ) -> Result<Option<OnChainLineageState>, ChainError> {
            self.fetches.fetch_add(1, Ordering::SeqCst);
            Ok(self.states.lock().unwrap().get(&address).cloned())
        }
    }

    /// Read-only fake: resolves a pre-registered `(snapshot, leaf_index)` per encrypted_value_account; no write /
    /// catch-up API. Semantic-key → index resolution itself is exercised by the SQL integration
    /// tests; these unit tests pin the resolved outcome and assert the classification logic.
    struct FakeStore {
        resolved: Mutex<HashMap<[u8; 32], ResolvedProofSnapshot>>,
        reads: AtomicUsize,
        inconsistent: Mutex<HashMap<[u8; 32], (u64, u64)>>,
    }

    impl FakeStore {
        fn new() -> Self {
            Self {
                resolved: Mutex::new(HashMap::new()),
                reads: AtomicUsize::new(0),
                inconsistent: Mutex::new(HashMap::new()),
            }
        }

        /// Registers a snapshot whose key resolves to `leaf_index`.
        fn insert_resolved(&self, snapshot: ProofSnapshot, leaf_index: Option<u64>) {
            self.resolved.lock().unwrap().insert(
                snapshot.encrypted_value_account,
                ResolvedProofSnapshot {
                    snapshot,
                    leaf_index,
                },
            );
        }

        /// Registers a snapshot whose key resolves to leaf 0 (the single-leaf case).
        fn insert(&self, snapshot: ProofSnapshot) {
            self.insert_resolved(snapshot, Some(0));
        }

        fn mark_inconsistent(
            &self,
            encrypted_value_account: [u8; 32],
            leaf_count: u64,
            leaf_rows: u64,
        ) {
            self.inconsistent
                .lock()
                .unwrap()
                .insert(encrypted_value_account, (leaf_count, leaf_rows));
        }
    }

    #[async_trait]
    impl ProofSnapshotSource for FakeStore {
        async fn proof_snapshot_for_leaf(
            &self,
            encrypted_value_account: [u8; 32],
            _key: &SemanticLeafKey,
        ) -> Result<Option<ResolvedProofSnapshot>, StoreError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if let Some(&(leaf_count, leaf_rows)) = self
                .inconsistent
                .lock()
                .unwrap()
                .get(&encrypted_value_account)
            {
                return Err(StoreError::SnapshotInconsistent {
                    leaf_count,
                    leaf_rows,
                });
            }
            Ok(self
                .resolved
                .lock()
                .unwrap()
                .get(&encrypted_value_account)
                .cloned())
        }
    }

    #[test]
    fn builds_verified_proof_when_snapshot_matches_chain() {
        let encrypted_value_account = pk(0x01);
        let leaf = zama_solana_acl::historical_access_leaf_commitment(
            encrypted_value_account,
            0,
            pk(0x10),
            pk(0x30),
        );
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let snapshot = snapshot_with_leaves(encrypted_value_account, vec![leaf]);
        let on_chain = OnChainLineageState {
            peaks,
            leaf_count,
            rpc_context_slot: 55,
        };
        let result = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap();
        assert!(result.verified);
        assert_eq!(result.leaf_count, 1);
        assert_eq!(result.rpc_context_slot, 55);
        // snapshot_with_leaves pins last_slot = 1.
        assert_eq!(result.encrypted_value_account_last_slot, Some(1));
        assert!(result.mmr_proof.is_some());
    }

    #[tokio::test]
    async fn build_public_proof_returns_encrypted_value_account_not_found_without_store_read() {
        let encrypted_value_account = pk(0x01);
        let chain = FakeChain::new();
        let store = FakeStore::new();
        let err = build_public_proof(&chain, &store, encrypted_value_account, pk(0x10))
            .await
            .unwrap_err();
        assert!(matches!(err, ProofError::LineageNotFound));
        assert_eq!(store.reads.load(Ordering::SeqCst), 0);
        assert_eq!(chain.fetches.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn build_public_proof_missing_leaf_at_parity_is_terminal_not_found() {
        let encrypted_value_account = pk(0x0A);
        let leaf =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            encrypted_value_account,
            OnChainLineageState {
                peaks,
                leaf_count, // == snapshot leaf_count below: store is caught up
                rpc_context_slot: 55,
            },
        );
        let store = FakeStore::new();
        // Snapshot at parity with chain (1 leaf) but the requested key resolves to nothing.
        store.insert_resolved(
            snapshot_with_leaves(encrypted_value_account, vec![leaf]),
            None,
        );

        let err = build_public_proof(&chain, &store, encrypted_value_account, pk(0xEE))
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            ProofError::LeafNotFound { leaf_count: 1, .. }
        ));
    }

    #[tokio::test]
    async fn build_public_proof_missing_leaf_behind_chain_is_lagging() {
        let encrypted_value_account = pk(0x0B);
        let leaf0 =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let leaf1 =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 1, pk(0x11));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf0).unwrap();
        mmr_append(&mut peaks, &mut leaf_count, leaf1).unwrap();

        let chain = FakeChain::new();
        chain.set(
            encrypted_value_account,
            OnChainLineageState {
                peaks,
                leaf_count, // 2 on chain
                rpc_context_slot: 55,
            },
        );
        let store = FakeStore::new();
        // Store has only leaf 0 (behind chain) and the just-sealed key isn't ingested yet.
        store.insert_resolved(
            snapshot_with_leaves(encrypted_value_account, vec![leaf0]),
            None,
        );

        let err = build_public_proof(&chain, &store, encrypted_value_account, pk(0x11))
            .await
            .unwrap_err();
        assert!(matches!(err, ProofError::Lagging { leaf_count: 2, .. }));
    }

    #[tokio::test]
    async fn build_access_proof_resolves_and_verifies() {
        let encrypted_value_account = pk(0x0C);
        let handle = pk(0x10);
        let subject = pk(0x30);
        let leaf = zama_solana_acl::historical_access_leaf_commitment(
            encrypted_value_account,
            0,
            handle,
            subject,
        );
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            encrypted_value_account,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 55,
            },
        );
        let store = FakeStore::new();
        store.insert_resolved(
            snapshot_with_leaves(encrypted_value_account, vec![leaf]),
            Some(0),
        );

        let result = build_access_proof(&chain, &store, encrypted_value_account, handle, subject)
            .await
            .unwrap();
        assert!(result.verified);
        assert_eq!(result.leaf_count, 1);
        assert!(result.mmr_proof.is_some());
    }

    #[test]
    fn returns_lagging_when_snapshot_leaf_count_is_behind() {
        let encrypted_value_account = pk(0x02);
        let leaf0 =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let leaf1 =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 1, pk(0x11));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf0).unwrap();
        mmr_append(&mut peaks, &mut leaf_count, leaf1).unwrap();

        let snapshot = snapshot_with_leaves(encrypted_value_account, vec![leaf0]);
        let on_chain = OnChainLineageState {
            peaks,
            leaf_count,
            rpc_context_slot: 55,
        };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(err, ProofError::Lagging { leaf_count: 2, .. }));
    }

    #[test]
    fn returns_lagging_when_store_is_ahead_of_rpc() {
        let encrypted_value_account = pk(0x05);
        let leaf0 =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let leaf1 =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 1, pk(0x11));
        let mut store_peaks = Vec::new();
        let mut store_count = 0u64;
        mmr_append(&mut store_peaks, &mut store_count, leaf0).unwrap();
        mmr_append(&mut store_peaks, &mut store_count, leaf1).unwrap();

        let mut chain_peaks = Vec::new();
        let mut chain_count = 0u64;
        mmr_append(&mut chain_peaks, &mut chain_count, leaf0).unwrap();

        let snapshot = snapshot_with_leaves(encrypted_value_account, vec![leaf0, leaf1]);
        let on_chain = OnChainLineageState {
            peaks: chain_peaks,
            leaf_count: chain_count,
            rpc_context_slot: 55,
        };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(err, ProofError::Lagging { leaf_count: 1, .. }));
    }

    #[test]
    fn returns_corrupt_when_equal_count_peaks_diverge() {
        let encrypted_value_account = pk(0x03);
        let leaf =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let other =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0xAA));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, other).unwrap();

        let snapshot = snapshot_with_leaves(encrypted_value_account, vec![leaf]);
        let on_chain = OnChainLineageState {
            peaks,
            leaf_count,
            rpc_context_slot: 55,
        };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(
            err,
            ProofError::CorruptStore { leaf_count: 1, .. }
        ));
    }

    #[test]
    fn returns_corrupt_when_persisted_peaks_do_not_match_leaves() {
        let encrypted_value_account = pk(0x04);
        let leaf =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let mut snapshot = snapshot_with_leaves(encrypted_value_account, vec![leaf]);
        snapshot.peaks = vec![pk(0xFF)];
        let on_chain = OnChainLineageState {
            peaks: peaks.clone(),
            leaf_count,
            rpc_context_slot: 55,
        };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(
            err,
            ProofError::CorruptStore { leaf_count: 1, .. }
        ));
    }

    #[tokio::test]
    async fn build_proof_maps_snapshot_inconsistency_to_corrupt() {
        let encrypted_value_account = pk(0x06);
        let leaf =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            encrypted_value_account,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 55,
            },
        );
        let store = FakeStore::new();
        store.mark_inconsistent(encrypted_value_account, 1, 0);

        let err = build_public_proof(&chain, &store, encrypted_value_account, pk(0x10))
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            ProofError::CorruptStore { leaf_count: 1, .. }
        ));
        assert_eq!(store.reads.load(Ordering::SeqCst), 1);
        assert_eq!(chain.fetches.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn build_proof_is_read_only_snapshot_plus_chain_fetch() {
        let encrypted_value_account = pk(0x07);
        let leaf =
            zama_solana_acl::public_decrypt_leaf_commitment(encrypted_value_account, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            encrypted_value_account,
            OnChainLineageState {
                peaks: peaks.clone(),
                leaf_count,
                rpc_context_slot: 55,
            },
        );
        let store = FakeStore::new();
        store.insert(snapshot_with_leaves(encrypted_value_account, vec![leaf]));

        let result = build_public_proof(&chain, &store, encrypted_value_account, pk(0x10))
            .await
            .unwrap();
        assert!(result.verified);
        // FakeStore has no write/catch-up API; only one snapshot read occurred.
        assert_eq!(store.reads.load(Ordering::SeqCst), 1);
        assert_eq!(chain.fetches.load(Ordering::SeqCst), 1);
    }
}
