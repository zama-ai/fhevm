use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use opentelemetry::trace::{SpanId, TracerProvider};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider, SpanData};
use scheduler::dfg::pattern::{self, PatternInput};
use serial_test::serial;
use tonic::metadata::MetadataValue;
use tracing_subscriber::layer::SubscriberExt;

use crate::server::common::FheOperation;
use crate::server::tfhe_worker::async_computation_input::Input;
use crate::server::tfhe_worker::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::tfhe_worker::{
    AsyncComputation, AsyncComputationInput, AsyncComputeRequest, TrivialEncryptBatch,
    TrivialEncryptRequestSingle,
};

use super::utils::{
    decrypt_ciphertexts, default_api_key, random_handle, setup_test_app,
    wait_until_all_allowed_handles_computed,
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

// ── Handle counter ──────────────────────────────────────────────────────────

struct HandleCounter(u64);

impl HandleCounter {
    fn new() -> Self {
        Self(random_handle())
    }
    fn next(&mut self) -> Vec<u8> {
        let h = self.0.to_be_bytes().to_vec();
        self.0 += 1;
        h
    }
}

// ── ERC-20 transfer builders ────────────────────────────────────────────────

/// Whitepaper encrypted transfer (5 operations, 2 allowed outputs).
///
/// ```text
///  balance_src ─┬─ FheGe(bal, amt) ───────┬─ FheIfThenElse(ge, add, dst) → new_to   [allowed]
///  amount ──────┤                          │
///  balance_dst ─┼─ FheAdd(dst, amt) ──────┘
///               │
///               ├─ FheSub(bal, amt) ──────┬─ FheIfThenElse(ge, sub, bal) → new_from  [allowed]
///               └─ FheGe(bal, amt) ───────┘
/// ```
struct WhitepaperTransfer {
    new_to_handle: Vec<u8>,
    new_from_handle: Vec<u8>,
    ops: Vec<AsyncComputation>,
}

fn build_whitepaper_transfer(
    handles: &mut HandleCounter,
    tx_id: &[u8],
    balance_src: &AsyncComputationInput,
    amount: &AsyncComputationInput,
    balance_dst: &AsyncComputationInput,
) -> WhitepaperTransfer {
    let h_ge = handles.next();
    let h_add_target = handles.next();
    let h_new_to = handles.next();
    let h_sub_target = handles.next();
    let h_new_from = handles.next();

    let ops = vec![
        // [0] FheGe(balance_src, amount) → has_enough_funds
        AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_ge.clone(),
            inputs: vec![balance_src.clone(), amount.clone()],
            is_allowed: false,
        },
        // [1] FheAdd(balance_dst, amount) → new_to_target
        AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_add_target.clone(),
            inputs: vec![balance_dst.clone(), amount.clone()],
            is_allowed: false,
        },
        // [2] FheIfThenElse(has_enough, new_to_target, balance_dst) → new_to
        AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_new_to.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_ge.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_add_target.clone())),
                },
                balance_dst.clone(),
            ],
            is_allowed: true,
        },
        // [3] FheSub(balance_src, amount) → new_from_target
        AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_sub_target.clone(),
            inputs: vec![balance_src.clone(), amount.clone()],
            is_allowed: false,
        },
        // [4] FheIfThenElse(has_enough, new_from_target, balance_src) → new_from
        AsyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_new_from.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_ge.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_sub_target.clone())),
                },
                balance_src.clone(),
            ],
            is_allowed: true,
        },
    ];

    WhitepaperTransfer {
        new_to_handle: h_new_to,
        new_from_handle: h_new_from,
        ops,
    }
}

/// No-cmux encrypted transfer variant (5 operations, 2 allowed outputs).
///
/// Uses FheCast + FheMul instead of FheIfThenElse for the conditional.
struct NoCmuxTransfer {
    new_to_handle: Vec<u8>,
    new_from_handle: Vec<u8>,
    ops: Vec<AsyncComputation>,
}

fn build_no_cmux_transfer(
    handles: &mut HandleCounter,
    tx_id: &[u8],
    ct_type: i32,
    balance_src: &AsyncComputationInput,
    amount: &AsyncComputationInput,
    balance_dst: &AsyncComputationInput,
) -> NoCmuxTransfer {
    let h_ge = handles.next();
    let h_cast = handles.next();
    let h_select = handles.next();
    let h_new_to = handles.next();
    let h_new_from = handles.next();

    let ops = vec![
        // [0] FheGe(balance_src, amount) → has_enough_funds (bool)
        AsyncComputation {
            operation: FheOperation::FheGe.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_ge.clone(),
            inputs: vec![balance_src.clone(), amount.clone()],
            is_allowed: false,
        },
        // [1] FheCast(has_enough_funds → ct_type)
        AsyncComputation {
            operation: FheOperation::FheCast.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_cast.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_ge.clone())),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![ct_type as u8])),
                },
            ],
            is_allowed: false,
        },
        // [2] FheMul(amount, cast_funds) → select_amount
        AsyncComputation {
            operation: FheOperation::FheMul.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_select.clone(),
            inputs: vec![
                amount.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_cast.clone())),
                },
            ],
            is_allowed: false,
        },
        // [3] FheAdd(balance_dst, select_amount) → new_to
        AsyncComputation {
            operation: FheOperation::FheAdd.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_new_to.clone(),
            inputs: vec![
                balance_dst.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_select.clone())),
                },
            ],
            is_allowed: true,
        },
        // [4] FheSub(balance_src, select_amount) → new_from
        AsyncComputation {
            operation: FheOperation::FheSub.into(),
            transaction_id: tx_id.to_vec(),
            output_handle: h_new_from.clone(),
            inputs: vec![
                balance_src.clone(),
                AsyncComputationInput {
                    input: Some(Input::InputHandle(h_select.clone())),
                },
            ],
            is_allowed: true,
        },
    ];

    NoCmuxTransfer {
        new_to_handle: h_new_to,
        new_from_handle: h_new_from,
        ops,
    }
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
#[serial]
async fn test_erc20_transaction_pattern_ids() -> Result<(), Box<dyn std::error::Error>> {
    let exporter = try_install_test_subscriber();

    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    let api_key_header = format!("bearer {}", default_api_key());

    // Clear startup spans.
    if let Some(ref exp) = exporter {
        exp.reset();
    }

    let ct_type = 4; // FheUint32
    let mut handles = HandleCounter::new();

    // ── Encrypt inputs ──────────────────────────────────────────────────
    //
    // Tx_single & Tx_nocmux: balance=100, amount=10, dest=20
    // Tx_triple transfer A:  balance=100, amount=10,  dest=20
    // Tx_triple transfer B:  balance=300, amount=25,  dest=40
    // Tx_triple transfer C:  balance=500, amount=50,  dest=60

    let h_bal1 = handles.next();
    let h_amt1 = handles.next();
    let h_dst1 = handles.next();
    // Separate handles for Tx_triple transfer B
    let h_bal_b = handles.next();
    let h_amt_b = handles.next();
    let h_dst_b = handles.next();
    // Separate handles for Tx_triple transfer C
    let h_bal_c = handles.next();
    let h_amt_c = handles.next();
    let h_dst_c = handles.next();

    {
        let mut req = tonic::Request::new(TrivialEncryptBatch {
            values: vec![
                TrivialEncryptRequestSingle {
                    handle: h_bal1.clone(),
                    be_value: vec![100],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_amt1.clone(),
                    be_value: vec![10],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_dst1.clone(),
                    be_value: vec![20],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_bal_b.clone(),
                    be_value: vec![0x01, 0x2C], // 300
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_amt_b.clone(),
                    be_value: vec![25],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_dst_b.clone(),
                    be_value: vec![40],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_bal_c.clone(),
                    be_value: vec![0x01, 0xF4], // 500
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_amt_c.clone(),
                    be_value: vec![50],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
                    handle: h_dst_c.clone(),
                    be_value: vec![60],
                    output_type: ct_type,
                },
            ],
        });
        req.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        client.trivial_encrypt_ciphertexts(req).await?;
    }

    let input = |h: &[u8]| AsyncComputationInput {
        input: Some(Input::InputHandle(h.to_vec())),
    };

    // ── Tx_single: 1 whitepaper transfer ────────────────────────────────

    let tx_single_id = handles.next();
    let tx_single = build_whitepaper_transfer(
        &mut handles,
        &tx_single_id,
        &input(&h_bal1),
        &input(&h_amt1),
        &input(&h_dst1),
    );

    // ── Tx_triple: 3 independent whitepaper transfers, same tx_id ───────

    let tx_triple_id = handles.next();
    let tx_triple_a = build_whitepaper_transfer(
        &mut handles,
        &tx_triple_id,
        &input(&h_bal1),
        &input(&h_amt1),
        &input(&h_dst1),
    );
    let tx_triple_b = build_whitepaper_transfer(
        &mut handles,
        &tx_triple_id,
        &input(&h_bal_b),
        &input(&h_amt_b),
        &input(&h_dst_b),
    );
    let tx_triple_c = build_whitepaper_transfer(
        &mut handles,
        &tx_triple_id,
        &input(&h_bal_c),
        &input(&h_amt_c),
        &input(&h_dst_c),
    );

    // ── Tx_nocmux: 1 no-cmux transfer ──────────────────────────────────

    let tx_nocmux_id = handles.next();
    let tx_nocmux = build_no_cmux_transfer(
        &mut handles,
        &tx_nocmux_id,
        ct_type,
        &input(&h_bal1),
        &input(&h_amt1),
        &input(&h_dst1),
    );

    // ── Submit all computations ─────────────────────────────────────────

    let mut all_ops = Vec::with_capacity(25);
    all_ops.extend(tx_single.ops);
    all_ops.extend(tx_triple_a.ops);
    all_ops.extend(tx_triple_b.ops);
    all_ops.extend(tx_triple_c.ops);
    all_ops.extend(tx_nocmux.ops);

    {
        let mut req = tonic::Request::new(AsyncComputeRequest {
            computations: all_ops,
        });
        req.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        client.async_compute(req).await?;
    }

    // ── Wait & decrypt ──────────────────────────────────────────────────

    wait_until_all_allowed_handles_computed(&app).await?;

    let results = decrypt_ciphertexts(
        &pool,
        vec![
            tx_single.new_to_handle.clone(),
            tx_single.new_from_handle.clone(),
            tx_triple_a.new_to_handle.clone(),
            tx_triple_a.new_from_handle.clone(),
            tx_triple_b.new_to_handle.clone(),
            tx_triple_b.new_from_handle.clone(),
            tx_triple_c.new_to_handle.clone(),
            tx_triple_c.new_from_handle.clone(),
            tx_nocmux.new_to_handle.clone(),
            tx_nocmux.new_from_handle.clone(),
        ],
    )
    .await?;

    assert_eq!(results.len(), 10);

    // Tx_single: whitepaper(100, 10, 20) → new_to=30, new_from=90
    assert_eq!(results[0].value, "30", "single new_to");
    assert_eq!(results[1].value, "90", "single new_from");

    // Tx_triple A: whitepaper(100, 10, 20) → 30, 90
    assert_eq!(results[2].value, "30", "triple_a new_to");
    assert_eq!(results[3].value, "90", "triple_a new_from");

    // Tx_triple B: whitepaper(300, 25, 40) → 65, 275
    assert_eq!(results[4].value, "65", "triple_b new_to");
    assert_eq!(results[5].value, "275", "triple_b new_from");

    // Tx_triple C: whitepaper(500, 50, 60) → 110, 450
    assert_eq!(results[6].value, "110", "triple_c new_to");
    assert_eq!(results[7].value, "450", "triple_c new_from");

    // Tx_nocmux: no-cmux(100, 10, 20) → 30, 90
    assert_eq!(results[8].value, "30", "nocmux new_to");
    assert_eq!(results[9].value, "90", "nocmux new_from");

    println!("FHE correctness verified for all transfers");

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
        //   [2] FheIfThenElse(ref0, ref1, ext)  ← is_allowed
        //   [3] FheSub(ext, ext)
        //   [4] FheIfThenElse(ref0, ref3, ext)  ← is_allowed
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
        //   [3] FheAdd(ext, ref2)  ← is_allowed
        //   [4] FheSub(ext, ref2)  ← is_allowed
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
