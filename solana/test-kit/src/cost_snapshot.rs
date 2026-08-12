//! Rolling limits snapshot over Mollusk fixtures.
//!
//! One JSON document per test suite under the calling crate's `cost-snapshots/`
//! directory, with four sections:
//!
//! - `toolchain` — the versions the numbers were minted under (`solana`,
//!   `anchor`, `rustc`), captured at regeneration time. Provenance for review:
//!   a snapshot regenerated under a divergent toolchain shows up in the diff
//!   instead of silently shifting every number. Never compared at test time.
//! - `ceilings` — the runtime and policy limits the measurements are taken
//!   against, each with `value`, `extendable`, and a note saying how (or why
//!   not). This is reference data, asserted verbatim so a Solana upgrade or a
//!   policy change has to arrive as a reviewed snapshot diff.
//! - `boundaries` — capacity walls that cannot be observed per run, only
//!   probed: `max_ok` / `first_fail` / `limited_by`, plus the full cost of the
//!   `max_ok` run. `limited_by` names the wall the first failure actually hit
//!   (heap, compute units, instruction trace, …) so a boundary can never
//!   silently converge on the wrong limit.
//! - `measurements` — per-profile instruction costs: compute units, unique
//!   account count, instruction-data size, and the count and total data size
//!   of the CPIs the instruction issues.
//!
//! Drift in either direction fails the dedicated `cost_snapshot_*` tests:
//! costlier is a regression, cheaper invalidates design assumptions derived
//! from the old number (transaction packing, per-phase budgets), and both
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

/// Serializes read-modify-write cycles when several tests in one binary
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

    if update_requested() {
        record(&path, |document| {
            document.measurements.insert(profile.to_string(), measured);
        });
        return;
    }

    let document = document_for_compare(&path);
    let expected = document.measurements.get(profile).unwrap_or_else(|| {
        panic!(
            "profile {profile:?} is missing from {}; record it with {UPDATE_ENV}=1 and commit \
             the update",
            path.display()
        )
    });

    let failures = cost_failures(expected, &measured);
    assert_snapshot_matches(&path, &format!("cost profile {profile:?}"), failures);
}

/// A probed capacity wall: the largest size that executes, the first that does
/// not, and the name of the limit the first failure actually hit.
pub struct Boundary {
    pub max_ok: u64,
    pub first_fail: u64,
    /// Which wall killed `first_fail` — e.g. `heap`, `compute_units`,
    /// `instruction_trace`, or a policy cap. A boundary whose wall changes is
    /// as much of a design event as one whose position moves.
    pub limited_by: String,
}

/// Asserts a probed boundary against the snapshot, recording alongside it the
/// full cost of the `max_ok` run so the boundary carries its own evidence
/// (compute units and byte sizes at the wall).
pub fn assert_boundary_snapshot(
    snapshot: &str,
    profile: &str,
    boundary: &Boundary,
    instruction_at_max_ok: &Instruction,
    result_at_max_ok: &InstructionResult,
) {
    let entry = BoundaryEntry {
        max_ok: boundary.max_ok,
        first_fail: boundary.first_fail,
        limited_by: boundary.limited_by.clone(),
        cost_at_max_ok: measure(instruction_at_max_ok, result_at_max_ok),
    };
    let path = snapshot_path(snapshot);
    let _guard = SNAPSHOT_FILE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    if update_requested() {
        record(&path, |document| {
            document.boundaries.insert(profile.to_string(), entry);
        });
        return;
    }

    let document = document_for_compare(&path);
    let expected = document.boundaries.get(profile).unwrap_or_else(|| {
        panic!(
            "boundary {profile:?} is missing from {}; record it with {UPDATE_ENV}=1 and commit \
             the update",
            path.display()
        )
    });

    let mut failures = Vec::new();
    if entry.max_ok != expected.max_ok {
        let direction = if entry.max_ok > expected.max_ok {
            "headroom gained — re-check caps derived from the old boundary"
        } else {
            "regression"
        };
        failures.push(format!(
            "max_ok changed: {} -> {} ({direction})",
            expected.max_ok, entry.max_ok
        ));
    }
    if entry.first_fail != expected.first_fail {
        failures.push(format!(
            "first_fail changed: {} -> {}",
            expected.first_fail, entry.first_fail
        ));
    }
    if entry.limited_by != expected.limited_by {
        failures.push(format!(
            "the binding wall changed: {} -> {}",
            expected.limited_by, entry.limited_by
        ));
    }
    failures.extend(cost_failures(&expected.cost_at_max_ok, &entry.cost_at_max_ok));
    assert_snapshot_matches(&path, &format!("boundary {profile:?}"), failures);
}

/// A runtime or policy limit the suite's measurements are taken against.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ceiling {
    pub value: u64,
    /// Whether a transaction can raise it — and the note says how, or why not.
    pub extendable: bool,
    pub note: String,
}

/// Asserts the suite's full ceilings block verbatim against the snapshot, or
/// replaces it wholesale when [`UPDATE_ENV`] is set. Whole-block semantics:
/// the caller is the single source of the list, so removed ceilings do not
/// linger in the JSON.
pub fn assert_ceilings_snapshot(snapshot: &str, ceilings: &BTreeMap<String, Ceiling>) {
    let path = snapshot_path(snapshot);
    let _guard = SNAPSHOT_FILE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    if update_requested() {
        record(&path, |document| {
            document.ceilings = ceilings.clone();
        });
        return;
    }

    let document = document_for_compare(&path);
    let mut failures = Vec::new();
    for (name, expected) in &document.ceilings {
        match ceilings.get(name) {
            None => failures.push(format!("ceiling {name:?} was removed")),
            Some(declared) if declared != expected => {
                failures.push(format!(
                    "ceiling {name:?} changed: value {} -> {}, extendable {} -> {}",
                    expected.value, declared.value, expected.extendable, declared.extendable
                ));
            }
            Some(_) => {}
        }
    }
    for name in ceilings.keys() {
        if !document.ceilings.contains_key(name) {
            failures.push(format!("ceiling {name:?} is new"));
        }
    }
    assert_snapshot_matches(&path, "ceilings block", failures);
}

/// Measured cost of one instruction profile.
///
/// `unique_accounts` is a static property of the instruction shape (unique
/// account-meta pubkeys plus the program id), not a count of accounts loaded
/// at runtime.
///
/// The CPI metrics cover every inner instruction at any stack depth, including
/// application-to-host and host-issued nested calls. The total captures the
/// aggregate payload while the maximum tracks the per-instruction runtime limit.
#[derive(Clone, Copy, Deserialize, Serialize)]
struct Cost {
    compute_units: u64,
    unique_accounts: usize,
    instruction_data_bytes: usize,
    cpi_instructions: usize,
    total_cpi_instruction_data_bytes: usize,
    max_cpi_instruction_data_bytes: usize,
}

/// A stored boundary: the probed wall plus the cost evidence of its `max_ok` run.
#[derive(Deserialize, Serialize)]
struct BoundaryEntry {
    max_ok: u64,
    first_fail: u64,
    limited_by: String,
    cost_at_max_ok: Cost,
}

/// The whole snapshot file. Sections are independent; absent sections are
/// omitted from the JSON rather than serialized empty.
#[derive(Default, Deserialize, Serialize)]
struct SnapshotDocument {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    toolchain: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    ceilings: BTreeMap<String, Ceiling>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    boundaries: BTreeMap<String, BoundaryEntry>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    measurements: BTreeMap<String, Cost>,
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
        total_cpi_instruction_data_bytes: result
            .inner_instructions
            .iter()
            .map(|inner| inner.instruction.data.len())
            .sum(),
        max_cpi_instruction_data_bytes: result
            .inner_instructions
            .iter()
            .map(|inner| inner.instruction.data.len())
            .max()
            .unwrap_or(0),
    }
}

fn cost_failures(expected: &Cost, measured: &Cost) -> Vec<String> {
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
    if measured.total_cpi_instruction_data_bytes != expected.total_cpi_instruction_data_bytes {
        failures.push(format!(
            "total CPI instruction data bytes changed: {} -> {}",
            expected.total_cpi_instruction_data_bytes, measured.total_cpi_instruction_data_bytes
        ));
    }
    if measured.max_cpi_instruction_data_bytes != expected.max_cpi_instruction_data_bytes {
        failures.push(format!(
            "maximum CPI instruction data bytes changed: {} -> {}",
            expected.max_cpi_instruction_data_bytes, measured.max_cpi_instruction_data_bytes
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
    failures
}

fn assert_snapshot_matches(path: &Path, subject: &str, failures: Vec<String>) {
    assert!(
        failures.is_empty(),
        "snapshot mismatch for {subject} in {}:\n  {}\naccept intentional changes with \
         `bash scripts/update-cost-snapshots.sh` (or {UPDATE_ENV}=1) and commit the updated snapshot",
        path.display(),
        failures.join("\n  ")
    );
}

fn update_requested() -> bool {
    match std::env::var_os(UPDATE_ENV) {
        None => false,
        Some(value) if value == "1" => true,
        Some(value) if value == "0" || value.is_empty() => false,
        Some(value) => panic!(
            "{UPDATE_ENV} must be unset, \"0\", or \"1\" (got {value:?}); use \
             `bash scripts/update-cost-snapshots.sh` to regenerate"
        ),
    }
}

/// Update-mode read-modify-write: loads the document (tolerating a missing or
/// unreadable file by starting fresh), refreshes the toolchain header, applies
/// the caller's section edit, and writes the result.
fn record(path: &Path, mutate: impl FnOnce(&mut SnapshotDocument)) {
    let mut document = match load_document(path) {
        LoadDocument::Missing => SnapshotDocument::default(),
        LoadDocument::Ok(document) => document,
        LoadDocument::Invalid(err) => {
            eprintln!(
                "warning: invalid cost snapshot {} ({err}); regenerating from scratch",
                path.display()
            );
            SnapshotDocument::default()
        }
    };
    document.toolchain = toolchain_fingerprint();
    mutate(&mut document);
    write_document(path, &document);
}

fn document_for_compare(path: &Path) -> SnapshotDocument {
    match load_document(path) {
        LoadDocument::Missing => panic!(
            "cost snapshot file {} is missing; generate it with {UPDATE_ENV}=1 and commit it",
            path.display()
        ),
        LoadDocument::Ok(document) => document,
        LoadDocument::Invalid(err) => panic!(
            "invalid cost snapshot {}: {err}; fix the file or regenerate with {UPDATE_ENV}=1",
            path.display()
        ),
    }
}

/// The versions the snapshot was minted under. Captured only in update mode
/// (compare mode never shells out); a missing tool records as "unavailable"
/// rather than failing the regeneration.
fn toolchain_fingerprint() -> BTreeMap<String, String> {
    fn version_line(tool: &str) -> String {
        std::process::Command::new(tool)
            .arg("--version")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .ok()
                    .and_then(|text| text.lines().next().map(str::to_string))
            })
            .unwrap_or_else(|| "unavailable".to_string())
    }
    [
        ("anchor".to_string(), version_line("anchor")),
        ("rustc".to_string(), version_line("rustc")),
        ("solana".to_string(), version_line("solana")),
    ]
    .into_iter()
    .collect()
}

fn snapshot_path(snapshot: &str) -> PathBuf {
    // Snapshots live in the consuming test crate, not in the kit. The `env!` macro would bake in
    // the kit's own directory at compile time; the runtime variable names the crate whose test
    // target is running, which is where the committed snapshots live.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("cost snapshots resolve against CARGO_MANIFEST_DIR; run tests through cargo");
    PathBuf::from(manifest_dir)
        .join("cost-snapshots")
        .join(format!("{snapshot}.json"))
}

enum LoadDocument {
    Missing,
    Ok(SnapshotDocument),
    Invalid(String),
}

fn load_document(path: &Path) -> LoadDocument {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return LoadDocument::Missing,
        Err(err) => {
            return LoadDocument::Invalid(format!("failed to read {}: {err}", path.display()))
        }
    };
    match serde_json::from_str(&text) {
        Ok(document) => LoadDocument::Ok(document),
        Err(err) => LoadDocument::Invalid(err.to_string()),
    }
}

fn write_document(path: &Path, document: &SnapshotDocument) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut text = serde_json::to_string_pretty(document).unwrap();
    text.push('\n');
    std::fs::write(path, text).unwrap();
}
