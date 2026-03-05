/// # DFG Pattern IDs
///
/// Deterministic structural fingerprints of a Data Flow Graph, used as
/// low-cardinality span attributes for latency segmentation in OpenTelemetry
/// metrics.
///
/// Two transactions performing the same FHE computation (e.g., an ERC20
/// `transferFrom`) produce the **same** `operation_pattern_id` regardless of
/// which ciphertext handles, addresses, or transaction IDs are involved.
/// Additionally, a `transaction_pattern_id` fingerprints the **entire**
/// transaction graph, so dashboards can segment by both "what kind of FHE
/// operation is this?" (operation-level) and "what kind of transaction is
/// this?" (tx-level, e.g., ERC20 transfer vs approval).
///
/// # Why not hash each component individually?
///
/// The scheduler partitions every DFG into single-operation [`ComponentNode`]s
/// for parallel execution. Hashing individual components would label every
/// `FheAdd` the same — useless for distinguishing contract-level operations
/// like "ERC20 transfer" vs "mint".
///
/// Instead, we group FHE ops into **logical operations** on the pre-partition
/// graph and fingerprint each group.
///
/// # Example
///
/// A diamond-shaped computation with one published output (`✓` = `is_allowed`):
///
/// ```text
///      ext_a  ext_b              ← external inputs (from DB)
///        │      │
///        ▼      ▼
///      ┌──────────┐
///      │  FheGe   │              node 0
///      └────┬─────┘
///           │
///      ┌────┴─────┐
///      ▼          ▼
///   ┌────────┐ ┌────────┐
///   │ FheAdd │ │ FheSub │        nodes 1, 2
///   └───┬────┘ └───┬────┘
///       └────┬─────┘
///            ▼
///     ┌──────────────┐
///     │FheIfThenElse │ ✓         node 3 → published result
///     └──────────────┘
/// ```
///
/// All four nodes form **one logical group** → one `pattern_id`.
///
/// A second transaction with the same shape but different ciphertext handles
/// produces the **same** `pattern_id`:
///
/// ```text
///   Tx A: handles 0x1A.. → diamond → pattern_id = a1b2c3...
///   Tx B: handles 0x2B.. → diamond → pattern_id = a1b2c3...  ← identical
///   Tx C: handles 0x1A.. → chain   → pattern_id = f4e5d6...  ← different shape
/// ```
///
/// # Logical-operation grouping
///
/// A transaction may contain multiple FHE operations. The grouping algorithm
/// figures out which ops belong to the same "logical operation" (e.g., one
/// ERC20 transfer) so they all get the same `pattern_id`.
///
/// ## Walkthrough: how grouping works
///
/// Suppose a computation has one intermediate node (`Le`) feeding two
/// published outputs (`✓`):
///
/// ```text
///   [Le]  ← takes ext_a
///    │
///    ├──► [Select] ✓   ← also takes ext_b   → out_a
///    │
///    └──► [Add] ✓      ← also takes ext_c   → out_b
/// ```
///
/// **Step 1 — trace backward from each `✓` node** (its "cone"):
///
/// ```text
///   From Select✓:  trace back → finds Le      cone = {Le, Select}
///   From Add✓:     trace back → finds Le      cone = {Le, Add}
///                                      ^^
///                                Le is in both cones
/// ```
///
/// **Step 2 — merge overlapping cones** (union-find):
///
/// Both cones contain `Le`, so they merge into one group:
/// `{Le, Select, Add}` → **one** `pattern_id`.
///
/// If `Select✓` and `Add✓` had *no* shared nodes, they would stay
/// separate groups with *different* `pattern_id`s.
///
/// ## Applied to ERC20 `transferFrom`
///
/// A real `transferFrom` compiles to 9 FHE operations with three published
/// outputs. The intermediate nodes `And` and `Select` are shared across
/// all three output cones:
///
/// ```text
///   [0]Le ──┐
///   [1]Le ──┼──► [2]And ──┬──► [4]Select ✓   newAllowance
///   [3]Sub ─┘             │
///                         └──► [6]Select ──┬──► [7]Add ✓  newBalanceTo
///                                          │
///   [5]TrivialEncrypt(0)                   └──► [8]Sub ✓  newBalanceFrom
///         ↑ source node (excluded)
/// ```
///
/// Tracing backward from the three `✓` nodes, all cones overlap at `And`,
/// `Le`, etc. — so union-find merges everything into **one group** of 8
/// computation nodes (the `TrivialEncrypt` is a source node and excluded).
///
/// Three `transferFrom` calls in one transaction — even with chained
/// dependencies — all get the **same** `pattern_id` because their
/// *structure* is identical. External inputs (DB handles, outputs from
/// other groups, source nodes) are all treated as opaque during encoding.
///
/// ## Algorithm summary
///
/// 1. **Classify nodes** — `TrivialEncrypt`, `FheRand`, `FheRandBounded`
///    that have no FHE dependencies are **source nodes** (input provisioning).
///    They are excluded from cone tracing but inherit their group's
///    `pattern_id` afterward if they feed exactly one group.
///
/// 2. **Backward cones** — From each `is_allowed=true` computation node,
///    walk backward collecting every non-source computation node. Stop at
///    other `is_allowed=true` nodes (they mark the boundary of another cone).
///
/// 3. **Union-find merge** — Cones sharing any node get merged.
///
/// 4. **Encode each group** — Compact binary encoding over structure
///    (see `encode_subgraph`).
///
/// # Encoding
///
/// Each group is encoded as a compact self-describing binary structure:
/// opcodes, `is_allowed` flags, input counts, and edges (internal refs vs
/// external) — all in topological order. The encoding is decodable without
/// any external lookup, unlike the previous Keccak-256 hash approach.
///
/// Only graph **shape** contributes to the fingerprint. Runtime identifiers
/// (output handles, transaction IDs, ciphertext data) are excluded.
mod encoding;
mod grouping;
mod types;

pub use encoding::{decode_pattern, is_hashed_pattern, pattern_to_base64url};
pub use grouping::{compute_logical_pattern_ids, compute_transaction_pattern_id};
pub use types::{PatternDescription, PatternInput, PatternNode};

#[cfg(test)]
use encoding::{ENCODING_VERSION, HASH_VERSION};

#[cfg(test)]
mod tests;
