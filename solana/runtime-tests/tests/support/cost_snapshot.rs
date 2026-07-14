//! Rolling cost snapshot over Mollusk fixtures.
//!
//! Records the Solana runtime cost of representative instruction profiles —
//! compute units, unique account count, instruction-data size, and the count
//! and total data size of the CPIs the instruction issues — and compares them
//! against the committed snapshot under `cost-snapshots/`.
//! Drift in either direction fails the dedicated `cost_snapshot_*` tests:
//! costlier is a regression, cheaper invalidates design assumptions derived
//! from the old number (transaction packing, per-leg budgets), and both
//! deserve a reviewed snapshot update in the same diff. Behavior tests never
//! depend on these numbers.
//!
//! Accept a new baseline from `solana/` with:
//!
//! ```text
//! bash scripts/update-cost-snapshots.sh
//! ```
//!
//! That script checks the CI-pinned Solana/Anchor versions, cleans, rebuilds
//! SBF artifacts, clears existing snapshot JSON (so orphaned profiles cannot
//! linger), and rewrites the baselines. Prefer it over setting
//! `ZAMA_UPDATE_COST_SNAPSHOT=1` by hand (the env gate remains for escape-hatch
//! use but skips the toolchain/clean/orphan-clear guardrails). Costs are exact
//! for the pinned toolchain.
//!
//! Profiles use fixed fixture keys because on-chain PDA bump searches are part
//! of the measured compute: absolute values therefore include an
//! arbitrary-but-stable bump-search cost (roughly ±10% across key choices) and
//! Mollusk is not a mainnet cost oracle. The signal is the delta between
//! commits, not the absolute number.
//!
//! Update mode inserts or overwrites the current profile only; it never deletes
//! keys. Full regenerations go through `scripts/update-cost-snapshots.sh`, which
//! clears the JSON files first so renamed/deleted profiles do not linger.

use mollusk_svm::result::InstructionResult;
use serde::{Deserialize, Serialize};
use solana_sdk::instruction::Instruction;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const UPDATE_ENV: &str = "ZAMA_UPDATE_COST_SNAPSHOT";

/// Serializes read-modify-write cycles when several tests in this binary
/// update the same snapshot file. Distinct test binaries use distinct files.
static SNAPSHOT_FILE_LOCK: Mutex<()> = Mutex::new(());

/// Asserts the measured cost of one instruction profile against the snapshot
/// file `cost-snapshots/<snapshot>.json`, or rewrites the entry when
/// [`UPDATE_ENV`] is set.
pub fn assert_cost_snapshot(
    snapshot: &str,
    profile: &str,
    instruction: &Instruction,
    result: &InstructionResult,
) {
    let measured = measure(instruction, result);
    let path = snapshot_path(snapshot);
    // A panic in another cost test must not cascade into poison errors here.
    let _guard = SNAPSHOT_FILE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let update = match std::env::var_os(UPDATE_ENV) {
        None => false,
        Some(value) if value == "1" => true,
        Some(value) if value == "0" || value.is_empty() => false,
        Some(value) => panic!(
            "{UPDATE_ENV} must be unset, \"0\", or \"1\" (got {value:?}); use \
             `bash scripts/update-cost-snapshots.sh` to regenerate"
        ),
    };

    if update {
        let mut entries = match load_entries(&path) {
            LoadEntries::Missing => BTreeMap::new(),
            LoadEntries::Ok(entries) => entries,
            LoadEntries::Invalid(err) => {
                eprintln!(
                    "warning: invalid cost snapshot {} ({err}); regenerating from scratch",
                    path.display()
                );
                BTreeMap::new()
            }
        };
        entries.insert(profile.to_string(), measured);
        write_entries(&path, &entries);
        return;
    }

    let entries = match load_entries(&path) {
        LoadEntries::Missing => panic!(
            "cost snapshot file {} is missing; generate it with {UPDATE_ENV}=1 and commit it",
            path.display()
        ),
        LoadEntries::Ok(entries) => entries,
        LoadEntries::Invalid(err) => panic!(
            "invalid cost snapshot {}: {err}; fix the file or regenerate with {UPDATE_ENV}=1",
            path.display()
        ),
    };
    let expected = entries.get(profile).unwrap_or_else(|| {
        panic!(
            "profile {profile:?} is missing from {}; record it with {UPDATE_ENV}=1 and commit \
             the update",
            path.display()
        )
    });

    let mut failures = Vec::new();
    if measured.unique_accounts != expected.unique_accounts {
        failures.push(format!(
            "unique accounts changed: {} -> {}",
            expected.unique_accounts, measured.unique_accounts
        ));
    }
    if measured.instruction_data_bytes != expected.instruction_data_bytes {
        failures.push(format!(
            "instruction data bytes changed: {} -> {}",
            expected.instruction_data_bytes, measured.instruction_data_bytes
        ));
    }
    if measured.cpi_instructions != expected.cpi_instructions {
        failures.push(format!(
            "CPI instruction count changed: {} -> {}",
            expected.cpi_instructions, measured.cpi_instructions
        ));
    }
    if measured.cpi_instruction_data_bytes != expected.cpi_instruction_data_bytes {
        failures.push(format!(
            "CPI instruction data bytes changed: {} -> {}",
            expected.cpi_instruction_data_bytes, measured.cpi_instruction_data_bytes
        ));
    }
    if measured.compute_units != expected.compute_units {
        let direction = if measured.compute_units > expected.compute_units {
            "regression"
        } else {
            "improvement — re-check packing decisions derived from the old cost"
        };
        failures.push(format!(
            "compute units changed: {} -> {} ({direction})",
            expected.compute_units, measured.compute_units
        ));
    }
    assert!(
        failures.is_empty(),
        "cost snapshot mismatch for {profile:?} in {}:\n  {}\naccept intentional changes with \
         `bash scripts/update-cost-snapshots.sh` (or {UPDATE_ENV}=1) and commit the updated snapshot",
        path.display(),
        failures.join("\n  ")
    );
}

/// Measured cost of one instruction profile.
///
/// `unique_accounts` is a static property of the instruction shape (unique
/// account-meta pubkeys plus the program id), not a count of accounts loaded
/// at runtime.
///
/// `cpi_instructions` / `cpi_instruction_data_bytes` cover every inner
/// instruction at any stack depth, so nested CPIs (token -> host -> event
/// batch) are all included. The dominant contributor today is the batched
/// event CPI that carries the FHE operation records to the coprocessor.
#[derive(Clone, Copy, Deserialize, Serialize)]
struct Cost {
    compute_units: u64,
    unique_accounts: usize,
    instruction_data_bytes: usize,
    cpi_instructions: usize,
    cpi_instruction_data_bytes: usize,
}

fn measure(instruction: &Instruction, result: &InstructionResult) -> Cost {
    let mut accounts: HashSet<_> = instruction
        .accounts
        .iter()
        .map(|meta| meta.pubkey)
        .collect();
    accounts.insert(instruction.program_id);
    Cost {
        compute_units: result.compute_units_consumed,
        unique_accounts: accounts.len(),
        instruction_data_bytes: instruction.data.len(),
        cpi_instructions: result.inner_instructions.len(),
        cpi_instruction_data_bytes: result
            .inner_instructions
            .iter()
            .map(|inner| inner.instruction.data.len())
            .sum(),
    }
}

fn snapshot_path(snapshot: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cost-snapshots")
        .join(format!("{snapshot}.json"))
}

enum LoadEntries {
    Missing,
    Ok(BTreeMap<String, Cost>),
    Invalid(String),
}

fn load_entries(path: &Path) -> LoadEntries {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return LoadEntries::Missing,
        Err(err) => {
            return LoadEntries::Invalid(format!("failed to read {}: {err}", path.display()))
        }
    };
    match serde_json::from_str(&text) {
        Ok(entries) => LoadEntries::Ok(entries),
        Err(err) => LoadEntries::Invalid(err.to_string()),
    }
}

fn write_entries(path: &Path, entries: &BTreeMap<String, Cost>) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut text = serde_json::to_string_pretty(entries).unwrap();
    text.push('\n');
    std::fs::write(path, text).unwrap();
}
