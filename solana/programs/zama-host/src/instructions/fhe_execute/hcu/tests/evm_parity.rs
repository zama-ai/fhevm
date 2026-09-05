//! Pins every cost cell in `hcu/mod.rs` to `host-contracts/contracts/HCULimit.sol`, the EVM
//! table it transcribes. The Solidity file is parsed into cells; every cell of a type Solana
//! ships must be reached by the sweep below and agree, so a change on either side fails here
//! instead of drifting.

use super::super::*;
use super::ALL_BINARY_OPS;
use crate::state::FheBinaryOpCode;

const HCU_LIMIT_SOL: &str =
    include_str!("../../../../../../../../host-contracts/contracts/HCULimit.sol");

/// One `opHCU = N` cell of a `checkHCUFor*` function: which function, which `scalarByte`
/// branch, which FHE type, and for the reductions which `n <= K` bucket (`None` is the
/// trailing `else`).
struct SolCell {
    op: String,
    scalar: bool,
    ty: u8,
    bucket: Option<usize>,
    hcu: u64,
}

/// `Some(Some(ty))` when the line names a type Solana ships, `Some(None)` when it names one it
/// does not (euint4, euint160, euint256), `None` when the line names no type.
fn sol_type(line: &str) -> Option<Option<u8>> {
    let idx = line.find("FheType.")?;
    let name: String = line[idx + "FheType.".len()..]
        .chars()
        .take_while(|c| c.is_alphanumeric())
        .collect();
    Some(match name.as_str() {
        "Bool" => Some(0),
        "Uint8" => Some(2),
        "Uint16" => Some(3),
        "Uint32" => Some(4),
        "Uint64" => Some(5),
        "Uint128" => Some(6),
        _ => None,
    })
}

/// The decimal literal following `marker` on `line`, if the marker is present.
fn number_after<T: std::str::FromStr>(line: &str, marker: &str) -> Option<T> {
    let start = line.find(marker)? + marker.len();
    let digits: &str = &line[start..];
    let end = digits
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(digits.len());
    Some(digits[..end].parse().unwrap_or_else(|_| {
        panic!("HCULimit.sol: `{marker}` is not followed by a literal: {line}")
    }))
}

fn parse_hcu_limit_sol() -> Vec<SolCell> {
    let mut cells = Vec::new();
    for chunk in HCU_LIMIT_SOL.split("function checkHCUFor").skip(1) {
        let name_end = chunk
            .find('(')
            .expect("HCULimit.sol: `function checkHCUFor` without a parameter list");
        let op = chunk[..name_end].to_string();
        // Div and Rem accept only the scalar form (`if (scalarByte != 0x01) revert`).
        let mut scalar = chunk.contains("scalarByte !=");
        let mut scalar_if_depth: Option<i32> = None;
        let mut depth = 0i32;
        let mut ty: Option<u8> = None;
        for line in chunk.lines() {
            let line = line.trim();
            // `if (scalarByte == 0x01) {` opens the scalar branch; MulDiv spells both branches
            // out (`FHE_MUL_DIV_FACTOR2_SCALAR` / `_ENCRYPTED`); a bare `} else` back at the
            // depth of the first such `if` is the ciphertext branch or the unsupported revert.
            if line.contains("scalarByte ==") {
                scalar = !line.contains("_ENCRYPTED");
                scalar_if_depth.get_or_insert(depth);
            } else if line.starts_with("} else")
                && !line.contains("if")
                && scalar_if_depth == Some(depth - 1)
            {
                scalar = false;
            }
            if let Some(mapped) = sol_type(line) {
                ty = mapped;
            }
            if let Some(hcu) = number_after(line, "opHCU = ") {
                if let Some(ty) = ty {
                    cells.push(SolCell {
                        op: op.clone(),
                        scalar,
                        ty,
                        bucket: number_after(line, "n <= "),
                        hcu,
                    });
                }
            }
            depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
        }
    }
    cells
}

fn sol_binary_name(op: FheBinaryOpCode) -> String {
    match op {
        FheBinaryOpCode::And => "FheBitAnd".to_string(),
        FheBinaryOpCode::Or => "FheBitOr".to_string(),
        FheBinaryOpCode::Xor => "FheBitXor".to_string(),
        other => format!("Fhe{other:?}"),
    }
}

/// Looks cells up by (function, branch, type) and remembers which ones were consulted, so the
/// test can prove the sweep reached every cell the Solidity file defines.
struct SolTable {
    cells: Vec<SolCell>,
    consumed: Vec<bool>,
}

impl SolTable {
    fn matching(&self, op: &str, scalar: bool, ty: u8) -> impl Iterator<Item = usize> + '_ {
        let op = op.to_string();
        self.cells
            .iter()
            .enumerate()
            .filter(move |(_, c)| c.op == op && c.scalar == scalar && c.ty == ty)
            .map(|(i, _)| i)
    }

    fn take(&mut self, index: usize) -> u64 {
        self.consumed[index] = true;
        self.cells[index].hcu
    }

    /// The single cell of a non-reduction function.
    fn cell(&mut self, op: &str, scalar: bool, ty: u8) -> Option<u64> {
        let index = self.matching(op, scalar, ty).next()?;
        Some(self.take(index))
    }

    /// The `n <= K` bucket that prices `n` operands, or the trailing `else` cell.
    fn bucket(&mut self, op: &str, ty: u8, n: usize) -> Option<u64> {
        let mut buckets: Vec<(usize, usize)> = self
            .matching(op, false, ty)
            .filter_map(|i| self.cells[i].bucket.map(|k| (k, i)))
            .collect();
        buckets.sort_unstable();
        let index = buckets
            .into_iter()
            .find(|(k, _)| n <= *k)
            .map(|(_, i)| i)
            .or_else(|| {
                self.matching(op, false, ty)
                    .find(|&i| self.cells[i].bucket.is_none())
            })?;
        Some(self.take(index))
    }

    fn unconsumed(&self) -> Vec<String> {
        self.cells
            .iter()
            .zip(&self.consumed)
            .filter(|(_, &used)| !used)
            .map(|(c, _)| {
                format!(
                    "{} scalar={} ty{} bucket={:?}",
                    c.op, c.scalar, c.ty, c.bucket
                )
            })
            .collect()
    }
}

#[test]
fn cost_rows_match_evm_hculimit_sol() {
    let cells = parse_hcu_limit_sol();
    assert!(!cells.is_empty(), "parsed no cells from HCULimit.sol");
    let mut sol = SolTable {
        consumed: vec![false; cells.len()],
        cells,
    };
    let mut mismatches = Vec::new();
    let mut check = |what: String, ours: Result<u64>, theirs: Option<u64>| match (ours.ok(), theirs)
    {
        (Some(a), Some(b)) if a == b => {}
        (None, None) => {}
        (a, b) => mismatches.push(format!("{what}: Solana {a:?} vs HCULimit.sol {b:?}")),
    };
    for ty in 0..N as u8 {
        for op in ALL_BINARY_OPS {
            for scalar in [false, true] {
                check(
                    format!("{op:?} ty{ty} scalar={scalar}"),
                    binary_op_hcu(op, ty, scalar),
                    sol.cell(&sol_binary_name(op), scalar, ty),
                );
            }
        }
        for (op, name) in [
            (FheUnaryOpCode::Neg, "FheNeg"),
            (FheUnaryOpCode::Not, "FheNot"),
            (FheUnaryOpCode::Cast, "Cast"),
        ] {
            check(
                format!("{op:?} ty{ty}"),
                unary_op_hcu(op, ty),
                sol.cell(name, false, ty),
            );
        }
        check(
            format!("IfThenElse ty{ty}"),
            ternary_op_hcu(FheTernaryOpCode::IfThenElse, ty),
            sol.cell("IfThenElse", false, ty),
        );
        check(
            format!("TrivialEncrypt ty{ty}"),
            trivial_encrypt_hcu(ty),
            sol.cell("TrivialEncrypt", false, ty),
        );
        check(
            format!("Rand ty{ty}"),
            rand_hcu(ty),
            sol.cell("FheRand", false, ty),
        );
        check(
            format!("RandBounded ty{ty}"),
            rand_bounded_hcu(ty),
            sol.cell("FheRandBounded", false, ty),
        );
        for scalar in [false, true] {
            check(
                format!("MulDiv ty{ty} scalar={scalar}"),
                mul_div_hcu(ty, scalar),
                sol.cell("FheMulDiv", scalar, ty),
            );
        }
        for n in [1usize, 10, 11, 30, 31, 60, 61, 100] {
            check(
                format!("Sum ty{ty} n{n}"),
                sum_hcu(ty, n),
                sol.bucket("FheSum", ty, n),
            );
            check(
                format!("IsIn ty{ty} n{n}"),
                is_in_hcu(ty, n),
                sol.bucket("FheIsIn", ty, n),
            );
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    let unreached = sol.unconsumed();
    assert!(
        unreached.is_empty(),
        "HCULimit.sol cells the sweep never priced:\n{}",
        unreached.join("\n")
    );
}
