//! Cost report over Mollusk runs: one Markdown row per measured profile or probed boundary.
//!
//! When `ZAMA_COST_REPORT_DIR` names a directory, [`record_profile`] appends a row to its
//! `profiles.md` and [`record_boundary`] to its `boundaries.md`; when it is unset, both do
//! nothing. CI sets it and writes the rows, under the column headers in
//! `.github/workflows/solana-tests.yml`, to the job summary. Nothing compares the numbers, so a
//! cost change never fails a test: the tests that record a cost assert only that the instruction
//! executes.
//!
//! Profiles use fixed fixture keys because on-chain PDA bump searches are part of the measured
//! compute: absolute values include an arbitrary but stable bump-search cost, and Mollusk is not
//! a mainnet cost oracle. Compare the same profile across commits.

use mollusk_svm::result::InstructionResult;
use solana_sdk::instruction::Instruction;
use std::collections::HashSet;
use std::io::Write;

const REPORT_DIR_ENV: &str = "ZAMA_COST_REPORT_DIR";

/// Records the cost of one instruction profile. FHE fixtures run in a real transient store
/// transaction: compute units include open, body and close, while data bytes and CPIs describe
/// the body instruction.
///
/// Columns: profile, compute units, unique accounts (account-meta pubkeys plus the program id, a
/// static property of the instruction), instruction data bytes, CPIs at any stack depth, and the
/// total and largest CPI instruction data.
pub fn record_profile(
    suite: &str,
    profile: &str,
    instruction: &Instruction,
    result: &InstructionResult,
) {
    let cpi_data: Vec<usize> = result
        .inner_instructions
        .iter()
        .map(|inner| inner.instruction.data.len())
        .collect();
    let mut accounts: HashSet<_> = instruction
        .accounts
        .iter()
        .map(|meta| meta.pubkey)
        .collect();
    accounts.insert(instruction.program_id);
    append_row(
        "profiles.md",
        &[
            format!("{suite}/{profile}"),
            result.compute_units_consumed.to_string(),
            accounts.len().to_string(),
            instruction.data.len().to_string(),
            cpi_data.len().to_string(),
            format!(
                "{} / {}",
                cpi_data.iter().sum::<usize>(),
                cpi_data.iter().max().unwrap_or(&0)
            ),
        ],
    );
}

/// Records a probed capacity wall. Columns: shape, the largest step count that executes, the
/// limit the next count hit, and the compute units and instruction data bytes of the largest
/// passing run.
pub fn record_boundary(
    shape: &str,
    max_ok: usize,
    stopped_by: &str,
    instruction_at_max_ok: &Instruction,
    result_at_max_ok: &InstructionResult,
) {
    append_row(
        "boundaries.md",
        &[
            shape.to_string(),
            max_ok.to_string(),
            stopped_by.to_string(),
            result_at_max_ok.compute_units_consumed.to_string(),
            instruction_at_max_ok.data.len().to_string(),
        ],
    );
}

/// Appends one row in a single write, so rows from tests running in parallel do not interleave.
fn append_row(file: &str, cells: &[String]) {
    let Some(dir) = std::env::var_os(REPORT_DIR_ENV) else {
        return;
    };
    let path = std::path::Path::new(&dir).join(file);
    std::fs::create_dir_all(&dir).unwrap();
    let row = format!("| {} |\n", cells.join(" | "));
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .and_then(|mut report| report.write_all(row.as_bytes()))
        .unwrap_or_else(|err| panic!("append to the cost report {}: {err}", path.display()));
}
