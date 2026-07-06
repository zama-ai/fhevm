//! Append-only per-lineage leaf-event log, plus the ingestion cursor and each
//! lineage's replay state (handle + subjects/roles), persisted so a poll cycle
//! can resume without replaying the whole chain history.
//!
//! Backend: a single JSON file behind an async `RwLock`, written through on
//! every mutation. The relayer already carries Postgres via `sqlx`, but this
//! store is a rebuildable ingestion cache (every byte in it can be
//! reconstructed by replaying `zama-host` transactions from genesis) rather
//! than durable business state, so a file avoids a migration for a cache and
//! keeps the proof service usable standalone (e.g. in tests, or a sidecar
//! deployment without its own Postgres schema). Swap for a `sqlx`-backed
//! `LeafStore` impl behind this same trait if/when this graduates to owning
//! non-rebuildable state.

use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::warn;
use zama_solana_acl::lineage::LineageEvent;

use crate::solana_proof::replay::LineageReplayState;

#[derive(thiserror::Error, Debug)]
pub enum StoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("(de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Ingestion progress marker: the last fully-processed signature/slot, so a
/// restart resumes rather than replaying from genesis.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cursor {
    pub last_signature: Option<String>,
    pub last_slot: u64,
}

#[async_trait]
pub trait LeafStore: Send + Sync {
    async fn append_events(
        &self,
        lineage: [u8; 32],
        events: &[LineageEvent],
    ) -> Result<(), StoreError>;

    async fn get_events(&self, lineage: [u8; 32]) -> Result<Vec<LineageEvent>, StoreError>;

    async fn get_cursor(&self) -> Result<Option<Cursor>, StoreError>;

    async fn set_cursor(&self, cursor: Cursor) -> Result<(), StoreError>;

    /// The replayer's tracked (current_handle, subjects+roles) for a lineage,
    /// needed to correctly interpret the *next* instruction without replaying
    /// history from genesis on every poll.
    async fn get_replay_state(
        &self,
        lineage: [u8; 32],
    ) -> Result<Option<LineageReplayState>, StoreError>;

    async fn set_replay_state(
        &self,
        lineage: [u8; 32],
        state: LineageReplayState,
    ) -> Result<(), StoreError>;

    /// Signatures already ingested for `lineage`, used to dedupe a targeted
    /// catch-up poll against the main program-wide poll.
    async fn get_seen_signatures(&self, lineage: [u8; 32]) -> Result<Vec<String>, StoreError>;

    async fn mark_signature_seen(
        &self,
        lineage: [u8; 32],
        signature: &str,
    ) -> Result<(), StoreError>;
}

// --- wire DTOs (LineageEvent has no Serialize; mirror it locally) ---

#[derive(Clone, Debug, Serialize, Deserialize)]
enum LineageEventDto {
    HandleSuperseded {
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
    },
    MarkedPublic {
        handle: [u8; 32],
    },
}

impl From<&LineageEvent> for LineageEventDto {
    fn from(e: &LineageEvent) -> Self {
        match e {
            LineageEvent::HandleSuperseded {
                previous_handle,
                previous_subjects,
            } => LineageEventDto::HandleSuperseded {
                previous_handle: *previous_handle,
                previous_subjects: previous_subjects.clone(),
            },
            LineageEvent::MarkedPublic { handle } => {
                LineageEventDto::MarkedPublic { handle: *handle }
            }
        }
    }
}

impl From<LineageEventDto> for LineageEvent {
    fn from(dto: LineageEventDto) -> Self {
        match dto {
            LineageEventDto::HandleSuperseded {
                previous_handle,
                previous_subjects,
            } => LineageEvent::handle_superseded(previous_handle, &previous_subjects),
            LineageEventDto::MarkedPublic { handle } => LineageEvent::MarkedPublic { handle },
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct FileContents {
    cursor: Option<Cursor>,
    #[serde(default)]
    events: HashMap<String, Vec<LineageEventDto>>,
    #[serde(default)]
    replay_states: HashMap<String, LineageReplayStateDto>,
    #[serde(default)]
    seen_signatures: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LineageReplayStateDto {
    current_handle: [u8; 32],
    subjects: Vec<([u8; 32], u8)>,
}

impl From<&LineageReplayState> for LineageReplayStateDto {
    fn from(s: &LineageReplayState) -> Self {
        Self {
            current_handle: s.current_handle,
            subjects: s.subjects.clone(),
        }
    }
}

impl From<LineageReplayStateDto> for LineageReplayState {
    fn from(dto: LineageReplayStateDto) -> Self {
        Self {
            current_handle: dto.current_handle,
            subjects: dto.subjects,
        }
    }
}

fn key(lineage: [u8; 32]) -> String {
    hex::encode(lineage)
}

fn path_with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut raw = path.as_os_str().to_os_string();
    raw.push(suffix);
    PathBuf::from(raw)
}

/// File-backed `LeafStore`. Rebuildable cache: safe to delete and re-ingest
/// from `start_slot`/`start_signature` in config.
pub struct FileLeafStore {
    path: PathBuf,
    state: Arc<RwLock<FileContents>>,
}

impl FileLeafStore {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let contents = match tokio::fs::read(&path).await {
            Ok(bytes) if bytes.is_empty() => FileContents::default(),
            Ok(bytes) => match serde_json::from_slice(&bytes) {
                Ok(contents) => contents,
                Err(error) => {
                    let corrupt_path = path_with_suffix(&path, ".corrupt");
                    warn!(
                        path = %path.display(),
                        corrupt_path = %corrupt_path.display(),
                        "Solana proof leaf store is corrupt; quarantining and starting empty: {error}"
                    );
                    if let Err(rename_error) = tokio::fs::rename(&path, &corrupt_path).await {
                        warn!(
                            path = %path.display(),
                            corrupt_path = %corrupt_path.display(),
                            "Failed to quarantine corrupt Solana proof leaf store: {rename_error}"
                        );
                    }
                    FileContents::default()
                }
            },
            Err(error) if error.kind() == ErrorKind::NotFound => FileContents::default(),
            Err(error) => return Err(StoreError::Io(error)),
        };
        Ok(Self {
            path,
            state: Arc::new(RwLock::new(contents)),
        })
    }

    async fn flush(&self, contents: &FileContents) -> Result<(), StoreError> {
        let bytes = serde_json::to_vec_pretty(contents)?;
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let temp_path = path_with_suffix(&self.path, ".tmp");
        tokio::fs::write(&temp_path, bytes).await?;
        tokio::fs::rename(&temp_path, &self.path).await?;
        Ok(())
    }

    async fn update_state(&self, update: impl FnOnce(&mut FileContents)) -> Result<(), StoreError> {
        let mut guard = self.state.write().await;
        let mut snapshot = guard.clone();
        update(&mut snapshot);
        self.flush(&snapshot).await?;
        *guard = snapshot;
        Ok(())
    }
}

#[async_trait]
impl LeafStore for FileLeafStore {
    async fn append_events(
        &self,
        lineage: [u8; 32],
        events: &[LineageEvent],
    ) -> Result<(), StoreError> {
        let lineage_key = key(lineage);
        self.update_state(|contents| {
            contents
                .events
                .entry(lineage_key)
                .or_default()
                .extend(events.iter().map(LineageEventDto::from));
        })
        .await
    }

    async fn get_events(&self, lineage: [u8; 32]) -> Result<Vec<LineageEvent>, StoreError> {
        let guard = self.state.read().await;
        Ok(guard
            .events
            .get(&key(lineage))
            .map(|events| events.iter().cloned().map(LineageEvent::from).collect())
            .unwrap_or_default())
    }

    async fn get_cursor(&self) -> Result<Option<Cursor>, StoreError> {
        Ok(self.state.read().await.cursor.clone())
    }

    async fn set_cursor(&self, cursor: Cursor) -> Result<(), StoreError> {
        self.update_state(|contents| {
            contents.cursor = Some(cursor);
        })
        .await
    }

    async fn get_replay_state(
        &self,
        lineage: [u8; 32],
    ) -> Result<Option<LineageReplayState>, StoreError> {
        Ok(self
            .state
            .read()
            .await
            .replay_states
            .get(&key(lineage))
            .cloned()
            .map(LineageReplayState::from))
    }

    async fn set_replay_state(
        &self,
        lineage: [u8; 32],
        state: LineageReplayState,
    ) -> Result<(), StoreError> {
        let lineage_key = key(lineage);
        self.update_state(|contents| {
            contents
                .replay_states
                .insert(lineage_key, LineageReplayStateDto::from(&state));
        })
        .await
    }

    async fn get_seen_signatures(&self, lineage: [u8; 32]) -> Result<Vec<String>, StoreError> {
        Ok(self
            .state
            .read()
            .await
            .seen_signatures
            .get(&key(lineage))
            .cloned()
            .unwrap_or_default())
    }

    async fn mark_signature_seen(
        &self,
        lineage: [u8; 32],
        signature: &str,
    ) -> Result<(), StoreError> {
        let lineage_key = key(lineage);
        self.update_state(|contents| {
            contents
                .seen_signatures
                .entry(lineage_key)
                .or_default()
                .push(signature.to_string());
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    #[tokio::test]
    async fn append_and_read_back_events_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();
        let lineage = pk(1);
        let events = vec![
            LineageEvent::handle_superseded(pk(0x10), &[pk(0x30)]),
            LineageEvent::MarkedPublic { handle: pk(0x11) },
        ];
        store.append_events(lineage, &events).await.unwrap();
        let read_back = store.get_events(lineage).await.unwrap();
        assert_eq!(read_back, events);
    }

    #[tokio::test]
    async fn cursor_persists_across_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("leaves.json");
        {
            let store = FileLeafStore::open(&path).await.unwrap();
            assert_eq!(store.get_cursor().await.unwrap(), None);
            store
                .set_cursor(Cursor {
                    last_signature: Some("sig123".to_string()),
                    last_slot: 42,
                })
                .await
                .unwrap();
        }
        let reopened = FileLeafStore::open(&path).await.unwrap();
        assert_eq!(
            reopened.get_cursor().await.unwrap(),
            Some(Cursor {
                last_signature: Some("sig123".to_string()),
                last_slot: 42,
            })
        );
    }

    #[tokio::test]
    async fn corrupt_store_is_quarantined_and_reopened_empty() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("leaves.json");
        tokio::fs::write(&path, b"{not-json").await.unwrap();

        let store = FileLeafStore::open(&path).await.unwrap();
        assert_eq!(store.get_cursor().await.unwrap(), None);
        assert!(!path.exists());
        assert!(path_with_suffix(&path, ".corrupt").exists());
    }

    #[tokio::test]
    async fn flush_failure_does_not_advance_cursor_in_memory() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("leaves.json");
        let store = FileLeafStore::open(&path).await.unwrap();
        tokio::fs::create_dir(path_with_suffix(&path, ".tmp"))
            .await
            .unwrap();

        let error = store
            .set_cursor(Cursor {
                last_signature: Some("sig123".to_string()),
                last_slot: 42,
            })
            .await
            .unwrap_err();
        assert!(matches!(error, StoreError::Io(_)));
        assert_eq!(store.get_cursor().await.unwrap(), None);
    }

    #[tokio::test]
    async fn replay_state_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();
        let lineage = pk(2);
        assert_eq!(store.get_replay_state(lineage).await.unwrap(), None);
        let state = LineageReplayState {
            current_handle: pk(0x10),
            subjects: vec![(pk(0x30), 1)],
        };
        store
            .set_replay_state(lineage, state.clone())
            .await
            .unwrap();
        assert_eq!(store.get_replay_state(lineage).await.unwrap(), Some(state));
    }

    #[tokio::test]
    async fn seen_signatures_dedupe_catch_up_polls() {
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();
        let lineage = pk(3);
        assert!(store.get_seen_signatures(lineage).await.unwrap().is_empty());
        store.mark_signature_seen(lineage, "sig-a").await.unwrap();
        store.mark_signature_seen(lineage, "sig-b").await.unwrap();
        assert_eq!(
            store.get_seen_signatures(lineage).await.unwrap(),
            vec!["sig-a".to_string(), "sig-b".to_string()]
        );
    }
}
