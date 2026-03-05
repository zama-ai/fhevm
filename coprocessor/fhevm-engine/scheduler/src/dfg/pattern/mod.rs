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
mod tests {
    use crate::dfg::{build_component_nodes, types::DFGTaskInput, DFGOp, OpEdge};
    use daggy::{petgraph::graph::node_index, Dag};
    use fhevm_engine_common::types::SupportedFheOperations;
    use std::collections::HashMap;

    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    use super::{
        decode_pattern, is_hashed_pattern, pattern_to_base64url, ENCODING_VERSION, HASH_VERSION,
    };

    /// Build a pre-partition graph + produced_handles from `Vec<DFGOp>` and call
    /// `compute_logical_pattern_ids` directly. Avoids going through `build_component_nodes`
    /// for finer-grained assertions on the logical grouping.
    fn compute_logical_ids(mut ops: Vec<DFGOp>) -> HashMap<usize, Vec<u8>> {
        ops.sort_by_key(|o| o.output_handle.clone());
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        let mut produced_handles: HashMap<Vec<u8>, usize> = HashMap::new();
        for (index, op) in ops.iter().enumerate() {
            produced_handles.insert(op.output_handle.clone(), index);
        }
        let mut dependence_pairs = vec![];
        for (index, op) in ops.iter().enumerate() {
            for (pos, i) in op.inputs.iter().enumerate() {
                if let DFGTaskInput::Dependence(dh) = i {
                    if let Some(&producer) = produced_handles.get(dh) {
                        dependence_pairs.push((producer, index, pos));
                    }
                }
            }
            graph.add_node((op.is_allowed, index));
        }
        for (source, destination, pos) in dependence_pairs {
            graph
                .add_edge(node_index(source), node_index(destination), pos as u8)
                .expect("cycle in test graph");
        }
        super::compute_logical_pattern_ids(&graph, &ops, &produced_handles)
    }

    /// Helper: build a unique 32-byte handle from an id byte and an index.
    fn handle(prefix: u8, idx: u8) -> Vec<u8> {
        let mut h = vec![prefix; 32];
        h[0] = idx;
        h
    }

    /// Collect all per-op pattern_ids from a component's inner DFGraph.
    fn collect_op_pattern_ids(component: &super::super::ComponentNode) -> Vec<Vec<u8>> {
        use daggy::petgraph::visit::IntoNodeReferences;
        component
            .graph
            .graph
            .node_references()
            .map(|(_, node)| node.operation_pattern_id().to_vec())
            .collect()
    }

    /// Assert that a pattern_id is a valid pattern — either a v1 encoding
    /// (decodable) or a v2 hash (23 bytes, starts with HASH_VERSION).
    fn assert_valid_pattern(bytes: &[u8]) {
        assert!(!bytes.is_empty(), "pattern_id should not be empty");
        match bytes[0] {
            ENCODING_VERSION => {
                let desc = decode_pattern(bytes).expect("v1 pattern_id should be decodable");
                assert!(
                    !desc.nodes.is_empty(),
                    "decoded pattern should have at least one node"
                );
            }
            HASH_VERSION => {
                assert_eq!(bytes.len(), 23, "hashed pattern should be exactly 23 bytes");
            }
            other => panic!("unexpected pattern version byte: 0x{other:02x}"),
        }
    }

    /// Build DFGOps matching one ERC20 transferFrom call.
    ///
    /// The 9 compute ops (excluding verify/fromExternal which doesn't enter computations):
    ///   [0] Le(amount, currentAllowance)               → allowedTransfer
    ///   [1] Le(amount, balances[from])                  → canTransfer
    ///   [2] And(canTransfer, allowedTransfer)           → isTransferable
    ///   [3] Sub(currentAllowance, amount)               → allowanceDiff
    ///   [4] Select(isTransferable, allowanceDiff, currentAllowance) → newAllowance
    ///   [5] TrivialEncrypt(0)                           → zero
    ///   [6] Select(isTransferable, amount, zero)        → transferValue
    ///   [7] Add(balances[to], transferValue)            → newBalanceTo
    ///   [8] Sub(balances[from], transferValue)          → newBalanceFrom
    ///
    /// `prefix` differentiates handles across multiple calls in the same tx.
    fn build_transfer_from_ops(prefix: u8) -> Vec<DFGOp> {
        build_transfer_from_ops_with_amount(prefix, handle(0xE0, prefix))
    }

    /// Build a transfer where the amount input is supplied externally (e.g.,
    /// from a prior allowed output to simulate dependent transfers).
    fn build_transfer_from_ops_with_amount(prefix: u8, ext_amount: Vec<u8>) -> Vec<DFGOp> {
        // External handles (not produced by this set of ops)
        let ext_allowance = handle(0xE1, prefix);
        let ext_bal_from = handle(0xE2, prefix);
        let ext_bal_to = handle(0xE3, prefix);

        // Output handles for each op
        let h0 = handle(prefix, 0); // allowedTransfer
        let h1 = handle(prefix, 1); // canTransfer
        let h2 = handle(prefix, 2); // isTransferable
        let h3 = handle(prefix, 3); // allowanceDiff
        let h4 = handle(prefix, 4); // newAllowance
        let h5 = handle(prefix, 5); // zero
        let h6 = handle(prefix, 6); // transferValue
        let h7 = handle(prefix, 7); // newBalanceTo
        let h8 = handle(prefix, 8); // newBalanceFrom

        vec![
            // [0] Le(amount, currentAllowance) → allowedTransfer
            DFGOp {
                output_handle: h0.clone(),
                fhe_op: SupportedFheOperations::FheLe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [1] Le(amount, balances[from]) → canTransfer
            DFGOp {
                output_handle: h1.clone(),
                fhe_op: SupportedFheOperations::FheLe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(ext_bal_from.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [2] And(canTransfer, allowedTransfer) → isTransferable
            DFGOp {
                output_handle: h2.clone(),
                fhe_op: SupportedFheOperations::FheBitAnd,
                inputs: vec![
                    DFGTaskInput::Dependence(h1.clone()),
                    DFGTaskInput::Dependence(h0.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [3] Sub(currentAllowance, amount) → allowanceDiff
            DFGOp {
                output_handle: h3.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                    DFGTaskInput::Dependence(ext_amount.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [4] Select(isTransferable, allowanceDiff, currentAllowance) → newAllowance
            DFGOp {
                output_handle: h4.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h2.clone()),
                    DFGTaskInput::Dependence(h3.clone()),
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [5] TrivialEncrypt(0) → zero
            DFGOp {
                output_handle: h5.clone(),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            // [6] Select(isTransferable, amount, zero) → transferValue
            DFGOp {
                output_handle: h6.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h2.clone()),
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(h5.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [7] Add(balances[to], transferValue) → newBalanceTo
            DFGOp {
                output_handle: h7.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bal_to.clone()),
                    DFGTaskInput::Dependence(h6.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [8] Sub(balances[from], transferValue) → newBalanceFrom
            DFGOp {
                output_handle: h8.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bal_from.clone()),
                    DFGTaskInput::Dependence(h6.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ]
    }

    #[test]
    fn transfer_from_partition_analysis() {
        let tx_id = vec![0xFFu8; 32];

        // === Single transferFrom ===
        let ops = build_transfer_from_ops(0x10);
        let (components, _unneeded) = build_component_nodes(ops, &tx_id).unwrap();
        // All ops of a single transfer should share the same logical pattern_id
        let mut all_op_patterns: Vec<Vec<u8>> =
            components.iter().flat_map(collect_op_pattern_ids).collect();
        all_op_patterns.sort();
        all_op_patterns.dedup();
        assert_eq!(
            all_op_patterns.len(),
            1,
            "all ops within one transfer should share the same pattern_id"
        );
        let single_pattern = &all_op_patterns[0];
        assert_valid_pattern(single_pattern);

        // === Three independent transferFrom calls in one tx ===
        let mut all_ops = build_transfer_from_ops(0x10);
        all_ops.extend(build_transfer_from_ops(0x20));
        all_ops.extend(build_transfer_from_ops(0x30));
        let (components_3, _) = build_component_nodes(all_ops, &tx_id).unwrap();

        // All ops across all three transfers should share ONE pattern_id
        let mut pattern_ids: Vec<Vec<u8>> = components_3
            .iter()
            .flat_map(collect_op_pattern_ids)
            .collect();
        pattern_ids.sort();
        pattern_ids.dedup();
        assert_eq!(
            pattern_ids.len(),
            1,
            "three independent identical transfers should produce 1 distinct pattern_id, got {}",
            pattern_ids.len()
        );
        assert_eq!(
            &pattern_ids[0], single_pattern,
            "the pattern_id should match the single-transfer pattern_id"
        );
    }

    /// Build DFGOps matching the "whitepaper" ERC20 transfer used by integration
    /// tests in scheduling_bench.rs (5 ops):
    ///   [0] FheGe(bals, trxa)                             → has_enough_funds
    ///   [1] FheAdd(bald, trxa)                            → new_to_amount_target
    ///   [2] FheIfThenElse(has_enough_funds, new_to_target, bald) → new_to_amount
    ///   [3] FheSub(bals, trxa)                            → new_from_amount_target
    ///   [4] FheIfThenElse(has_enough_funds, new_from_target, bals) → new_from_amount
    fn build_whitepaper_erc20_ops(prefix: u8) -> Vec<DFGOp> {
        let ext_bals = handle(0xA0, prefix); // balance source
        let ext_trxa = handle(0xA1, prefix); // transfer amount
        let ext_bald = handle(0xA2, prefix); // balance destination

        let h0 = handle(prefix, 0); // has_enough_funds
        let h1 = handle(prefix, 1); // new_to_amount_target
        let h2 = handle(prefix, 2); // new_to_amount
        let h3 = handle(prefix, 3); // new_from_amount_target
        let h4 = handle(prefix, 4); // new_from_amount

        vec![
            // [0] FheGe(bals, trxa) → has_enough_funds
            DFGOp {
                output_handle: h0.clone(),
                fhe_op: SupportedFheOperations::FheGe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bals.clone()),
                    DFGTaskInput::Dependence(ext_trxa.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [1] FheAdd(bald, trxa) → new_to_amount_target
            DFGOp {
                output_handle: h1.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bald.clone()),
                    DFGTaskInput::Dependence(ext_trxa.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [2] FheIfThenElse(has_enough_funds, new_to_target, bald) → new_to_amount
            DFGOp {
                output_handle: h2.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h0.clone()),
                    DFGTaskInput::Dependence(h1.clone()),
                    DFGTaskInput::Dependence(ext_bald.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [3] FheSub(bals, trxa) → new_from_amount_target
            DFGOp {
                output_handle: h3.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bals.clone()),
                    DFGTaskInput::Dependence(ext_trxa.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [4] FheIfThenElse(has_enough_funds, new_from_target, bals) → new_from_amount
            DFGOp {
                output_handle: h4.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h0.clone()),
                    DFGTaskInput::Dependence(h3.clone()),
                    DFGTaskInput::Dependence(ext_bals.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ]
    }

    #[test]
    fn whitepaper_erc20_partition_analysis() {
        let tx_id = vec![0xFFu8; 32];

        // The whitepaper ERC20 test marks ALL ops is_allowed=true, which means
        // every op is its own cone boundary. This is a test artifact — in
        // production only final outputs are allowed. Each op becomes its own
        // single-node group, so we get per-op pattern_ids (degraded but correct).
        let ops = build_whitepaper_erc20_ops(0x10);
        let (components, _) = build_component_nodes(ops, &tx_id).unwrap();
        // Verify all ops have valid pattern_ids
        for c in &components {
            for pid in collect_op_pattern_ids(c) {
                assert_valid_pattern(&pid);
            }
        }

        // Seven samples: each transfer produces the same set of per-op patterns
        let mut all_ops = Vec::new();
        for i in 0..7u8 {
            all_ops.extend(build_whitepaper_erc20_ops(i + 1));
        }
        let (components_7, _) = build_component_nodes(all_ops, &tx_id).unwrap();
        let mut pattern_ids: Vec<Vec<u8>> = components_7
            .iter()
            .flat_map(collect_op_pattern_ids)
            .collect();
        pattern_ids.sort();
        pattern_ids.dedup();
        // With all-allowed, each op becomes its own single-node group. Ops with
        // the same opcode+structure share a pattern_id. The whitepaper ERC20 has
        // 4 distinct opcodes: FheGe, FheAdd, FheSub, FheIfThenElse.
        assert_eq!(
            pattern_ids.len(),
            4,
            "4 distinct opcodes (Ge, Add, Sub, IfThenElse) should produce 4 distinct pattern_ids"
        );
    }

    /// Build a transfer where one input comes from a prior allowed output
    /// (simulating a dependent transfer in a chain).
    fn build_transfer_from_ops_dependent(prefix: u8, prior_allowed_handle: Vec<u8>) -> Vec<DFGOp> {
        build_transfer_from_ops_with_amount(prefix, prior_allowed_handle)
    }

    #[test]
    fn three_mixed_transfers_same_pattern() {
        // Three transfers in one tx:
        //   1) From DB (standard)
        //   2) Dependent on transfer 1's newBalanceTo output
        //   3) From DB (standard, different prefix)
        // All three should produce the SAME logical pattern_id.
        let tx_id = vec![0xFFu8; 32];

        // Transfer 1: standard from DB
        let ops_1 = build_transfer_from_ops(0x10);
        // The allowed output handle from transfer 1 (newBalanceTo = h7)
        let prior_output = handle(0x10, 7);
        // Transfer 2: dependent — uses transfer 1's output as its amount input
        let ops_2 = build_transfer_from_ops_dependent(0x20, prior_output);
        // Transfer 3: standard from DB
        let ops_3 = build_transfer_from_ops(0x30);

        let mut all_ops = ops_1;
        all_ops.extend(ops_2);
        all_ops.extend(ops_3);
        let (components, _) = build_component_nodes(all_ops, &tx_id).unwrap();

        let mut pattern_ids: Vec<Vec<u8>> =
            components.iter().flat_map(collect_op_pattern_ids).collect();
        pattern_ids.sort();
        pattern_ids.dedup();
        assert_eq!(pattern_ids.len(), 1,
            "three mixed transfers (DB, dependent, DB) should all produce the same pattern_id, got {} distinct",
            pattern_ids.len());
    }

    #[test]
    fn transfer_vs_different_op_different_pattern() {
        // A transferFrom and a simple Add operation should have different pattern_ids.
        let tx_id = vec![0xFFu8; 32];

        let transfer_ops = build_transfer_from_ops(0x10);
        let (transfer_components, _) = build_component_nodes(transfer_ops, &tx_id).unwrap();
        let transfer_pattern = collect_op_pattern_ids(&transfer_components[0])[0].clone();

        // A simple FheAdd with two external inputs
        let simple_ops = vec![DFGOp {
            output_handle: handle(0x50, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xD0, 0)),
                DFGTaskInput::Dependence(handle(0xD1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let (simple_components, _) = build_component_nodes(simple_ops, &tx_id).unwrap();
        let simple_pattern = collect_op_pattern_ids(&simple_components[0])[0].clone();

        assert_ne!(
            transfer_pattern, simple_pattern,
            "a transferFrom and a simple FheAdd should produce different pattern_ids"
        );
    }

    // -----------------------------------------------------------------------
    // Tests using compute_logical_ids helper
    // -----------------------------------------------------------------------

    #[test]
    fn two_independent_groups_different_patterns() {
        // FheAdd✓ and FheSub✓ with disjoint external inputs → separate groups,
        // different pattern_ids.
        let ops = vec![
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert_eq!(ids.len(), 2, "both allowed nodes should have pattern_ids");
        let id_vals: Vec<&Vec<u8>> = ids.values().collect();
        assert_ne!(
            id_vals[0], id_vals[1],
            "different opcodes → different pattern_ids"
        );
    }

    #[test]
    fn chain_of_allowed_nodes_separate_groups() {
        // A✓(Add) → B✓(Sub) → C✓(Mul). Each allowed node is its own cone
        // boundary, so 3 groups with 3 distinct pattern_ids.
        let h_a = handle(0x10, 0);
        let h_b = handle(0x20, 0);
        let h_c = handle(0x30, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_a.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_b.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_a.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_c.clone(),
                fhe_op: SupportedFheOperations::FheMul,
                inputs: vec![
                    DFGTaskInput::Dependence(h_b.clone()),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert_eq!(ids.len(), 3, "each allowed node should have a pattern_id");
        let mut unique: Vec<Vec<u8>> = ids.values().cloned().collect();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            3,
            "3 distinct opcodes → 3 distinct pattern_ids"
        );
    }

    #[test]
    fn source_node_feeding_two_groups_not_inherited() {
        // TrivialEncrypt feeds both FheAdd✓ and FheSub✓ (different groups).
        // Source node should NOT appear in result map (feeds >1 pattern).
        let h_src = handle(0x01, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_src.clone(),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(h_src.clone()),
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_src.clone()),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        // The source node feeds two groups with different pattern_ids,
        // so it should not inherit either. Only the two allowed nodes appear.
        assert_eq!(
            ids.len(),
            2,
            "source node feeding 2 groups should not be in result"
        );
    }

    #[test]
    fn fhe_rand_is_source_node() {
        // FheRand (no deps) feeds FheAdd✓. FheRand should inherit the group's
        // pattern_id (same behavior as TrivialEncrypt feeding a single group).
        let h_rand = handle(0x01, 0);
        let h_add = handle(0x10, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_rand.clone(),
                fhe_op: SupportedFheOperations::FheRand,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_add.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(h_rand.clone()),
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        // Both FheRand and FheAdd should have the same pattern_id (source
        // feeds single group → inherits).
        assert_eq!(
            ids.len(),
            2,
            "both rand source and add should have pattern_ids"
        );
        let vals: Vec<Vec<u8>> = ids.values().cloned().collect();
        assert_eq!(
            vals[0], vals[1],
            "source feeding single group inherits pattern_id"
        );
    }

    #[test]
    fn three_way_fan_out_single_group() {
        // One intermediate (non-allowed) feeds 3 allowed outputs.
        // The intermediate and all 3 outputs should merge into one group.
        let h_mid = handle(0x01, 0);
        let h_a = handle(0x10, 0);
        let h_b = handle(0x20, 0);
        let h_c = handle(0x30, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_mid.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_a.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_mid.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_b.clone(),
                fhe_op: SupportedFheOperations::FheMul,
                inputs: vec![
                    DFGTaskInput::Dependence(h_mid.clone()),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_c.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(h_mid.clone()),
                    DFGTaskInput::Dependence(handle(0xE4, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert_eq!(
            ids.len(),
            4,
            "all 4 nodes (1 intermediate + 3 allowed) should be in result"
        );
        let mut unique: Vec<Vec<u8>> = ids.values().cloned().collect();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            1,
            "shared intermediate merges all into one group"
        );
    }

    #[test]
    fn all_non_allowed_empty_result() {
        // Two non-allowed computation nodes, no allowed nodes → empty result.
        let ops = vec![
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert!(ids.is_empty(), "no allowed nodes → no pattern_ids");
    }

    #[test]
    fn only_source_nodes_empty_result() {
        // Two TrivialEncrypt nodes, no allowed nodes → empty result.
        let ops = vec![
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![1u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert!(ids.is_empty(), "only source nodes → no pattern_ids");
    }

    // -----------------------------------------------------------------------
    // Transaction-level pattern ID tests
    // -----------------------------------------------------------------------

    #[test]
    fn transaction_pattern_id_single_vs_triple_transfer() {
        let tx_id = vec![0xFFu8; 32];

        // Single transferFrom: one tx-level pattern
        let ops_single = build_transfer_from_ops(0x10);
        let (components_single, _) = build_component_nodes(ops_single, &tx_id).unwrap();
        let tx_pat_single = &components_single[0].transaction_pattern_id;
        // All components in the same tx share the same transaction_pattern_id
        for c in &components_single {
            assert_eq!(
                &c.transaction_pattern_id, tx_pat_single,
                "all components within one tx share the same transaction_pattern_id"
            );
        }

        // Three independent transferFrom calls in one tx: different tx-level pattern
        let mut ops_triple = build_transfer_from_ops(0x10);
        ops_triple.extend(build_transfer_from_ops(0x20));
        ops_triple.extend(build_transfer_from_ops(0x30));
        let (components_triple, _) = build_component_nodes(ops_triple, &tx_id).unwrap();
        let tx_pat_triple = &components_triple[0].transaction_pattern_id;
        for c in &components_triple {
            assert_eq!(
                &c.transaction_pattern_id, tx_pat_triple,
                "all components within one tx share the same transaction_pattern_id"
            );
        }

        // Single-transfer tx vs triple-transfer tx should differ
        assert_ne!(
            tx_pat_single, tx_pat_triple,
            "a 1-transfer tx and a 3-transfer tx should have different transaction_pattern_ids"
        );
    }

    #[test]
    fn transaction_pattern_id_same_structure_same_id() {
        let tx_id = vec![0xFFu8; 32];

        // Two txs with identical structure (same number of ops, same shape)
        // but different handle prefixes should produce the same tx pattern.
        let ops_a = build_transfer_from_ops(0x10);
        let ops_b = build_transfer_from_ops(0x20);
        let (components_a, _) = build_component_nodes(ops_a, &tx_id).unwrap();
        let (components_b, _) = build_component_nodes(ops_b, &tx_id).unwrap();

        assert_eq!(
            components_a[0].transaction_pattern_id, components_b[0].transaction_pattern_id,
            "same tx structure with different handles should produce same transaction_pattern_id"
        );
    }

    #[test]
    fn transaction_pattern_id_different_structure_different_id() {
        let tx_id = vec![0xFFu8; 32];

        // transferFrom (9 ops)
        let ops_transfer = build_transfer_from_ops(0x10);
        let (components_transfer, _) = build_component_nodes(ops_transfer, &tx_id).unwrap();

        // simple single FheAdd
        let ops_simple = vec![DFGOp {
            output_handle: handle(0x50, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xD0, 0)),
                DFGTaskInput::Dependence(handle(0xD1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let (components_simple, _) = build_component_nodes(ops_simple, &tx_id).unwrap();

        assert_ne!(
            components_transfer[0].transaction_pattern_id,
            components_simple[0].transaction_pattern_id,
            "transferFrom tx and simple-add tx should have different transaction_pattern_ids"
        );
    }

    // -----------------------------------------------------------------------
    // Regression: per-op pattern_id survives partition merging
    // -----------------------------------------------------------------------

    #[test]
    fn chain_of_allowed_nodes_per_op_pattern_ids() {
        // A✓(Add) → B✓(Sub) → C✓(Mul): each allowed node is its own cone
        // boundary, so each gets a DISTINCT pattern_id. But
        // partition_preserving_parallelism merges the chain (in-degree=1,
        // out-degree=1) into ONE component. With the old per-component
        // pattern_id this was a debug_assert! violation; with per-op
        // pattern_ids each OpNode keeps its own.
        let tx_id = vec![0xFFu8; 32];

        let h_a = handle(0x10, 0);
        let h_b = handle(0x20, 0);
        let h_c = handle(0x30, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_a.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_b.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_a.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_c.clone(),
                fhe_op: SupportedFheOperations::FheMul,
                inputs: vec![
                    DFGTaskInput::Dependence(h_b.clone()),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];

        let (components, _) = build_component_nodes(ops, &tx_id).unwrap();

        // The chain should be merged into ONE component by the partitioner
        // (each link has in-degree=1 and out-degree=1).
        assert_eq!(
            components.len(),
            1,
            "A→B→C chain should be merged into one component"
        );

        // Each OpNode in that component's inner DFG must have a DISTINCT
        // non-empty operation_pattern_id (3 different opcodes → 3 distinct encodings).
        let op_pids = collect_op_pattern_ids(&components[0]);
        assert_eq!(op_pids.len(), 3, "component should contain 3 ops");

        for (i, pid) in op_pids.iter().enumerate() {
            assert!(
                !pid.is_empty(),
                "operation_pattern_id for op {i} should not be empty"
            );
            assert_valid_pattern(pid);
        }

        let mut unique = op_pids.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            3,
            "3 distinct opcodes (Add, Sub, Mul) in a chain of allowed nodes \
             must produce 3 distinct per-op pattern_ids, got {} distinct",
            unique.len()
        );

        // transaction_pattern_id is uniform across the component (whole-tx encoding)
        let tx_pid = &components[0].transaction_pattern_id;
        assert!(
            !tx_pid.is_empty(),
            "transaction_pattern_id should not be empty"
        );
        assert_valid_pattern(tx_pid);
    }

    // -----------------------------------------------------------------------
    // Encoding / decoding round-trip tests
    // -----------------------------------------------------------------------

    #[test]
    fn decode_single_node_pattern() {
        // Single FheAdd with 2 external inputs, is_allowed=true
        let ops = vec![DFGOp {
            output_handle: handle(0x10, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xE0, 0)),
                DFGTaskInput::Dependence(handle(0xE1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let ids = compute_logical_ids(ops);
        let pattern = ids.values().next().unwrap();

        let desc = decode_pattern(pattern).expect("should decode");
        assert_eq!(desc.nodes.len(), 1);
        assert_eq!(desc.nodes[0].opcode, SupportedFheOperations::FheAdd as i32);
        assert_eq!(desc.nodes[0].opcode_name, "FHE_ADD");
        assert!(desc.nodes[0].is_allowed);
        assert_eq!(desc.nodes[0].inputs.len(), 2);
        // Both inputs are external
        for inp in &desc.nodes[0].inputs {
            assert!(matches!(inp, super::PatternInput::External));
        }
        assert_eq!(format!("{desc}"), "FHE_ADD[a]");
    }

    #[test]
    fn decode_multi_node_pattern() {
        // Two-node group: FheAdd(ext, ext) → FheSub(Add, ext), Sub is allowed
        let h_add = handle(0x10, 0);
        let h_sub = handle(0x20, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_add.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_sub.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_add.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        // Both should be in the same group
        assert_eq!(ids.len(), 2);
        let pattern = ids.values().next().unwrap();
        let desc = decode_pattern(pattern).expect("should decode");
        assert_eq!(desc.nodes.len(), 2);

        // Node 0: FheAdd, not allowed, 2 external inputs
        assert_eq!(desc.nodes[0].opcode_name, "FHE_ADD");
        assert!(!desc.nodes[0].is_allowed);
        assert_eq!(desc.nodes[0].inputs.len(), 2);

        // Node 1: FheSub, allowed, 1 internal + 1 external
        assert_eq!(desc.nodes[1].opcode_name, "FHE_SUB");
        assert!(desc.nodes[1].is_allowed);
        assert_eq!(desc.nodes[1].inputs.len(), 2);
        assert!(matches!(
            desc.nodes[1].inputs[0],
            super::PatternInput::Internal(0)
        ));
        assert!(matches!(
            desc.nodes[1].inputs[1],
            super::PatternInput::External
        ));

        assert_eq!(format!("{desc}"), "FHE_ADD,FHE_SUB[a]");
    }

    #[test]
    fn base64url_roundtrip() {
        let ops = vec![DFGOp {
            output_handle: handle(0x10, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xE0, 0)),
                DFGTaskInput::Dependence(handle(0xE1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let ids = compute_logical_ids(ops);
        let pattern = ids.values().next().unwrap();
        let b64 = pattern_to_base64url(pattern);
        // Verify the base64url decodes back to the same bytes
        let decoded = URL_SAFE_NO_PAD
            .decode(b64.as_bytes())
            .expect("should decode base64url");
        assert_eq!(&decoded, pattern);
        // Verify the base64url string is much shorter than the old 64-char hex
        assert!(
            b64.len() < 64,
            "base64url for single-node pattern should be shorter than 64 chars, got {}",
            b64.len()
        );
    }

    // -----------------------------------------------------------------------
    // V1 encoding limit tests
    // -----------------------------------------------------------------------

    #[test]
    fn oversize_group_hashes_for_transaction_pattern() {
        // Build a group with > 255 nodes (V1_MAX_NODES).
        // All are independent allowed FheAdd ops with external inputs.
        let n = 256;
        let ops: Vec<DFGOp> = (0..n)
            .map(|i| {
                let mut h = vec![0u8; 32];
                h[0] = (i >> 8) as u8;
                h[1] = (i & 0xFF) as u8;
                DFGOp {
                    output_handle: h,
                    fhe_op: SupportedFheOperations::FheAdd,
                    inputs: vec![
                        DFGTaskInput::Dependence(handle(0xE0, i as u8)),
                        DFGTaskInput::Dependence(handle(0xE1, i as u8)),
                    ],
                    is_allowed: true,
                    ..Default::default()
                }
            })
            .collect();
        let ids = compute_logical_ids(ops);
        // With 256 independent allowed nodes, each is its own group (1 node),
        // so individual groups are within limits. The overflow applies to
        // transaction_pattern_id which encodes all 256 as one group.
        // Verify individual groups still work:
        assert!(
            !ids.is_empty(),
            "individual single-node groups should still encode"
        );

        // Now test that compute_transaction_pattern_id still attributes
        // oversized groups via hash-only fallback (no full encoding in span attrs).
        let n = 256;
        let mut ops2: Vec<DFGOp> = Vec::with_capacity(n);
        // Build a chain: each op depends on the previous, so the whole thing
        // is one connected component for the tx-level encoding.
        for i in 0..n {
            let mut h = vec![0u8; 32];
            h[0] = (i >> 8) as u8;
            h[1] = (i & 0xFF) as u8;
            let inputs = if i == 0 {
                vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ]
            } else {
                let mut prev_h = vec![0u8; 32];
                prev_h[0] = ((i - 1) >> 8) as u8;
                prev_h[1] = ((i - 1) & 0xFF) as u8;
                vec![
                    DFGTaskInput::Dependence(prev_h),
                    DFGTaskInput::Dependence(handle(0xE1, i as u8)),
                ]
            };
            ops2.push(DFGOp {
                output_handle: h,
                fhe_op: SupportedFheOperations::FheAdd,
                inputs,
                is_allowed: i == n - 1, // only last is allowed
                ..Default::default()
            });
        }

        let tx_id = vec![0xFFu8; 32];
        let (components, _) = build_component_nodes(ops2, &tx_id).unwrap();
        // >255 nodes: v1 encoding fails, wide encoding is hashed.
        let tx_pat = &components[0].transaction_pattern_id;
        assert!(
            is_hashed_pattern(tx_pat),
            "256-node tx should produce hashed transaction_pattern_id"
        );
        assert_eq!(tx_pat.len(), 23, "hashed pattern should be 23 bytes");
        let node_count = u16::from_be_bytes([tx_pat[1], tx_pat[2]]);
        assert_eq!(
            node_count, 256,
            "hashed pattern should encode node_count=256"
        );
    }

    // -----------------------------------------------------------------------
    // Two-tier encoding tests
    // -----------------------------------------------------------------------

    /// Helper: build a chain of `n` FheAdd ops where each depends on the
    /// previous (except the first which has external inputs). Only the last
    /// is `is_allowed`. This creates a single logical group of `n` computation
    /// nodes. Uses distinct handle prefixes to avoid collisions.
    fn build_chain(n: usize, prefix: u8) -> Vec<DFGOp> {
        let mut ops = Vec::with_capacity(n);
        for i in 0..n {
            let mut h = vec![0u8; 32];
            h[0] = prefix;
            h[1] = (i >> 8) as u8;
            h[2] = (i & 0xFF) as u8;
            let inputs = if i == 0 {
                vec![
                    DFGTaskInput::Dependence(handle(0xE0, prefix)),
                    DFGTaskInput::Dependence(handle(0xE1, prefix)),
                ]
            } else {
                let mut prev_h = vec![0u8; 32];
                prev_h[0] = prefix;
                prev_h[1] = ((i - 1) >> 8) as u8;
                prev_h[2] = ((i - 1) & 0xFF) as u8;
                vec![
                    DFGTaskInput::Dependence(prev_h),
                    DFGTaskInput::Dependence(handle(0xE1, (i & 0xFF) as u8)),
                ]
            };
            ops.push(DFGOp {
                output_handle: h,
                fhe_op: SupportedFheOperations::FheAdd,
                inputs,
                is_allowed: i == n - 1,
                ..Default::default()
            });
        }
        ops
    }

    #[test]
    fn pattern_below_threshold_is_encoding() {
        // A group with ≤ DEFAULT_PATTERN_HASH_THRESHOLD nodes stays v1.
        let ops = build_chain(10, 0xA0);
        let ids = compute_logical_ids(ops);
        assert!(!ids.is_empty());
        let pattern = ids.values().next().unwrap();
        assert_eq!(
            pattern[0], ENCODING_VERSION,
            "small group should produce v1 encoding"
        );
        assert!(
            decode_pattern(pattern).is_some(),
            "v1 encoding should be decodable"
        );
        assert!(
            !is_hashed_pattern(pattern),
            "v1 encoding should not be identified as hashed"
        );
    }

    #[test]
    fn pattern_above_threshold_is_hashed() {
        // A group with > DEFAULT_PATTERN_HASH_THRESHOLD nodes gets hashed.
        // Default threshold is 25, so 30 nodes should trigger hashing.
        let ops = build_chain(30, 0xB0);
        let ids = compute_logical_ids(ops);
        assert!(!ids.is_empty());
        let pattern = ids.values().next().unwrap();
        assert_eq!(
            pattern[0], HASH_VERSION,
            "large group should produce v2 hashed pattern"
        );
        assert_eq!(
            pattern.len(),
            23,
            "hashed pattern should be exactly 23 bytes"
        );
        // node_count is encoded as u16 big-endian at bytes 1-2
        let node_count = u16::from_be_bytes([pattern[1], pattern[2]]);
        assert_eq!(node_count, 30, "hashed pattern should encode node_count=30");
        assert!(
            is_hashed_pattern(pattern),
            "v2 pattern should be identified as hashed"
        );
        assert!(
            decode_pattern(pattern).is_none(),
            "hashed pattern should not be decodable"
        );
        // Base64url should be at most 31 chars (23 bytes → ceil(23*4/3) = 31)
        let b64 = pattern_to_base64url(pattern);
        assert!(
            b64.len() <= 31,
            "hashed pattern base64url should be ≤31 chars, got {}",
            b64.len()
        );
    }

    #[test]
    fn hashed_pattern_deterministic() {
        // Two chains of the same length with different handles should produce
        // the same hash (structure is identical, handles don't matter).
        let ops_a = build_chain(30, 0xC0);
        let ops_b = build_chain(30, 0xD0);
        let ids_a = compute_logical_ids(ops_a);
        let ids_b = compute_logical_ids(ops_b);
        let pat_a = ids_a.values().next().unwrap();
        let pat_b = ids_b.values().next().unwrap();
        assert_eq!(
            pat_a, pat_b,
            "same structure with different handles should produce identical hashed pattern"
        );
    }
}
