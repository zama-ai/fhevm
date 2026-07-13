//! Rolling cost snapshot over Mollusk fixtures.
//!
//! Records the Solana runtime cost of representative instruction profiles —
//! compute units, unique account count, and instruction-data size — and
//! compares them against the committed snapshot under `cost-snapshots/`.
//! Drift in either direction fails the dedicated `cost_snapshot_*` tests:
//! costlier is a regression, cheaper invalidates design assumptions derived
//! from the old number (transaction packing, per-leg budgets), and both
//! deserve a reviewed snapshot update in the same diff. Behavior tests never
//! depend on these numbers.
//!
//! Accept a new baseline with:
//!
//! ```text
//! ZAMA_UPDATE_COST_SNAPSHOT=1 cargo test -p zama-solana-runtime-tests cost_snapshot_
//! ```
//!
//! and commit the updated JSON. Costs are deterministic for a pinned
//! toolchain; the compute-unit band only absorbs cross-toolchain rebuild
//! variation. Account count and data size are exact.
//!
//! Profiles use fixed fixture keys because on-chain PDA bump searches are part
//! of the measured compute: absolute values therefore include an
//! arbitrary-but-stable bump-search cost (roughly ±10% across key choices) and
//! Mollusk is not a mainnet cost oracle. The signal is the delta between
//! commits, not the absolute number.

use mollusk_svm::result::InstructionResult;
use solana_sdk::instruction::Instruction;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;

/// Allowed relative compute-unit drift before the check fails.
const COMPUTE_UNIT_TOLERANCE_PCT: u64 = 2;

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

    if std::env::var_os(UPDATE_ENV).is_some() {
        let mut entries = read_entries(&path).unwrap_or_default();
        entries.insert(profile.to_string(), measured);
        write_entries(&path, &entries);
        return;
    }

    let entries = read_entries(&path).unwrap_or_else(|| {
        panic!(
            "cost snapshot file {} is missing; generate it with {UPDATE_ENV}=1 and commit it",
            path.display()
        )
    });
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
    let drift = measured.compute_units.abs_diff(expected.compute_units);
    if drift * 100 > expected.compute_units * COMPUTE_UNIT_TOLERANCE_PCT {
        let direction = if measured.compute_units > expected.compute_units {
            "regression"
        } else {
            "improvement — re-check packing decisions derived from the old cost"
        };
        failures.push(format!(
            "compute units drifted beyond {COMPUTE_UNIT_TOLERANCE_PCT}%: {} -> {} ({direction})",
            expected.compute_units, measured.compute_units
        ));
    }
    assert!(
        failures.is_empty(),
        "cost snapshot mismatch for {profile:?} in {}:\n  {}\naccept intentional changes with \
         {UPDATE_ENV}=1 and commit the updated snapshot",
        path.display(),
        failures.join("\n  ")
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cost {
    compute_units: u64,
    unique_accounts: usize,
    instruction_data_bytes: usize,
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
    }
}

fn snapshot_path(snapshot: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cost-snapshots")
        .join(format!("{snapshot}.json"))
}

fn read_entries(path: &PathBuf) -> Option<std::collections::BTreeMap<String, Cost>> {
    let text = std::fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|err| panic!("invalid cost snapshot {}: {err}", path.display()));
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("cost snapshot {} is not a JSON object", path.display()));
    let mut entries = std::collections::BTreeMap::new();
    for (profile, cost) in object {
        let field = |name: &str| {
            cost.get(name)
                .and_then(serde_json::Value::as_u64)
                .unwrap_or_else(|| {
                    panic!(
                        "cost snapshot {} profile {profile:?} is missing field {name:?}",
                        path.display()
                    )
                })
        };
        entries.insert(
            profile.clone(),
            Cost {
                compute_units: field("compute_units"),
                unique_accounts: field("unique_accounts") as usize,
                instruction_data_bytes: field("instruction_data_bytes") as usize,
            },
        );
    }
    Some(entries)
}

fn write_entries(path: &PathBuf, entries: &std::collections::BTreeMap<String, Cost>) {
    let mut object = serde_json::Map::new();
    for (profile, cost) in entries {
        object.insert(
            profile.clone(),
            serde_json::json!({
                "compute_units": cost.compute_units,
                "unique_accounts": cost.unique_accounts,
                "instruction_data_bytes": cost.instruction_data_bytes,
            }),
        );
    }
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut text = serde_json::to_string_pretty(&serde_json::Value::Object(object)).unwrap();
    text.push('\n');
    std::fs::write(path, text).unwrap();
}
