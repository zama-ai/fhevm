use std::collections::{HashMap, HashSet};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, Transaction,
};
use opentelemetry::trace::{SpanId, TracerProvider};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider, SpanData};
use scheduler::dfg::pattern::{self, PatternInput};
use serial_test::serial;
use tracing_subscriber::layer::SubscriberExt;

use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    setup_event_harness, to_ty, wait_until_computed, zero_address,
};

// ── Subscriber / span helpers ───────────────────────────────────────────────

/// Try to install a test-only tracing subscriber with an in-memory OTel exporter.
///
/// Returns `Some(exporter)` if installation succeeded (first caller in the
/// process) or `None` if a subscriber was already registered by another test
/// or by the coprocessor itself.
fn try_install_test_subscriber() -> Option<InMemorySpanExporter> {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = provider.tracer("test");
    let layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::registry().with(layer);
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(()) => Some(exporter),
        Err(_) => None,
    }
}

fn get_string_attr(span: &SpanData, key: &str) -> Option<String> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| format!("{}", kv.value))
}

fn span_id_key(id: SpanId) -> u64 {
    u64::from_be_bytes(id.to_bytes())
}

/// Group `fhe_operation` spans by their parent `execute_transaction` span.
///
/// Returns a map from `transaction_pattern_id` to a list of tuples
/// `(execute_transaction span_id_key, Vec<operation_pattern_id>)`.
///
/// Multiple `execute_transaction` spans may share the same
/// `transaction_pattern_id` when a transaction is split into several
/// scheduler components.
fn group_ops_by_transaction(spans: &[SpanData]) -> HashMap<String, Vec<(u64, Vec<String>)>> {
    // execute_transaction span_id → transaction_pattern_id
    let exec_spans: HashMap<u64, String> = spans
        .iter()
        .filter(|s| s.name == "execute_transaction")
        .filter_map(|s| {
            let tp = get_string_attr(s, "transaction_pattern_id")?;
            Some((span_id_key(s.span_context.span_id()), tp))
        })
        .collect();

    // fhe_operation → (parent_span_id, operation_pattern_id)
    let fhe_ops: Vec<(u64, String)> = spans
        .iter()
        .filter(|s| s.name == "fhe_operation")
        .filter_map(|s| {
            let op = get_string_attr(s, "operation_pattern_id")?;
            Some((span_id_key(s.parent_span_id), op))
        })
        .collect();

    // Group: tx_pattern_id → [(exec_span_id, [op_pattern_ids])]
    let mut result: HashMap<String, Vec<(u64, Vec<String>)>> = HashMap::new();
    for (&exec_id, tx_pat) in &exec_spans {
        let child_ops: Vec<String> = fhe_ops
            .iter()
            .filter(|(parent, _)| *parent == exec_id)
            .map(|(_, op)| op.clone())
            .collect();
        if child_ops.is_empty() {
            continue;
        }
        result
            .entry(tx_pat.clone())
            .or_default()
            .push((exec_id, child_ops));
    }
    result
}

// ── ERC-20 transfer builders ────────────────────────────────────────────────

/// Insert a whitepaper encrypted transfer (5 operations) into the database.
///
/// ```text
///  balance_src ─┬─ FheGe(bal, amt) ───────┬─ FheIfThenElse(ge, add, dst) → new_to   [allowed]
///  amount ──────┤                          │
///  balance_dst ─┼─ FheAdd(dst, amt) ──────┘
///               │
///               ├─ FheSub(bal, amt) ──────┬─ FheIfThenElse(ge, sub, bal) → new_from  [allowed]
///               └─ FheGe(bal, amt) ───────┘
/// ```
async fn insert_whitepaper_transfer(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    balance_src: Handle,
    amount: Handle,
    balance_dst: Handle,
) -> Result<(), sqlx::Error> {
    let caller = zero_address();
    let h_ge = next_handle();
    let h_add = next_handle();
    let h_new_to = next_handle();
    let h_sub = next_handle();
    let h_new_from = next_handle();

    // [0] FheGe(balance_src, amount)
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheGe(TfheContract::FheGe {
            caller,
            lhs: balance_src,
            rhs: amount,
            scalarByte: scalar_flag(false),
            result: h_ge,
        }),
        false,
    )
    .await?;

    // [1] FheAdd(balance_dst, amount)
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: balance_dst,
            rhs: amount,
            scalarByte: scalar_flag(false),
            result: h_add,
        }),
        false,
    )
    .await?;

    // [2] FheIfThenElse(ge, add, balance_dst) → new_to [allowed]
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
            caller,
            control: h_ge,
            ifTrue: h_add,
            ifFalse: balance_dst,
            result: h_new_to,
        }),
        true,
    )
    .await?;
    allow_handle(listener_db, tx, &h_new_to).await?;

    // [3] FheSub(balance_src, amount)
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheSub(TfheContract::FheSub {
            caller,
            lhs: balance_src,
            rhs: amount,
            scalarByte: scalar_flag(false),
            result: h_sub,
        }),
        false,
    )
    .await?;

    // [4] FheIfThenElse(ge, sub, balance_src) → new_from [allowed]
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
            caller,
            control: h_ge,
            ifTrue: h_sub,
            ifFalse: balance_src,
            result: h_new_from,
        }),
        true,
    )
    .await?;
    allow_handle(listener_db, tx, &h_new_from).await?;

    Ok(())
}

/// Insert a no-cmux encrypted transfer (5 operations) into the database.
///
/// Uses FheCast + FheMul instead of FheIfThenElse for the conditional.
async fn insert_no_cmux_transfer(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    ct_type: i32,
    balance_src: Handle,
    amount: Handle,
    balance_dst: Handle,
) -> Result<(), sqlx::Error> {
    let caller = zero_address();
    let h_ge = next_handle();
    let h_cast = next_handle();
    let h_select = next_handle();
    let h_new_to = next_handle();
    let h_new_from = next_handle();

    // [0] FheGe(balance_src, amount)
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheGe(TfheContract::FheGe {
            caller,
            lhs: balance_src,
            rhs: amount,
            scalarByte: scalar_flag(false),
            result: h_ge,
        }),
        false,
    )
    .await?;

    // [1] FheCast(ge → ct_type)
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::Cast(TfheContract::Cast {
            caller,
            ct: h_ge,
            toType: to_ty(ct_type),
            result: h_cast,
        }),
        false,
    )
    .await?;

    // [2] FheMul(amount, cast)
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheMul(TfheContract::FheMul {
            caller,
            lhs: amount,
            rhs: h_cast,
            scalarByte: scalar_flag(false),
            result: h_select,
        }),
        false,
    )
    .await?;

    // [3] FheAdd(balance_dst, select) → new_to [allowed]
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: balance_dst,
            rhs: h_select,
            scalarByte: scalar_flag(false),
            result: h_new_to,
        }),
        true,
    )
    .await?;
    allow_handle(listener_db, tx, &h_new_to).await?;

    // [4] FheSub(balance_src, select) → new_from [allowed]
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheSub(TfheContract::FheSub {
            caller,
            lhs: balance_src,
            rhs: h_select,
            scalarByte: scalar_flag(false),
            result: h_new_from,
        }),
        true,
    )
    .await?;
    allow_handle(listener_db, tx, &h_new_from).await?;

    Ok(())
}

// ── Test ────────────────────────────────────────────────────────────────────

/// End-to-end test for `transaction_pattern_id` and `operation_pattern_id`
/// using realistic encrypted-transfer transaction shapes.
///
/// ## Transactions
///
/// | Label       | Description                              | Ops |
/// |-------------|------------------------------------------|-----|
/// | `Tx_single` | 1 whitepaper transfer  (bal=100,amt=10)  |  5  |
/// | `Tx_triple` | 3 whitepaper transfers in **one** tx     | 15  |
/// | `Tx_nocmux` | 1 no-cmux transfer     (bal=100,amt=10)  |  5  |
///
/// ## Key assertions
///
/// 1. **Single transfer**: `operation_pattern_id == transaction_pattern_id`
///    for every `fhe_operation` span in `Tx_single`, because the single
///    logical group covers the entire transaction graph.
///
/// 2. **Triple transfer**: every `operation_pattern_id` across all 15 ops
///    equals `Tx_single`'s `transaction_pattern_id`, because each of the
///    3 independent transfers has the same 5-op subgraph as the single
///    transfer.  Meanwhile `Tx_triple`'s own `transaction_pattern_id`
///    differs (it hashes a larger 15-op graph).
///
/// 3. **Different implementation**: `Tx_nocmux` has a different
///    `transaction_pattern_id` than `Tx_single` (different opcodes/edges).
#[tokio::test]
#[serial(db)]
async fn test_erc20_transaction_pattern_ids() -> Result<(), Box<dyn std::error::Error>> {
    let exporter = try_install_test_subscriber();

    let harness = setup_event_harness().await?;
    let listener_db = &harness.listener_db;

    // Clear startup spans.
    if let Some(ref exp) = exporter {
        exp.reset();
    }

    let ct_type = 4; // FheUint32

    // ── Encrypt inputs ──────────────────────────────────────────────────
    //
    // Shared inputs for Tx_single, Tx_triple A, and Tx_nocmux:
    //   balance=100, amount=10, dest=20
    // Tx_triple B: balance=300, amount=25, dest=40
    // Tx_triple C: balance=500, amount=50, dest=60

    let setup_tx_id = next_handle();
    let h_bal1 = next_handle();
    let h_amt1 = next_handle();
    let h_dst1 = next_handle();
    let h_bal_b = next_handle();
    let h_amt_b = next_handle();
    let h_dst_b = next_handle();
    let h_bal_c = next_handle();
    let h_amt_c = next_handle();
    let h_dst_c = next_handle();

    {
        let mut tx = listener_db.new_transaction().await?;
        for (value, handle) in [
            (100, h_bal1),
            (10, h_amt1),
            (20, h_dst1),
            (300, h_bal_b),
            (25, h_amt_b),
            (40, h_dst_b),
            (500, h_bal_c),
            (50, h_amt_c),
            (60, h_dst_c),
        ] {
            insert_trivial_encrypt(
                listener_db,
                &mut tx,
                setup_tx_id,
                value,
                ct_type,
                handle,
                true,
            )
            .await?;
            allow_handle(listener_db, &mut tx, &handle).await?;
        }
        tx.commit().await?;
    }

    // ── Tx_single: 1 whitepaper transfer ────────────────────────────────

    let tx_single_id = next_handle();
    {
        let mut tx = listener_db.new_transaction().await?;
        insert_whitepaper_transfer(listener_db, &mut tx, tx_single_id, h_bal1, h_amt1, h_dst1)
            .await?;
        tx.commit().await?;
    }

    // ── Tx_triple: 3 independent whitepaper transfers, same tx_id ───────

    let tx_triple_id = next_handle();
    {
        let mut tx = listener_db.new_transaction().await?;
        insert_whitepaper_transfer(listener_db, &mut tx, tx_triple_id, h_bal1, h_amt1, h_dst1)
            .await?;
        insert_whitepaper_transfer(
            listener_db,
            &mut tx,
            tx_triple_id,
            h_bal_b,
            h_amt_b,
            h_dst_b,
        )
        .await?;
        insert_whitepaper_transfer(
            listener_db,
            &mut tx,
            tx_triple_id,
            h_bal_c,
            h_amt_c,
            h_dst_c,
        )
        .await?;
        tx.commit().await?;
    }

    // ── Tx_nocmux: 1 no-cmux transfer ──────────────────────────────────

    let tx_nocmux_id = next_handle();
    {
        let mut tx = listener_db.new_transaction().await?;
        insert_no_cmux_transfer(
            listener_db,
            &mut tx,
            tx_nocmux_id,
            ct_type,
            h_bal1,
            h_amt1,
            h_dst1,
        )
        .await?;
        tx.commit().await?;
    }

    // ── Wait for all computations ──────────────────────────────────────

    wait_until_computed(&harness.app).await?;

    println!("All computations completed for all transfers");

    // ── Span assertions ─────────────────────────────────────────────────

    if let Some(ref exp) = exporter {
        let spans = exp.get_finished_spans().expect("failed to read spans");
        let groups = group_ops_by_transaction(&spans);

        // We expect exactly 3 distinct transaction_pattern_ids.
        //
        // The scheduler may split each transaction into multiple
        // `execute_transaction` scheduling components, so we identify
        // transactions by their **total op count** across all components:
        //   Tx_triple  – 15 total ops (3 independent 5-op transfers)
        //   Tx_single  – 5 total ops, whitepaper transfer
        //   Tx_nocmux  – 5 total ops, no-cmux transfer
        assert!(
            groups.len() >= 3,
            "expected >= 3 distinct transaction_pattern_ids, got {}",
            groups.len()
        );

        // Collect (tx_pattern, total_op_count, all_op_pids) for each group.
        let mut by_op_count: Vec<(String, Vec<String>)> = groups
            .iter()
            .map(|(tx_pat, components)| {
                let all_op_pids: Vec<String> =
                    components.iter().flat_map(|(_, ops)| ops.clone()).collect();
                (tx_pat.clone(), all_op_pids)
            })
            .collect();

        // Find Tx_triple: the only one with 15 ops.
        let triple_idx = by_op_count
            .iter()
            .position(|(_, ops)| ops.len() == 15)
            .expect("should find a transaction_pattern with 15 ops (triple transfer)");
        let (p_triple_tx, p_triple_ops) = by_op_count.remove(triple_idx);

        // The remaining two both have 5 ops.  Distinguish whitepaper from
        // no-cmux: Tx_triple's operation_pattern_ids should all equal
        // Tx_single's transaction_pattern_id (the whitepaper pattern).
        let whitepaper_op_pid = p_triple_ops.first().unwrap().clone();

        let single_idx = by_op_count
            .iter()
            .position(|(tx_pat, _)| *tx_pat == whitepaper_op_pid)
            .expect("should find single-transfer tx whose tx_pattern matches triple's op_pattern");
        let (p_single_tx, p_single_ops) = by_op_count.remove(single_idx);

        // The remaining 5-op pattern is the no-cmux transfer.
        let nocmux_idx = by_op_count
            .iter()
            .position(|(_, ops)| ops.len() == 5)
            .expect("should find no-cmux transfer transaction");
        let (p_nocmux_tx, p_nocmux_ops) = by_op_count.remove(nocmux_idx);

        let empty_pattern = "";

        // ── Assertion 1: single transfer ────────────────────────────────
        //
        // For a single-transfer transaction, the one logical group IS the
        // entire graph.  Therefore every fhe_operation's
        // operation_pattern_id must equal the transaction_pattern_id.
        assert_ne!(p_single_tx, empty_pattern);
        assert_eq!(p_single_ops.len(), 5, "single transfer should have 5 ops");
        for op_pid in &p_single_ops {
            assert_eq!(
                op_pid, &p_single_tx,
                "single transfer: operation_pattern_id should equal transaction_pattern_id"
            );
        }
        println!("assertion 1 passed: single transfer op_pattern == tx_pattern");
        println!("  pattern: {p_single_tx}");

        // ── Assertion 2: triple transfer ────────────────────────────────
        //
        // Each of the 3 independent transfers in Tx_triple has the same
        // 5-op subgraph as Tx_single.  So:
        //   a) all 15 operation_pattern_ids are identical
        //   b) they equal Tx_single's transaction_pattern_id
        //   c) Tx_triple's own transaction_pattern_id differs (15-op graph)
        assert_eq!(p_triple_ops.len(), 15, "triple transfer should have 15 ops");
        let unique_triple_op_pids: HashSet<&String> = p_triple_ops.iter().collect();
        assert_eq!(
            unique_triple_op_pids.len(),
            1,
            "all 15 ops in triple transfer should share one operation_pattern_id, \
             got {unique_triple_op_pids:?}"
        );
        let triple_op_pid = p_triple_ops.first().unwrap();
        assert_eq!(
            triple_op_pid, &p_single_tx,
            "triple transfer operation_pattern_id should equal \
             single transfer's transaction_pattern_id"
        );
        assert_ne!(
            p_triple_tx, p_single_tx,
            "triple transfer's transaction_pattern_id must differ from \
             single transfer (15-op graph vs 5-op graph)"
        );
        println!("assertion 2 passed: triple transfer op_patterns == single tx_pattern");
        println!("  triple tx_pattern:  {p_triple_tx} (15-op graph)");
        println!("  triple op_pattern:  {triple_op_pid} == single tx_pattern");

        // ── Assertion 3: different implementation ───────────────────────
        assert_ne!(
            p_nocmux_tx, p_single_tx,
            "no-cmux transfer must have different transaction_pattern_id than whitepaper"
        );
        assert_ne!(
            p_nocmux_tx, p_triple_tx,
            "no-cmux transfer must have different transaction_pattern_id than triple"
        );
        // The no-cmux single transfer should also satisfy op_pattern == tx_pattern.
        assert_eq!(p_nocmux_ops.len(), 5, "no-cmux should have 5 ops");
        for op_pid in &p_nocmux_ops {
            assert_eq!(
                op_pid, &p_nocmux_tx,
                "no-cmux: operation_pattern_id should equal transaction_pattern_id"
            );
        }
        println!("assertion 3 passed: no-cmux has different tx_pattern");
        println!("  no-cmux pattern: {p_nocmux_tx}");

        // ── Assertion 4: encodings are decodable and structurally correct ─
        //
        // Pattern IDs in span attributes are base64url-encoded binary
        // encodings. Decode them and verify the actual DFG structure.

        let decode_b64 = |b64: &str| -> pattern::PatternDescription {
            let bytes = URL_SAFE_NO_PAD
                .decode(b64.as_bytes())
                .unwrap_or_else(|e| panic!("invalid base64url pattern '{b64}': {e}"));
            if pattern::is_hashed_pattern(&bytes) {
                panic!("expected decodable pattern but got hashed pattern '{b64}'");
            }
            pattern::decode_pattern(&bytes)
                .unwrap_or_else(|| panic!("malformed pattern encoding '{b64}'"))
        };

        // -- Whitepaper single-transfer encoding --
        //
        // Expected DFG (5 nodes in topo order):
        //   [0] FheGe(ext, ext)
        //   [1] FheAdd(ext, ext)
        //   [2] FheIfThenElse(ref0, ref1, ext)  <- is_allowed
        //   [3] FheSub(ext, ext)
        //   [4] FheIfThenElse(ref0, ref3, ext)  <- is_allowed
        let wp = decode_b64(&p_single_tx);
        assert_eq!(wp.nodes.len(), 5, "whitepaper pattern should have 5 nodes");

        let wp_opcodes: Vec<&str> = wp.nodes.iter().map(|n| n.opcode_name).collect();
        assert_eq!(
            wp_opcodes,
            &[
                "FHE_GE",
                "FHE_ADD",
                "FHE_IF_THEN_ELSE",
                "FHE_SUB",
                "FHE_IF_THEN_ELSE"
            ],
            "whitepaper opcode sequence"
        );

        // FheGe and FheAdd: 2 external inputs each
        assert!(wp.nodes[0]
            .inputs
            .iter()
            .all(|i| matches!(i, PatternInput::External)));
        assert!(wp.nodes[1]
            .inputs
            .iter()
            .all(|i| matches!(i, PatternInput::External)));

        // First FheIfThenElse: refs GE(0) and ADD(1), plus external
        assert!(matches!(wp.nodes[2].inputs[0], PatternInput::Internal(0)));
        assert!(matches!(wp.nodes[2].inputs[1], PatternInput::Internal(1)));
        assert!(matches!(wp.nodes[2].inputs[2], PatternInput::External));
        assert!(wp.nodes[2].is_allowed);

        // FheSub: 2 external inputs
        assert!(wp.nodes[3]
            .inputs
            .iter()
            .all(|i| matches!(i, PatternInput::External)));

        // Second FheIfThenElse: refs GE(0) and SUB(3), plus external
        assert!(matches!(wp.nodes[4].inputs[0], PatternInput::Internal(0)));
        assert!(matches!(wp.nodes[4].inputs[1], PatternInput::Internal(3)));
        assert!(matches!(wp.nodes[4].inputs[2], PatternInput::External));
        assert!(wp.nodes[4].is_allowed);

        println!("assertion 4a passed: whitepaper encoding = {wp}");

        // -- No-cmux single-transfer encoding --
        //
        // Expected DFG (5 nodes in topo order):
        //   [0] FheGe(ext, ext)
        //   [1] FheCast(ref0, ext)
        //   [2] FheMul(ext, ref1)
        //   [3] FheAdd(ext, ref2)  <- is_allowed
        //   [4] FheSub(ext, ref2)  <- is_allowed
        let nc = decode_b64(&p_nocmux_tx);
        assert_eq!(nc.nodes.len(), 5, "no-cmux pattern should have 5 nodes");

        let nc_opcodes: Vec<&str> = nc.nodes.iter().map(|n| n.opcode_name).collect();
        assert_eq!(
            nc_opcodes,
            &["FHE_GE", "FHE_CAST", "FHE_MUL", "FHE_ADD", "FHE_SUB"],
            "no-cmux opcode sequence"
        );

        // FheGe: 2 external inputs
        assert!(nc.nodes[0]
            .inputs
            .iter()
            .all(|i| matches!(i, PatternInput::External)));

        // FheCast: refs GE(0), plus scalar (external)
        assert!(matches!(nc.nodes[1].inputs[0], PatternInput::Internal(0)));
        assert!(matches!(nc.nodes[1].inputs[1], PatternInput::External));

        // FheMul: external + ref Cast(1)
        assert!(matches!(nc.nodes[2].inputs[0], PatternInput::External));
        assert!(matches!(nc.nodes[2].inputs[1], PatternInput::Internal(1)));

        // FheAdd: external + ref Mul(2), is_allowed
        assert!(matches!(nc.nodes[3].inputs[0], PatternInput::External));
        assert!(matches!(nc.nodes[3].inputs[1], PatternInput::Internal(2)));
        assert!(nc.nodes[3].is_allowed);

        // FheSub: external + ref Mul(2), is_allowed
        assert!(matches!(nc.nodes[4].inputs[0], PatternInput::External));
        assert!(matches!(nc.nodes[4].inputs[1], PatternInput::Internal(2)));
        assert!(nc.nodes[4].is_allowed);

        println!("assertion 4b passed: no-cmux encoding = {nc}");

        // -- Triple-transfer transaction-level encoding --
        //
        // The transaction_pattern_id encodes the full 15-op graph.
        let triple = decode_b64(&p_triple_tx);
        assert_eq!(
            triple.nodes.len(),
            15,
            "triple transfer tx pattern should have 15 nodes"
        );
        // All 15 nodes should use the same opcodes as 3 whitepaper transfers
        let triple_opcodes: Vec<&str> = triple.nodes.iter().map(|n| n.opcode_name).collect();
        for chunk in triple_opcodes.chunks(5) {
            assert_eq!(
                chunk,
                &[
                    "FHE_GE",
                    "FHE_ADD",
                    "FHE_IF_THEN_ELSE",
                    "FHE_SUB",
                    "FHE_IF_THEN_ELSE"
                ],
                "each 5-op chunk in triple tx should match whitepaper opcode sequence"
            );
        }

        println!("assertion 4c passed: triple tx encoding = {triple}");
    } else {
        println!("skipping span assertions: global subscriber already set by another test");
    }

    Ok(())
}
